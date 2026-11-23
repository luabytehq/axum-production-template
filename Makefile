.PHONY: help dev check test fmt clippy compose-up compose-down

help:
	@echo "Available commands:"
	@echo "  make dev          - Run local development server"
	@echo "  make check        - Run cargo check"
	@echo "  make test         - Run test suite"
	@echo "  make fmt          - Run cargo fmt code formatting"
	@echo "  make clippy       - Run cargo clippy linter"
	@echo "  make compose-up   - Start PostgreSQL and service in Docker Compose"
	@echo "  make compose-down - Stop Docker Compose services"

dev:
	cargo run

check:
	cargo check --all-targets

test:
	cargo test --verbose

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets -- -D warnings

compose-up:
	docker compose up -d

compose-down:
	docker compose down
