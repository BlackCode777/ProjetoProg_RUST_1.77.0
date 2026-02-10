use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ErrorBody {error: String}

#[derive(Debug)]
pub struct ApiError(pub StatusCode, pub String);

impl ApiError{
    
    pub fn not_found(msg: &str) -> Self {
        Self(StatusCode::NOT_FOUND, msg.into())
    }

    pub fn bad_request(msg: &str) -> Self { Self(StatusCode::BAD_REQUEST, msg.into()) }

    pub fn internal(e: impl ToString) -> Self { Self(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()) }

}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.0, Json(ErrorBody { error: self.1 })).into_response()
    }
}