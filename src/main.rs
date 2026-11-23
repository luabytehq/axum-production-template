use axum_production_template::{build_router, ServerConfig};
use sqlx::postgres::PgPoolOptions;
use std::{sync::Arc, time::Duration};
use tokio::signal;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[cfg(all(not(target_env = "msvc"), feature = "jemalloc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(all(not(target_env = "msvc"), feature = "jemalloc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,axum_production_template=debug,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer().compact())
        .init();

    let config = ServerConfig::from_env()
        .map_err(|e| format!("Configuration initialization failed: {e}"))?;

    tracing::info!(
        max_connections = config.db_max_connections,
        min_connections = config.db_min_connections,
        acquire_timeout_secs = config.db_acquire_timeout_secs,
        "Initializing PostgreSQL connection pool via Little's Law sizing"
    );

    let pool = PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .min_connections(config.db_min_connections)
        .acquire_timeout(Duration::from_secs(config.db_acquire_timeout_secs))
        .idle_timeout(Duration::from_secs(600))
        .connect(&config.database_url)
        .await
        .map_err(|e| format!("Failed to connect to PostgreSQL: {e}"))?;

    tracing::info!("PostgreSQL connection pool established successfully");

    // Optional: run automatic embedded migrations
    // sqlx::migrate!("./migrations").run(&pool).await?;

    let shared_pool = Arc::new(pool);
    let app = build_router(shared_pool.clone(), config.request_timeout_secs);

    tracing::info!(listen_addr = %config.addr, "Binding TCP listener");
    let listener = tokio::net::TcpListener::bind(config.addr).await?;

    tracing::info!(
        addr = %config.addr,
        "LuaByte Axum Production Server listening"
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("Shutdown signal received. Draining pool connections...");
    shared_pool.close().await;
    tracing::info!("Shutdown sequence complete.");

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to register Ctrl+C signal handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to register SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Initiating graceful shutdown from SIGINT (Ctrl+C)");
        },
        _ = terminate => {
            tracing::info!("Initiating graceful shutdown from SIGTERM");
        },
    }
}
