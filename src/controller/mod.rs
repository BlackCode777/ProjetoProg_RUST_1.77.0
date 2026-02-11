use axum::Router;
use crate::AppState;

pub mod books;
pub mod health;

/* =============================================================================================
   MÓDULO: controller/mod.rs  (AGREGADOR DE ROTAS)

   Papel deste módulo:
   - Este arquivo funciona como um “hub” de controllers.
   - Ele não implementa endpoints diretamente (isso fica em books.rs e health.rs).
   - Ele só:
     1) declara os submódulos (`pub mod books; pub mod health;`)
     2) cria um Router central
     3) junta (merge) as rotas de cada controller em um único Router<AppState>

   Por que isso é útil:
   - Você mantém a API organizada por recursos:
     * controller/books.rs  -> rotas de livros
     * controller/health.rs -> rotas de saúde
   - No main.rs você faz apenas:
     `.merge(controller::routes())`
   - Se amanhã existir `users`, você só adiciona:
     `pub mod users;` e `.merge(users::routes())`

   Observação sobre visibilidade:
   - `pub mod books;` torna o módulo `books` acessível fora deste arquivo.
   - Sem `pub`, o módulo ficaria privado (só este módulo conseguiria acessar).
============================================================================================= */

pub fn routes() -> Router<AppState> 
{
    Router::new()
        /* `.merge(health::routes())`:
           - Pega o Router retornado pelo controller `health`
           - “cola” as rotas dele dentro deste Router principal.
           - Ex.: /health e /health/db passam a existir aqui. */
        .merge(health::routes())

        /* `.merge(books::routes())`:
           - Faz o mesmo para o controller de livros.
           - Ex.: /books e /books/{id} passam a existir aqui. */
        .merge(books::routes())
}
