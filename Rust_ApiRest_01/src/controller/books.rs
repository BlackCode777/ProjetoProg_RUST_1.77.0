use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use uuid::Uuid;

use crate::{
    errors::ApiError,
    model::{Book, NewBook, UpdateBook},
    AppState,
};

/* =============================================================================================
   MÓDULO: controller/book_controller.rs  (CONTROLLER)

   Papel do controller (Controller):
   - Esta camada é responsável por lidar com HTTP:
     * define rotas e métodos (GET/POST/PUT/DELETE)
     * extrai dados do request (Path, Json, State)
     * chama o service
     * devolve resposta HTTP (status + body)

   O que NÃO deve ficar aqui:
   - Regras de negócio (validações: title obrigatório, etc.)
   - SQL (queries)
   O controller deve ser “fino”: apenas ponte entre HTTP e Service.

   Tipos do Axum usados aqui:
   - Router: registrador de rotas.
   - State<T>: extrator do estado compartilhado (AppState).
   - Path<T>: extrator de parâmetros da URL (ex.: {id}).
   - Json<T>: extrator/serializador de JSON no body.
   - StatusCode: código HTTP (201, 204 etc.).
============================================================================================= */

/* =============================================================================================
   ROUTES: registra as rotas do recurso "books".

   Por que retornar um Router<AppState>?
   - Você organiza o projeto por controllers.
   - No main, junta tudo com `.merge(controller::routes())`.
============================================================================================= */
pub fn routes() -> Router<AppState>
{
    Router::new()

        /* /books:
           - POST /books  -> create_book (criar novo livro)
           - GET  /books  -> list_books  (listar livros) */
        .route("/books", post(create_book).get(list_books))

        /* /books/{id}:
           - GET    /books/{id} -> get_book     (buscar por id)
           - PUT    /books/{id} -> update_book  (atualizar por id)
           - DELETE /books/{id} -> delete_book  (remover por id)

           Observação:
           - Em Axum 0.8, o padrão do path é `{id}` (chaves).
           - Se usar `:id`, dá panic (você já viu esse erro). */
        .route("/books/{id}", get(get_book).put(update_book).delete(delete_book))
}

/* =============================================================================================
   HANDLER: create_book
   - Endpoint: POST /books
   - Entrada: JSON (NewBook)
   - Saída: (201 CREATED, JSON(Book))

   Extrações:
   - State(state): injeta AppState (para acessar book_service).
   - Json(payload): lê body JSON e desserializa para NewBook.

   Retorno:
   - Result<(StatusCode, Json<Book>), ApiError>
   - Se Ok -> devolve status 201 + book criado.
   - Se Err -> ApiError vira resposta HTTP (ex.: 400, 404, 500).
============================================================================================= */
async fn create_book( State(state): State<AppState>, Json(payload): Json<NewBook>) ->
                                                                                   Result<(StatusCode, Json<Book>), ApiError>
{
    /* Chama o service:
       - O service valida (ex.: title obrigatório) e chama o repository para persistir.
       - `.await` porque envolve I/O (banco).
       - `?` propaga ApiError imediatamente se falhar. */
    let book = state.book_service.create(payload).await?;

    /* Retorna 201 + JSON do book criado. */
    Ok((StatusCode::CREATED, Json(book)))
}

/* =============================================================================================
   HANDLER: list_books
   - Endpoint: GET /books
   - Saída: JSON(Vec<Book>)

   Extrações:
   - State(state): injeta AppState.

   Retorno:
   - Result<Json<Vec<Book>>, ApiError>
============================================================================================= */
async fn list_books(State(state): State<AppState>) ->
                                                   Result<Json<Vec<Book>>, ApiError>
{
    /* O service lista e retorna Vec<Book>.
       O controller só empacota em Json(...) para o Axum serializar. */
    Ok(Json(state.book_service.list().await?))
}

/* =============================================================================================
   HANDLER: get_book
   - Endpoint: GET /books/{id}
   - Entrada: Path<Uuid> (id vindo da URL)
   - Saída: JSON(Book)

   Extrações:
   - Path(id): converte o segmento {id} para Uuid.
     * Se o {id} não for UUID válido, o Axum falha na extração e retorna erro automaticamente.
============================================================================================= */
async fn get_book( State(state): State<AppState>, Path(id): Path<Uuid>) ->
                                                                        Result<Json<Book>, ApiError>
{
    /* O service tenta buscar:
       - Se existir: Ok(Book)
       - Se não existir: ApiError::not_found (404)
       - Se der erro de banco: ApiError::internal (500) */
    Ok(Json(state.book_service.get(id).await?))
}

/* =============================================================================================
   HANDLER: update_book
   - Endpoint: PUT /books/{id}
   - Entrada:
     * Path<Uuid> (id)
     * Json<UpdateBook> (payload com campos opcionais)
   - Saída: JSON(Book) com registro atualizado
============================================================================================= */
async fn update_book(State(state): State<AppState>, Path(id): Path<Uuid>, Json(payload): Json<UpdateBook>) ->
                                                                                                           Result<Json<Book>, ApiError>
{
    /* O service orquestra a atualização e devolve o book atualizado. */
    Ok(Json(state.book_service.update(id, payload).await?))
}

/* =============================================================================================
   HANDLER: delete_book
   - Endpoint: DELETE /books/{id}
   - Saída: StatusCode::NO_CONTENT (204)

   Por que 204?
   - Padrão REST comum: deletou com sucesso e não retorna body.
============================================================================================= */
async fn delete_book(State(state): State<AppState>,Path(id): Path<Uuid>) ->
                                                                         Result<StatusCode, ApiError>
{
    /* Se não existir, o service/repository devolve 404.
       Se der erro de banco, devolve 500.
       Se deletar, Ok(()) e seguimos para retornar 204. */
    state.book_service.delete(id).await?;

    Ok(StatusCode::NO_CONTENT)
}
