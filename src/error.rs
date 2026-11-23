use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    BadRequest(String),

    #[error("Internal service error: {0}")]
    Internal(String),
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let (status, client_message) = match &self {
            ServiceError::Database(err) => {
                tracing::error!(target: "service_error", error = %err, "Database operation failed");
                match err {
                    sqlx::Error::RowNotFound => (
                        StatusCode::NOT_FOUND,
                        "Requested record does not exist".to_string(),
                    ),
                    _ => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "An internal database error occurred".to_string(),
                    ),
                }
            }
            ServiceError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            ServiceError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            ServiceError::Internal(msg) => {
                tracing::error!(target: "service_error", error = %msg, "Internal error encountered");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An unexpected error occurred".to_string(),
                )
            }
        };

        let body = Json(json!({
            "status": status.as_u16(),
            "error": client_message,
        }));

        (status, body).into_response()
    }
}
