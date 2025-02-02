.PHONY: up down restart logs clean test build run init-db

# Docker commands
up:
	docker-compose up -d

down:
	docker-compose down

restart:
	docker-compose restart

logs:
	docker-compose logs -f

clean:
	docker-compose down -v
	docker system prune -f

# Development commands
test:
	cargo test

build:
	cargo build --release

run-api:
	cargo run --bin portfolio-api

sync-rss:
	cargo run --bin sync_rss

run-format:
	cargo fmt

check:
	cargo check && cargo clippy

fmt:
	cargo fmt

# Database commands
init-db:
	docker-compose up -d mongodb
	@echo "Waiting for MongoDB to start..."
	@sleep 5
	@echo "MongoDB is ready!"

# Backup commands
backup:
	@mkdir -p backup
	@echo "Creating database backup..."
	docker-compose exec mongodb mongodump --uri="mongodb://${MONGO_USER}:${MONGO_PASSWORD}@localhost:27017/${MONGO_DB}?authSource=admin" --out=/data/db/backup/
	@echo "Backup completed!"

restore:
	@echo "Restoring database from backup..."
	docker-compose exec mongodb mongorestore --uri="mongodb://${MONGO_USER}:${MONGO_PASSWORD}@localhost:27017/${MONGO_DB}?authSource=admin" /data/db/backup/
	@echo "Restore completed!"

# Help command
help:
	@echo "Available commands:"
	@echo "  up          - Start Docker containers"
	@echo "  down        - Stop Docker containers"
	@echo "  restart     - Restart Docker containers"
	@echo "  logs        - View Docker container logs"
	@echo "  clean       - Remove Docker containers and volumes"
	@echo "  test        - Run Rust tests"
	@echo "  build       - Build the Rust project"
	@echo "  run         - Run the Rust project"
	@echo "  check       - Run cargo check and clippy"
	@echo "  fmt         - Format Rust code"
	@echo "  init-db     - Initialize MongoDB"
	@echo "  backup      - Create MongoDB backup"
	@echo "  restore     - Restore MongoDB from backup"
	@echo "  help        - Show this help message"
