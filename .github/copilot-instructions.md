# AI Coding Agent Instructions for learn-rust

## Project Overview
Production-ready Rust microservice demonstrating modern async development practices, Kubernetes deployment, and comprehensive CI/CD. RESTful API with 6 endpoints (/, /ping, /healthz, /info, /version, /echo), security middleware, and multi-environment testing (local, Docker, Kubernetes).

## Architecture & Code Organization

### Standard Rust Project Layout (src/)
- **`src/main.rs`**: Application entry point (~70 lines) - sets up Axum router and starts server
- **`src/handlers.rs`**: HTTP request handlers with business logic (all 6 endpoints)
- **`src/middleware.rs`**: Security headers middleware
- **`src/models.rs`**: Shared models (ApiResponse, AppInfo, HealthData, etc.)
- **`src/tests.rs`**: Integration tests for all endpoints
- **`scripts/`**: Testing scripts (smoke-test.sh, e2e-test.sh, integration-test.sh)

### Key Architectural Patterns
1. **Axum Web Framework**: Modern, ergonomic, and fast async web framework
2. **Middleware Stack**: Security headers applied via Axum middleware
3. **State Management**: Arc<AppState> shared across handlers for app info
4. **Async/Await**: All handlers are async using Tokio runtime
5. **Type-Safe JSON**: Serde for serialization/deserialization

## Critical Developer Workflows

### Local Development
```bash
make dev                    # Quickest: cargo run (port 8080)
PORT=3000 cargo run         # Custom port

# Test endpoints
curl http://localhost:8080/healthz  # Detailed health with system metrics
curl http://localhost:8080/ping     # Simple pong response
curl -X POST -H "Content-Type: application/json" -d '{"message":"test"}' http://localhost:8080/echo
```

### Testing Strategy
```bash
# Unit tests
cargo test

# Tests with output
cargo test -- --nocapture

# Coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out Html --output-dir coverage

# Integration tests (LOCAL DEVELOPMENT ONLY - not in CI)
./scripts/integration-test.sh

# E2E tests (KUBERNETES ONLY - runs in CI after Kind deployment)
./scripts/smoke-test.sh          # Quick validation
./scripts/e2e-test.sh            # Comprehensive tests
```

### CI/CD Workflow (Unified full-workflow.yml)
**Single pipeline - streamlined for efficiency**

```
lint → test (matrix: 1.86) → build → helm-test
        ↓                      ↓
  security-scan          (artifact)
                              ↓
                       test-deployment (Kind)
                              ├─ Load image into Kind
                              ├─ Helm deploy
                              ├─ smoke-test.sh
                              ├─ e2e-test.sh
                              └─ health checks
                              ↓
              ┌──────────────────────────────┐
              ↓                              ↓
       deploy-staging              deploy-production
       (main branch)               (tags only)
       FluxCD webhook              FluxCD webhook
```

**Key Implementation Details:**
- Docker build: Multi-platform (amd64/arm64) with Alpine Linux
- Artifact sharing: Image exported to tar → uploaded → downloaded by test-deployment
- Kind cluster: Uses `.github/kind-config.yaml` with port mappings (80, 443)
- Helm deployment: `--set image.pullPolicy=Never --set autoscaling.enabled=false` for Kind
- **Test execution order**: Helm deploy → smoke-test.sh → e2e-test.sh → health checks

## Project-Specific Conventions

### HTTP Response Structure (CRITICAL)
All JSON endpoints return `models::ApiResponse<T>`:
```rust
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: String,  // RFC3339 format
}
```

### Handler Implementation Pattern
```rust
// Example: src/handlers.rs
pub async fn healthz(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let health = HealthData {
        status: "healthy".to_string(),
        uptime,
        memory: memory_info,
        system: system_info,
    };
    Json(ApiResponse::success(health))
}
```
**Exception**: `/ping` returns plain text "pong" (not JSON)

### Middleware Pattern
```rust
// src/middleware.rs
pub async fn security_headers<B>(
    request: Request<B>,
    next: Next<B>,
) -> Response {
    let mut response = next.run(request).await;
    // Add security headers
    response
}
```

### Environment Variable Pattern
```rust
// Always provide defaults
let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
```

## Integration Points & External Dependencies

### Cargo Dependencies (Cargo.toml)
- **axum** 0.7: Web framework with routing and middleware
- **tokio** 1.x: Async runtime with full features
- **serde/serde_json**: Serialization/deserialization
- **tower/tower-http**: Middleware (CORS, tracing, compression)
- **sysinfo**: System metrics for `/healthz`
- **chrono**: Time handling with RFC3339
- **tracing**: Application-level tracing

### Kubernetes/Helm Chart (k8s/chart/)
- **Chart name**: `learn-rust` (defined in Chart.yaml)
- **Release name**: `learn-rust` (used in CI deployment)
- **Resource naming**: Helm creates resources with chart name
- **Deployment**: Uses image `dxas90/learn-rust`
- **Service**: ClusterIP service on port 8080
- **Pod labels**: `app.kubernetes.io/name=learn-rust`
- **Default config**: 1 replica, ClusterIP service
- **Probes**: `readinessProbe` & `livenessProbe` hit `/healthz`
- **HTTPRoute**: Optional Gateway API routing (disabled by default)

### Docker Build (Dockerfile)
```dockerfile
# Stage 1: rust:1.86-alpine → builds release binary
# Stage 2: alpine:3.21 → minimal runtime (~20MB image)
EXPOSE 8080  # Container listens on 8080
```

### CI/CD Secrets Required
- **GitHub**: `FLUX_STAGING_RECEIVER_URL`, `FLUX_STAGING_WEBHOOK_SECRET`
- **GitHub**: `FLUX_PRODUCTION_RECEIVER_URL`, `FLUX_PRODUCTION_WEBHOOK_SECRET`
- **Auto-available**: `GITHUB_TOKEN` (for GHCR push)

## Common Pitfalls & Important Notes

1. **Cargo.lock**: SHOULD be committed for binary crates (applications)
2. **Async functions**: All handlers must be async and return `impl IntoResponse`
3. **State sharing**: Use `Arc<AppState>` for shared state across handlers
4. **Error handling**: Use `Result<T, E>` and proper error types
5. **Kind testing**: Image loaded with `--set image.pullPolicy=Never`
6. **Helm naming**: Chart creates `learn-rust` resources
7. **Pod labels**: Use `app.kubernetes.io/name=learn-rust` in kubectl selectors
8. **Workflow artifact**: Image built ONCE, shared via tar

## Quick Reference: Key Files & Commands

### Essential Files
- **Entry**: `src/main.rs` - sets up Axum router and starts server
- **Routes**: `src/handlers.rs` - all 6 endpoints + business logic
- **Models**: `src/models.rs` - all data structures
- **CI/CD**: `.github/workflows/full-workflow.yml` (unified pipeline)
- **Tests**: `scripts/{smoke,e2e,integration}-test.sh` - bash testing scripts
- **Helm**: `k8s/chart/values.yaml` - deployment configuration

### Daily Commands
```bash
# Development
make dev                           # Start server (port 8080)
cargo test -- --nocapture          # Run tests with output

# Local integration testing
./scripts/integration-test.sh      # Tests local app + Docker build

# Docker local testing
docker build -t learn-rust . && docker run -p 8080:8080 learn-rust

# Makefile targets
make help                          # Show all available targets
make test-coverage                 # Generate coverage.html
make check                         # Run fmt + clippy + test
```

### Debugging Kind Cluster Issues
```bash
# Check if cluster exists
kind get clusters

# Load image manually
kind load docker-image learn-rust:test --name test-cluster

# Check pods in Kind
kubectl get pods -l app.kubernetes.io/name=learn-rust
kubectl logs -l app.kubernetes.io/name=learn-rust

# Check deployment and service
kubectl get deployment learn-rust
kubectl get service learn-rust

# Port-forward for local testing
kubectl port-forward service/learn-rust 8080:8080
```

## Rust-Specific Best Practices

1. **Use `cargo fmt`** before committing - enforces consistent style
2. **Run `cargo clippy`** - catches common mistakes and suggests improvements
3. **Leverage type system** - use strong types instead of strings where possible
4. **Async best practices** - avoid blocking operations in async context
5. **Error handling** - use `?` operator and proper error types
6. **Testing** - use `#[tokio::test]` for async tests
7. **Dependencies** - keep minimal, prefer well-maintained crates
