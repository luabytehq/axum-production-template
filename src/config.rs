use std::env;
use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub addr: SocketAddr,
    pub database_url: String,
    pub db_max_connections: u32,
    pub db_min_connections: u32,
    pub db_acquire_timeout_secs: u64,
    pub request_timeout_secs: u64,
}

impl ServerConfig {
    pub fn from_env() -> Result<Self, String> {
        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .map_err(|e| format!("Invalid PORT: {e}"))?;

        let addr = format!("{host}:{port}")
            .parse::<SocketAddr>()
            .map_err(|e| format!("Failed to parse server address {host}:{port}: {e}"))?;

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| "Missing required environment variable: DATABASE_URL".to_string())?;

        let db_max_connections = env::var("DATABASE_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "50".to_string())
            .parse::<u32>()
            .unwrap_or(50);

        let db_min_connections = env::var("DATABASE_MIN_CONNECTIONS")
            .unwrap_or_else(|_| "10".to_string())
            .parse::<u32>()
            .unwrap_or(10);

        let db_acquire_timeout_secs = env::var("DATABASE_ACQUIRE_TIMEOUT_SECS")
            .unwrap_or_else(|_| "3".to_string())
            .parse::<u64>()
            .unwrap_or(3);

        let request_timeout_secs = env::var("REQUEST_TIMEOUT_SECS")
            .unwrap_or_else(|_| "10".to_string())
            .parse::<u64>()
            .unwrap_or(10);

        Ok(Self {
            addr,
            database_url,
            db_max_connections,
            db_min_connections,
            db_acquire_timeout_secs,
            request_timeout_secs,
        })
    }
}
