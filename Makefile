.PHONY: help build run test clean docker-build docker-run install dev fmt clippy

# Default target
help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-15s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

# Development
install: ## Install dependencies
	cargo fetch

dev: ## Run the application in development mode
	cargo run

run: ## Run the application
	cargo run --release

# Building
build: ## Build the application
	cargo build --release

# Testing
test: ## Run all tests
	cargo test

test-coverage: ## Run tests with coverage
	cargo tarpaulin --out Html --output-dir coverage

# Docker
docker-build: ## Build Docker image
	docker build -t learn-rust .

docker-run: ## Run Docker container
	docker run -p 8080:8080 learn-rust

# Cleanup
clean: ## Clean build artifacts
	cargo clean
	rm -rf coverage/

# Linting
lint: clippy ## Alias for clippy

clippy: ## Run clippy linter
	cargo clippy -- -D warnings

# Formatting
fmt: ## Format code
	cargo fmt

fmt-check: ## Check code formatting
	cargo fmt -- --check

# Check everything
check: ## Run all checks (fmt, clippy, test)
	cargo fmt -- --check
	cargo clippy -- -D warnings
	cargo test
