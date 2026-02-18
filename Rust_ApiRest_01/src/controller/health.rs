use axum::{routing::get, Router};
use crate::{errors::ApiError, AppState};

/* =============================================================================================
   MÓDULO: controller/health_controller.rs  (CONTROLLER)

   Papel do controller (Controller):
   - Esta camada é responsável por lidar com HTTP.
   - Ela registra rotas, recebe requests, extrai dados (path/query/body/state),
_toggle: chama o service e devolve uma resposta.

   O que deve ficar aqui:
   - Definição das rotas (URLs) e métodos HTTP (GET/POST/PUT/DELETE).
   - Extração de dados do request (State, Path, Query, Json).
   - Mapeamento do retorno do service para uma resposta HTTP.

   O que NÃO deve ficar aqui:
   - Regras de negócio (validação de campos, regras de atualização, etc.).
   - SQL (isso fica no repository).
   - O controller deve ser “fino”: pouco código, só orquestração do HTTP.

   Como esse controller funciona:
   - /health      -> testa se a API está no ar (sem depender do banco)
   - /health/db   -> testa se a API consegue falar com o Postgres (usando o service)
============================================================================================= */

/* =============================================================================================
   ROUTES: registra as rotas deste controller e retorna um Router configurado.

   Por que retornar um Router?
   - Você consegue organizar a API em vários controllers (books, health, etc.)
   - No main, você faz `.merge(controller::routes())` e junta tudo num Router só.
============================================================================================= */
pub fn routes() -> Router<AppState> 
{
    Router::new()
        /* `.route("/health", get(health))`:
           - Registra o endpoint GET /health
           - Quando bater nessa rota, o Axum chama a função `health`. */
        .route("/health", get(health))

        /* `.route("/health/db", get(health_db))`:
           - Registra o endpoint GET /health/db
           - Quando bater nessa rota, o Axum chama a função `health_db`. */
        .route("/health/db", get(health_db))
}

/* =============================================================================================
   HANDLER: health
   - Endpoint: GET /health
   - Retorno: &'static str

   O que significa &'static str:
   - Uma string "fixa" que vive durante toda a execução do programa.
   - Aqui é perfeito porque a resposta é constante: "ok".
============================================================================================= */
async fn health() -> &'static str 
{
    "ok"
}

/* =============================================================================================
   HANDLER: health_db
   - Endpoint: GET /health/db
   - Objetivo: validar conectividade com o banco via service.

   Extração de State:
   - `axum::extract::State(state)` pede pro Axum injetar o estado compartilhado (AppState).
   - Esse AppState foi registrado no main com `.with_state(state)`.
   - Com isso, o handler consegue acessar `state.book_service`.

   Retorno:
   - Result<&'static str, ApiError>
   - Se der tudo certo, devolve "db_ok"
   - Se der qualquer erro (ex.: banco fora, pool quebrado), retorna ApiError (que vira resposta HTTP)
============================================================================================= */
async fn health_db(axum::extract::State(state): axum::extract::State<AppState>) -> Result<&'static str, ApiError> 
{
    /* `state.book_service.ping().await?;`
       - Chama o service (regra de negócio/orquestração).
       - O service chama o repository, que executa `SELECT 1` no banco.
       - `.await` porque envolve I/O (rede).
       - `?` significa:
         - se ping der Ok(()), continua
         - se ping der Err(ApiError), retorna imediatamente esse erro para o Axum */
    state.book_service.ping().await?;

    /* Se chegou aqui, o banco respondeu corretamente. */
    Ok("db_ok")
}
