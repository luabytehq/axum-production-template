use axum::response::IntoResponse;
use axum_production_template::{handlers::health::liveness_check, ServerConfig};

#[tokio::test]
async fn test_liveness_endpoint_returns_ok() {
    let response = liveness_check().await.into_response();
    assert_eq!(response.status(), axum::http::StatusCode::OK);

    let body_bytes = axum::body::to_bytes(response.into_body(), 1024)
        .await
        .expect("Failed to read response body");

    let json_body: serde_json::Value =
        serde_json::from_slice(&body_bytes).expect("Failed to parse JSON body");

    assert_eq!(json_body["status"], "healthy");
}

#[test]
fn test_config_parsing_defaults() {
    std::env::set_var("DATABASE_URL", "postgres://localhost/test_db");
    let config = ServerConfig::from_env().expect("Config should parse with valid DATABASE_URL");

    assert_eq!(config.db_max_connections, 50);
    assert_eq!(config.db_min_connections, 10);
    assert_eq!(config.db_acquire_timeout_secs, 3);
}
