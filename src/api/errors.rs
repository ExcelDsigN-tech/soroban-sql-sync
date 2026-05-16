use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    NotFound(String),
    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
    code: &'static str,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            ApiError::BadRequest(message) => {
                (StatusCode::BAD_REQUEST, "bad_request", message.clone())
            }
            ApiError::NotFound(message) => (StatusCode::NOT_FOUND, "not_found", message.clone()),
            ApiError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                self.to_string(),
            ),
        };

        (status, Json(ErrorResponse { error: message, code })).into_response()
    }
}
