use crate::{
    handlers::{account, health},
    middleware::track_execution_time,
};
use axum::{
    http::StatusCode,
    middleware,
    routing::{get, post, put},
    Router,
};
use sqlx::PgPool;
use std::{sync::Arc, time::Duration};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, timeout::TimeoutLayer, trace::TraceLayer,
};

pub fn build_router(pool: Arc<PgPool>, request_timeout_secs: u64) -> Router {
    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(request_timeout_secs),
        ))
        .layer(CompressionLayer::new().gzip(true).br(true))
        .layer(CorsLayer::permissive());

    Router::new()
        .route("/health/live", get(health::liveness_check))
        .route("/health/ready", get(health::readiness_check))
        .route("/api/v1/accounts", post(account::create_account))
        .route("/api/v1/accounts/{id}", get(account::get_account))
        .route(
            "/api/v1/accounts/{id}/balance",
            put(account::adjust_balance),
        )
        .layer(middleware::from_fn(track_execution_time))
        .layer(middleware_stack)
        .with_state(pool)
}
