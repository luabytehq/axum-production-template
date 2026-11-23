use crate::error::ServiceError;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use sqlx::PgPool;
use std::sync::Arc;

pub async fn liveness_check() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({ "status": "healthy" })))
}

pub async fn readiness_check(
    State(pool): State<Arc<PgPool>>,
) -> Result<impl IntoResponse, ServiceError> {
    sqlx::query("SELECT 1")
        .execute(pool.as_ref())
        .await
        .map_err(|e| ServiceError::Internal(format!("Database health check failed: {e}")))?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "status": "ready",
            "database": "connected"
        })),
    ))
}
