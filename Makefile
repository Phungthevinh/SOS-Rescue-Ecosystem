# ==============================================================================
# Beacon Makefile
# ==============================================================================

.PHONY: setup db-up db-down run db-migrate check test format lint

setup:
	cargo install sqlx-cli --no-default-features --features rustls,postgres

db-up:
	docker-compose up -d

db-down:
	docker-compose down

db-logs:
	docker-compose logs -f

db-migrate:
	sqlx database create
	sqlx migrate run

run:
	cargo run --bin beacon_server

check:
	cargo check

test:
	cargo test

format:
	cargo fmt --all

lint:
	cargo clippy --all-targets --all-features -- -D warnings
