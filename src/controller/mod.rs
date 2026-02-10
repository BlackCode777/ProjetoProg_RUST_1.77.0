// src/controller/mod.rs
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use uuid::Uuid;

use crate::{
    errors::ApiError,
    model::book_model::{Book, NewBook, UpdateBook},
    AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/books", post(create_book).get(list_books))
        .route("/books/{id}", get(get_book).put(update_book).delete(delete_book))
}

async fn create_book(
    State(state): State<AppState>,
    Json(payload): Json<NewBook>,
) -> Result<(StatusCode, Json<Book>), ApiError> {
    let book = state.book_service.create(payload).await?;
    Ok((StatusCode::CREATED, Json(book)))
}

async fn list_books(State(state): State<AppState>) -> Result<Json<Vec<Book>>, ApiError> {
    Ok(Json(state.book_service.list().await?))
}

async fn get_book(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<Book>, ApiError> {
    Ok(Json(state.book_service.get(id).await?))
}

async fn update_book(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateBook>,
) -> Result<Json<Book>, ApiError> {
    Ok(Json(state.book_service.update(id, payload).await?))
}

async fn delete_book(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<StatusCode, ApiError> {
    state.book_service.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}