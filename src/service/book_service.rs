use sqlx::PgPool;
use uuid::Uuid;

/* O serviço é a camada onde colocamos toda a lógica de negócio relacionada aos livros.
Ele é responsável por validar os dados, aplicar regras de negócio, e decidir como lidar com
erros. O repositório é apenas uma camada de acesso a dados, que o serviço usa para persistir
e recuperar informações do banco. O controlador (controller) é responsável por lidar com as
 requisições HTTP, extrair os dados da requisição, chamar o serviço e formatar a resposta.
 Essa separação de responsabilidades torna o código mais organizado, testável e fácil de manter.

 crate -> indica que estamos importando algo do mesmo pacote (src/). Aqui estamos importando:
- `ApiError` do módulo `errors` (src/errors.rs)
- `Book`, `NewBook`, `UpdateBook` do módulo `model` (src/model.rs)
- `book_repo` do módulo `repository` (src/repository/book_repo.rs */
use crate::{
    errors::ApiError,
    model::{Book, NewBook, UpdateBook},
    repository::book_repo,
};

/* O serviço é responsável por toda a lógica de negócio relacionada aos livros. Ele é
quem "sabe" o que um livro é, quais regras ele deve seguir, e como lidar com erros
relacionados a livros. O repositório é apenas uma camada de acesso a dados, que
o serviço usa para persistir e recuperar informações do banco. O controlador (controller)
é responsável por lidar com as requisições HTTP, extrair os dados da requisição, chamar
o serviço e formatar a resposta. Essa separação de responsabilidades torna o código
mais organizado, testável e fácil de manter.

derive -> gera código automaticamente para implementar traits comuns. Aqui, `#[derive(Clone)]`
gera uma implementação de `Clone` para `BookService`, permitindo que ele seja facilmente
clonado (útil para compartilhar o serviço entre threads/handlers). O `BookService` tem um
campo `pool` do tipo `PgPool`, que é a conexão com o banco de dados.
O serviço tem métodos assíncronos para criar, listar, obter, atualizar e deletar livros,
cada um deles usando o repositório para acessar o banco e aplicando validações e regras
de negócio antes de chamar o repositório.

Clone -> Permite criar uma cópia de um valor. No caso do `BookService`, isso é útil porque
o serviço pode ser compartilhado entre múltiplos handlers HTTP, e cada handler pode precisar
de sua própria "cópia" do serviço para evitar problemas de concorrência. O `PgPool` é projetado
para ser compartilhado e clonado, então isso funciona bem. */
#[derive(Clone)]
pub struct BookService {
    pool: PgPool,
}

/* O serviço é responsável por toda a lógica de negócio relacionada aos livros. Ele é
quem "sabe" o que um livro é, quais regras ele deve seguir, e como lidar com erros
relacionados a livros. O repositório é apenas uma camada de acesso a dados,
que o serviço usa para persistir e recuperar informações do banco. O controlador (controller)
é responsável por lidar com as requisições HTTP, extrair os dados da requisição, chamar
o serviço e formatar a resposta. Essa separação de responsabilidades torna o código
mais organizado, testável e fácil de manter.

impl -> Implementa métodos para a struct `BookService`. Aqui definimos os métodos que o serviço
oferece, como `new`, `ping`, `create`, `list`, `get`, `update` e `delete`. Cada método é responsável
por uma operação relacionada a livros, e pode chamar o repositório para acessar o banco de
dados. O serviço é onde colocamos toda a lógica de negócio, validações e tratamento de erros,
enquanto o repositório é apenas uma camada de acesso a dados. */
impl BookService {
    pub fn new(pool: PgPool) -> Self { Self { pool } }

    pub async fn ping(&self) -> Result<(), ApiError> {
        book_repo::ping(&self.pool).await
    }

    /* Validações básicas são feitas aqui, antes de chamar o repositório. O repositório assume que
    os dados já estão "limpos" e prontos para serem usados. Isso mantém a lógica de negócio
    (validações, regras) separada da lógica de acesso a dados (SQL). */
    pub async fn create(&self, dto: NewBook) -> Result<Book, ApiError> {
        let title:String = dto.title.trim().to_string();
        let author:String = dto.author.trim().to_string();

        if title.is_empty() { return Err(ApiError::bad_request("title é obrigatório")); }
        if author.is_empty() { return Err(ApiError::bad_request("author é obrigatório")); }
        if let Some(y) = dto.published_year {
            if y < 0 { return Err(ApiError::bad_request("published_year inválido")); }
        }

        book_repo::insert(&self.pool, Uuid::new_v4(), title, author, dto.published_year).await
    }

    /* O método `list` é simples porque não tem regras de negócio complexas, mas
            ainda assim é responsabilidade do serviço. O repositório só sabe "buscar dados",
             o serviço é quem decide "o que buscar" e "como tratar".  */
    pub async fn list(&self) -> Result<Vec<Book>, ApiError> {
        book_repo::list(&self.pool).await
    }

    /* O método `get` é responsável por decidir o que fazer se o livro não for encontrado.
            O repositório retorna um erro específico (ApiError::not_found) se o livro não existir,
            e o serviço pode decidir se quer propagar esse erro, transformar em outro tipo,
            ou adicionar lógica extra (ex: cache). */
    pub async fn get(&self, id: Uuid) -> Result<Book, ApiError> {
        book_repo::find_by_id(&self.pool, id).await
    }

    /* O método `update` é um pouco mais complexo porque pode receber campos opcionais.
            O serviço é responsável por validar esses campos (ex: não permitir título vazio) e
            decidir como lidar com atualizações parciais. O repositório só sabe "atualizar o que
            for passado", usando COALESCE para manter os valores antigos se nada for enviado. */
    pub async fn update(&self, id: Uuid, dto: UpdateBook) -> Result<Book, ApiError> {
        book_repo::update(&self.pool, id, dto.title, dto.author, dto.published_year).await
    }

    /* O método `delete` é simples, mas ainda assim é responsabilidade do serviço. O serviço pode
            decidir se quer adicionar lógica extra (ex: verificar permissões, registrar logs) antes
            de chamar o repositório para deletar o livro. O repositório só sabe "deletar por ID". */
    pub async fn delete(&self, id: Uuid) -> Result<(), ApiError> {
        book_repo::delete(&self.pool, id).await
    }
}
