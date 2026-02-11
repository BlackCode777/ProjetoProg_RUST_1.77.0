// ==============================
// src/main.rs (versão didática)
// ==============================

// Importa a função `dotenv()` (crate dotenvy) para carregar variáveis do arquivo `.env`
use dotenvy::dotenv;

// Importa o "builder" de pool de conexões para Postgres (SQLx)
use sqlx::postgres::PgPoolOptions;

// Importa `SocketAddr` (endereço IP + porta) do std, usado para bind do servidor
use std::net::SocketAddr;

// Importa `TcpListener` do Tokio (listener TCP assíncrono)
use tokio::net::TcpListener;

// Middleware do Tower para logar requisições/respostas HTTP automaticamente
use tower_http::trace::TraceLayer;

// Declara os módulos do projeto (cada `mod xyz;` corresponde a `src/xyz.rs` ou `src/xyz/mod.rs`)
mod controller;
mod errors;
mod model;
mod repository;
mod service;
mod settings;

// Importa o BookService de dentro do módulo service
use service::book_service::BookService;

// Importa Settings, que lê configurações do ambiente (.env / variáveis do sistema)
use settings::Settings;

// `AppState` é o "estado global" que o Axum injeta nos handlers via `State(...)`.
// Aqui você guarda suas dependências (BookService, e no futuro cache, config, etc.)
#[derive(Clone)]
pub struct AppState {
    pub book_service: BookService,
}

/*
    O que `#[tokio::main(flavor = "multi_thread")]` faz na prática?

    1) Cria e inicializa um runtime Tokio (motor async).
    2) Permite que `main` seja `async fn main()`.
    3) Executa o corpo do `main` dentro desse runtime usando múltiplas threads.

    Por que "multi_thread"?
    - Uma API atende várias conexões ao mesmo tempo.
    - O runtime multi-thread distribui tasks em um pool de workers.
*/
#[tokio::main(flavor = "multi_thread")]
async fn main() {

    /*
        1) dotenv().ok();

        - Procura um arquivo `.env` e carrega as variáveis para o ambiente do processo.
        - Ex: DATABASE_URL=postgres://...
        - `.ok()` ignora erro se o arquivo não existir (útil em produção, onde vem do ambiente).
    */
    dotenv().ok();

    /*
        2) tracing_subscriber::fmt::init();

        - Inicializa o sistema de logs (`tracing`).
        - Faz logs de libs como axum/sqlx aparecerem no terminal.
        - Em conjunto com TraceLayer, você vê logs de requisição HTTP.
    */
    tracing_subscriber::fmt::init();

    /*
        3) Settings::from_env()

        - Lê configurações (HOST, PORT, DATABASE_URL, DB_MAX_CONNS etc.)
        - Centraliza config em um lugar só (melhor que hardcode).
        - Evita espalhar env::var() por todo projeto.
    */
    let cfg = Settings::from_env();

    /*
        4) Criar Pool de conexões do Postgres (SQLx)

        - PgPoolOptions::new() cria o builder do pool.
        - max_connections(cfg.db_max_conns) define quantas conexões simultâneas manter abertas.
        - connect(&cfg.db_url).await efetivamente conecta (I/O assíncrono).
        - expect(...) derruba a aplicação com mensagem clara se falhar (DB off/credenciais etc).
    */
    let pool = PgPoolOptions::new()
        .max_connections(cfg.db_max_conns)
        .connect(&cfg.db_url)
        .await
        .expect("Falha ao conectar no Postgres");

    /*
        5) Montar o AppState

        - BookService recebe o pool.
        - AppState guarda o BookService.
        - Esse state será injetado automaticamente em cada handler via `State(AppState)`.
    */
    let state = AppState {
        book_service: BookService::new(pool),
    };

    /*
        6) Montar o Router (Axum)

        - Router::new() cria router vazio.
        - merge(controller::routes()) junta rotas definidas em módulos separados (health/books etc).
        - layer(TraceLayer::new_for_http()) adiciona middleware de log HTTP.
        - with_state(state) registra o estado global (AppState).
    */
    let app = axum::Router::new()
        .merge(controller::routes())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    /*
        7) Bind do servidor

        - Constrói o endereço com HOST:PORT.
        - parse() converte string -> SocketAddr.
        - TcpListener::bind(addr).await abre a porta e começa a escutar conexões.
    */
    let addr: SocketAddr = format!("{}:{}", cfg.host, cfg.port).parse().unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();

    /*
        8) Serve o app

        - axum::serve(listener, app).await mantém o servidor rodando.
        - Cada request entra no Router e cai nos handlers registrados.
        - unwrap() derruba o app se o servidor falhar.
    */
    axum::serve(listener, app).await.unwrap();
}
