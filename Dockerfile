# Syntax: Multi-stage container build for LuaByte Axum service

# Stage 1: Build the optimized binary
FROM rust:1.85-slim-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests for layer caching
COPY Cargo.toml Cargo.lock ./

# Copy source code and migrations
COPY src/ ./src/
COPY migrations/ ./migrations/

# Compile release binary
RUN cargo build --release --locked

# Stage 2: Minimal runtime image
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# Install runtime certificates and OpenSSL
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Run as dedicated unprivileged user for container security
RUN useradd -m -u 10001 -s /bin/bash appuser

# Copy executable from builder
COPY --from=builder /app/target/release/axum-production-template /app/server
COPY --from=builder /app/migrations /app/migrations

RUN chown -R appuser:appuser /app
USER appuser

EXPOSE 8080

ENV HOST=0.0.0.0
ENV PORT=8080
ENV RUST_LOG=info,axum_production_template=info

CMD ["/app/server"]
