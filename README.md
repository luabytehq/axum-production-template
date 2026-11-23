# Axum Production Service Blueprint

A high-concurrency microservice foundation built with Rust, Axum, Tokio, and SQLx.

## Architectural Highlights

- **Asynchronous Execution:** Built directly on Tokio's multi-threaded work-stealing runtime.
- **Connection Pooling:** PostgreSQL connection management configured via Little's Law to prevent database connection exhaustion.
- **Resilience Layers:** Composed Tower middleware including execution timing metrics, request timeouts, and response compression.
- **Structured Error Handling:** RFC 7807 problem responses with unified status mapping.
- **Zero-Downtime Signal Handling:** Clean shutdown listener for SIGINT and SIGTERM to drain active requests and safely disconnect the database pool.

## Quick Start

### 1. Configuration
Copy the sample environment file and set your database connection string:

```bash
cp .env.example .env
```

### 2. Run Database Migrations
Apply database schema using SQLx:

```bash
cargo sqlx migrate run
```

### 3. Start Development Server
```bash
cargo run
```

The service listens on `http://127.0.0.1:8080` by default.

## API Endpoints

- `GET /health/live` - Service liveness status
- `GET /health/ready` - Database connection readiness probe
- `POST /api/v1/accounts` - Create new account
- `GET /api/v1/accounts/:id` - Fetch account by ID
- `PUT /api/v1/accounts/:id/balance` - Adjust account balance atomically

## Documentation & Research

For an in-depth breakdown of the architecture, memory allocator tuning with jemalloc, and high-concurrency benchmarks, visit our engineering publication:

- [Architecting Scalable Rust Services with Axum](https://luabyte.com/blog/programming/scalable-rust-web-services-axum)
- [LuaByte Technical Portal](https://luabyte.com)

## Community

Join the discussion on our [Discord Community](https://discord.com/invite/mQtVnWkhSG).
