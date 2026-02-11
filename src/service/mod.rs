// src/service/mod.rs

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::ApiError,
    model::{Book, NewBook, UpdateBook},
    repository,
};

pub mod book_service {
    use crate::model::{Book, NewBook};
    use super::*;

    #[derive(Clone)]
    pub struct BookService {
        pool: PgPool,
    }

    impl BookService {
        pub fn new(pool: PgPool) -> Self { Self { pool } }

        pub async fn create(&self, dto: NewBook) -> Result<Book, ApiError> {
            let title = dto.title.trim().to_string();
            let author = dto.author.trim().to_string();

            if title.is_empty() { return Err(ApiError::bad_request("title é obrigatório")); }
            if author.is_empty() { return Err(ApiError::bad_request("author é obrigatório")); }
            if let Some(y) = dto.published_year {
                if y < 0 { return Err(ApiError::bad_request("published_year inválido")); }
            }

            repository::insert(&self.pool, Uuid::new_v4(), title, author, dto.published_year).await
        }

        pub async fn list(&self) -> Result<Vec<Book>, ApiError> {
            repository::list(&self.pool).await
        }

        pub async fn get(&self, id: Uuid) -> Result<Book, ApiError> {
            repository::find_by_id(&self.pool, id).await
        }

        pub async fn update(&self, id: Uuid, dto: UpdateBook) -> Result<Book, ApiError> {
            repository::update(&self.pool, id, dto.title, dto.author, dto.published_year).await
        }

        pub async fn delete(&self, id: Uuid) -> Result<(), ApiError> {
            repository::delete(&self.pool, id).await
        }
    }
}
