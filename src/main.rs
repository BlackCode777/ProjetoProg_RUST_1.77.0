use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;

mod controller;
mod enums;
mod errors;
mod model;
mod repository;
mod service;

use service::book_service::BookService;

#[derive(Clone)]
pub struct AppState {
    pub book_service: BookService,
}

/* O que ele faz na prática?
1) - Cria um runtime Tokio (um “motor” de execução assíncrona com event loop + thread pool).
2) - Permite que seu main seja async fn main().
3) - Executa o corpo do main dentro desse runtime. */
#[tokio::main(flavor = "multi_thread")]
async fn main() {

    /*BlackCode77, esse trecho faz duas inicializações bem comuns em backend Rust: carregar variáveis do `.env` e ligar o sistema de logs.

        ## 1) `dotenv().ok();`
        * `dotenv()` tenta ler um arquivo **`.env`** na raiz do projeto (ou no diretório atual onde o binário está sendo executado).
        * Ele pega linhas como:

          ```env
          DATABASE_URL=postgres://postgres:admin@localhost:5432/booksdb
          ```

          e coloca isso como **variáveis de ambiente** do processo.
        * O `.ok()` é só pra **ignorar erro** caso não exista `.env`.
          * Sem `.ok()`, você teria que tratar o erro (por exemplo, em produção pode não ter `.env`, e sim variáveis do sistema).

        Resultado prático: depois disso, `env::var("DATABASE_URL")` passa a funcionar lendo do `.env`.

        ## 2) `tracing_subscriber::fmt::init();`
        * `tracing` é a biblioteca de logs moderna do Rust.
        * `tracing_subscriber::fmt::init()` liga um “subscriber” que imprime logs no console (stdout) em formato legível.
        * Isso permite que logs de bibliotecas (ex.: Axum/Hyper/SQLx) apareçam no terminal.

        Exemplo do que começa a aparecer depois disso:
        * logs de requisições (se você usar middleware de trace)
        * logs do SQLx (ex.: “slow acquire threshold”)
        * seus logs se você usar `tracing::info!`, `tracing::warn!`, etc.

        Resumo: a primeira linha carrega config do `.env`; a segunda liga o motor de logs do backend. */
    dotenv().ok();
    tracing_subscriber::fmt::init();

    /* BlackCode77, esse trecho faz a **leitura da string de conexão** e cria um **pool de conexões** com o Postgres.

        1. `let db_url = env::var("DATABASE_URL").expect("DATABASE_URL não definido no .env");`
           * `env::var("DATABASE_URL")` tenta ler a variável de ambiente `DATABASE_URL`.
           * Como você chama `dotenv().ok()` antes, ele carrega o `.env` e coloca essas variáveis no ambiente do processo.
           * `expect(...)` significa: se não existir `DATABASE_URL`, o programa **para** e mostra essa mensagem.

        Exemplo de `.env`:
        ```env   DATABASE_URL=postgres://postgres:admin@localhost:5432/booksdb ```

        2. `let pool = PgPoolOptions::new()`
           * Começa a configurar opções do pool de conexões do SQLx.

        3. `.max_connections(10)`
           * Define o máximo de conexões simultâneas que esse pool pode manter abertas com o banco.
           * Isso é importante porque abrir conexão é “caro”; pool reaproveita conexões.

        4. `.connect(&db_url).await`
           * Conecta no Postgres usando a URL e cria o pool pronto.
           * É `await` porque envolve I/O (rede).

        5. `.expect("Falha ao conectar no Postgres");`
           * Se não conseguir conectar (Postgres fora do ar, porta errada, senha errada, DB não existe), ele **panic** com essa mensagem.

        Resumo: você carrega a URL do banco e cria um `PgPool` com até 10 conexões reutilizáveis, que será usado por
        repositórios/serviços pra executar queries sem ficar abrindo conexão toda hora. */
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL não definido no .env");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .expect("Falha ao conectar no Postgres");

    /*BlackCode77, esse trecho faz duas coisas: cria o “estado compartilhado” da aplicação e registra
        esse estado no Router do Axum para que os handlers consigam acessar o serviço.

        1. `let state = AppState { book_service: BookService::new(pool) };`
           * `AppState` é um struct que você definiu para carregar dependências da API (tipo um “container” simples).
           * Aqui você está criando um `BookService` com o `pool` do Postgres (`PgPool`).
           * Resultado: `state` passa a ter o serviço pronto para ser usado em qualquer endpoint.

        2. `let app = axum::Router::new()`
           * Cria um router vazio (sem rotas ainda).

        3. `.merge(controller::routes())`
           * “Mescla” as rotas definidas no seu módulo `controller` (por exemplo `/health`, `/books`, `/books/{id}`)
                dentro do router principal.
           * Isso te permite organizar rotas em arquivos separados e depois juntar tudo no `main`.

        4. `.with_state(state);`
           * Registra esse `state` dentro do Router.
           * A partir daí, qualquer handler que declarar `State(state): State<AppState>` consegue acessar
           `state.book_service` sem precisar criar service toda hora.

        Em resumo: você cria `BookService` **uma vez** (com o pool), guarda no `AppState`, e o Axum injeta esse
        `AppState` nos handlers via `State(...)`.  */
    let state = AppState { book_service: BookService::new(pool) };
    let app = axum::Router::new()
        .merge(controller::routes())
        .with_state(state);

    /* 1) - let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
                Monta um endereço de rede: IP 127.0.0.1 (loopback = só sua máquina) e porta 8080.
                Resultado: o servidor vai responder em http://127.0.0.1:8080.

        2) - let listener = TcpListener::bind(addr).await.unwrap();
                Cria um “ouvinte” TCP (socket) preso nesse endereço.
                Ele “abre” a porta 8080 e fica aguardando conexões.
                await porque abrir/bind pode envolver operação async.
                unwrap() porque se falhar (porta em uso, permissão, etc.) o programa panic.

        3) - axum::serve(listener, app).await.unwrap();
                Entrega esse listener pro Axum e manda ele servir o app (seu Router com rotas/handlers).
                A partir daí, cada request HTTP que chegar na porta 8080 é roteado pras funções (handlers) que você registrou.
                Esse .await normalmente fica rodando indefinidamente enquanto a API estiver no ar.
                Outro unwrap() para “morrer” se o servidor falhar por algum motivo. */
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

}
