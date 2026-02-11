use sqlx::PgPool;
use uuid::Uuid;

use crate::{errors::ApiError, model::Book};

/* =============================================================================================
   MÓDULO: repository/book_repo.rs  (REPOSITÓRIO)

   Papel do repositório (Repository):
   - Esta camada é responsável SOMENTE por acessar dados (SQL).
   - Aqui você escreve as queries (INSERT / SELECT / UPDATE / DELETE).
   - Aqui você converte o retorno do banco para structs (Book) via `query_as`.
   - Aqui você transforma erros do banco (sqlx::Error) em erro da API (ApiError).

   O que NÃO deve ficar aqui:
   - Regras de negócio: "title obrigatório", "ano não pode ser negativo", etc.
   - Isso fica no SERVICE, porque regra de negócio muda e precisa ser testável e centralizada.

   Por que isso é uma boa prática:
   - Controller cuida de HTTP.
   - Service cuida de regras.
   - Repository cuida de banco.
   Esse desacoplamento evita bagunça e facilita manutenção.
============================================================================================= */

/* =============================================================================================
   INSERT: cria um novo livro e retorna o registro criado.
   - Recebe: pool + campos já validados pelo service.
   - Retorna: Book (com created_at preenchido pelo banco).
============================================================================================= */
pub async fn insert(pool: &PgPool, id: Uuid, title: String,
    author: String, year: Option<i32> ) -> Result<Book, ApiError> 
{
    /* `sqlx::query_as::<_, Book>(SQL)`:
       - Cria uma query SQL e diz: "mapeie o resultado para a struct Book".
       - O `_` deixa o SQLx inferir o tipo de banco (Postgres) com base no pool.
       - O `r#"... "#` é string raw: evita ter que escapar aspas e barras. */

    sqlx::query_as::<_, Book>(r#"
        INSERT INTO books (id, title, author, published_year)
        VALUES ($1, $2, $3, $4)
        RETURNING id, title, author, published_year, created_at
    "#)
        /* `.bind(...)`:
           - Substitui os parâmetros $1..$4 com valores tipados.
           - Segurança: não concatena string, reduz risco de SQL injection.
           - Ordem importa: bind 1 -> $1, bind 2 -> $2, etc. */
        .bind(id)       /* $1 */
        .bind(title)    /* $2 */
        .bind(author)   /* $3 */
        .bind(year)     /* $4 (None vira NULL no Postgres) */

        /* `.fetch_one(pool).await`:
           - Pega 1 conexão do pool.
           - Executa a query.
           - Espera receber exatamente 1 linha (por causa do RETURNING).
           - Se não retornar linha, vira erro. */
        .fetch_one(pool).await

        /* `.map_err(ApiError::internal)`:
           - Converte erro do SQLx (ex.: conexão falhou, violação de constraint etc.)
             para ApiError (HTTP 500). */
        .map_err(ApiError::internal)
}

/* =============================================================================================
   LIST: lista livros do mais recente para o mais antigo.
============================================================================================= */
pub async fn list(pool: &PgPool) -> Result<Vec<Book>, ApiError> 
{
    /* Aqui também usamos query_as -> Book, mas retornando múltiplas linhas.
       O ORDER BY define a ordenação do resultado no banco. */

    sqlx::query_as::<_, Book>(r#"
        SELECT id, title, author, published_year, created_at
        FROM books
        ORDER BY created_at DESC
    "#)
        /* `.fetch_all(pool).await`:
           - Executa a query.
           - Retorna TODAS as linhas em memória como Vec<Book>. */
        .fetch_all(pool).await
        .map_err(ApiError::internal)
}

/* =============================================================================================
   FIND_BY_ID: busca livro por UUID. Se não existir, retorna 404.
============================================================================================= */
pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Book, ApiError> 
{
    
    sqlx::query_as::<_, Book>(r#"
        SELECT id, title, author, published_year, created_at
        FROM books
        WHERE id = $1
    "#)
        .bind(id) /* $1 */

        /* `.fetch_optional(pool).await`:
           - Ao invés de dar erro quando não acha registro, ele retorna Ok(None).
           - Isso é ideal para endpoints "GET /books/{id}".
           - Resultado possível:
             - Ok(Some(Book)) -> achou
             - Ok(None) -> não achou
             - Err(sqlx::Error) -> erro real (conexão, SQL etc.) */
        .fetch_optional(pool).await
        .map_err(ApiError::internal)?

        /* `.ok_or_else(...)`:
           - Se veio None, converte para erro 404 (not_found).
           - Se veio Some(book), devolve book normalmente. */
        .ok_or_else(|| ApiError::not_found("Book not found"))
}

/* =============================================================================================
   UPDATE: atualiza parcialmente (campos opcionais) e retorna o livro atualizado.
============================================================================================= */
pub async fn update( pool: &PgPool, id: Uuid, title: Option<String>, 
                     author: Option<String>, year: Option<i32>) -> Result<Book, ApiError> 
{
    /* Repare no COALESCE:
       - COALESCE($2, title) significa:
         "se $2 vier NULL, mantém o valor atual da coluna title".
       - Como Option<T> quando é None vira NULL no bind, isso permite PATCH-like update.
       - Usamos RETURNING pra trazer o registro atualizado sem fazer outro SELECT. */

    sqlx::query_as::<_, Book>(r#"
        UPDATE books
        SET
            title = COALESCE($2, title),
            author = COALESCE($3, author),
            published_year = COALESCE($4, published_year)
        WHERE id = $1
        RETURNING id, title, author, published_year, created_at
    "#)
        /* $1 é o id do WHERE, e $2..$4 são os valores atualizáveis. */
        .bind(id)     /* $1 */
        .bind(title)  /* $2 (None -> NULL -> mantém valor antigo) */
        .bind(author) /* $3 */
        .bind(year)   /* $4 */

        /* fetch_optional:
           - Se o id não existir, o UPDATE não afeta nada e não retorna linha.
           - Resultado: Ok(None) -> vira 404. */
        .fetch_optional(pool).await
        .map_err(ApiError::internal)?
        .ok_or_else(|| ApiError::not_found("Book not found"))
}

/* =============================================================================================
   DELETE: remove livro por id. Se não existir, retorna 404.
============================================================================================= */
pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), ApiError> 
{
    /* sqlx::query(...) (sem query_as) porque DELETE não retorna Book.
       execute(...) retorna um resultado com rows_affected(). */

    let result = sqlx::query("DELETE FROM books WHERE id = $1")
        .bind(id)
        .execute(pool).await
        .map_err(ApiError::internal)?;

    /* rows_affected():
       - Se 0, significa que não existia registro com aquele id. */
    if result.rows_affected() == 0 {
        return Err(ApiError::not_found("Book not found"));
    }

    Ok(())
}

/* =============================================================================================
   PING: verifica se o banco está respondendo (SELECT 1).
============================================================================================= */
pub async fn ping(pool: &PgPool) -> Result<(), ApiError> 
{
    /* SELECT 1:
       - Query mínima só pra testar conectividade e execução.
       - Se o banco estiver fora, credencial errada, pool quebrado, etc., falha aqui. */

    sqlx::query("SELECT 1")
        .execute(pool).await
        .map_err(ApiError::internal)?;

    Ok(())
}

/* =============================================================================================
   OBSERVAÇÃO IMPORTANTE (pra estudo)

   No `update`, `None` significa "não atualizar este campo".
   Por causa do COALESCE, você não consegue "setar published_year = NULL", porque:
   - None vira NULL -> COALESCE mantém o valor antigo.

   Se você quiser 3 estados:
   1) campo ausente -> não altera
   2) campo presente com null -> seta NULL
   3) campo presente com valor -> atualiza valor
   Você pode usar `Option<Option<i32>>` no DTO.
============================================================================================= */
