// src/model/mod.rs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, FromRow)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub published_year: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct NewBook {
    pub title: String,
    pub author: String,
    pub published_year: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBook {
    pub title: Option<String>,
    pub author: Option<String>,
    pub published_year: Option<i32>,
}

