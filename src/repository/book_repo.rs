use sqlx::PgPool;
use uuid::Uuid;

use crate::{errors::ApiError, model::book_model::Book};

pub async fn insert(pool: &PgPool, id: Uuid, title: String, author: String, year: Option<i32>)
                    -> Result<Book, ApiError>
{
    sqlx::query_as::<sqlx::Postgres, Book>(r#"
        INSERT INTO books (id, title, author, published_year)
        VALUES ($1, $2, $3, $4)
        RETURNING id, title, author, published_year, created_at
    "#)
        .bind(id).bind(title).bind(author).bind(year)
        .fetch_one(pool).await
        .map_err(ApiError::internal)
}

pub async fn list(pool: &PgPool) -> Result<Vec<Book>, ApiError> {
    sqlx::query_as::<sqlx::Postgres, Book>(r#"
        SELECT id, title, author, published_year, created_at
        FROM books
        ORDER BY created_at DESC
    "#)
        .fetch_all(pool).await
        .map_err(ApiError::internal)
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Book, ApiError>
{
    sqlx::query_as::<sqlx::Postgres, Book>(r#"
        SELECT id, title, author, published_year, created_at
        FROM books
        WHERE id = $1
    "#)
        .bind(id)
        .fetch_optional(pool).await
        .map_err(ApiError::internal)?
        .ok_or_else(|| ApiError::not_found("Book not found"))
}

pub async fn update(pool: &PgPool, id: Uuid, title: Option<String>, author: Option<String>, year: Option<i32>)
                    -> Result<Book, ApiError>
{
    sqlx::query_as::<sqlx::Postgres, Book>(r#"
        UPDATE books
        SET
            title = COALESCE($2, title),
            author = COALESCE($3, author),
            published_year = COALESCE($4, published_year)
        WHERE id = $1
        RETURNING id, title, author, published_year, created_at
    "#)
        .bind(id).bind(title).bind(author).bind(year)
        .fetch_optional(pool).await
        .map_err(ApiError::internal)?
        .ok_or_else(|| ApiError::not_found("Book not found"))
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), ApiError>
{
    let result = sqlx::query(r#"
        DELETE FROM books
        WHERE id = $1
    "#)
        .bind(id)
        .execute(pool).await
        .map_err(ApiError::internal)?;

    if result.rows_affected() == 0 {
        return Err(ApiError::not_found("Book not found"));
    }

    Ok(())
}
