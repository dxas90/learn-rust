# learn-rust

[![Build and Test](https://github.com/dxas90/learn-rust/workflows/Docker%20Build%20and%20Security%20Scan/badge.svg)](https://github.com/dxas90/learn-rust/actions)
[![Rust Version](https://img.shields.io/badge/Rust-1.83-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

A simple Rust microservice for learning Kubernetes, Docker, and modern Rust development practices.

## 🚀 Features

- **RESTful API** with multiple endpoints
- **Health checks** and monitoring endpoints
- **CORS support** for cross-origin requests
- **Security headers** (X-Frame-Options, CSP, etc.)
- **Docker support** with multi-stage builds
- **Kubernetes ready** with deployment configurations
- **CI/CD pipelines** (GitHub Actions)
- **Comprehensive testing** with Rust testing framework
- **Production-ready** with proper logging and error handling
- **Async runtime** using Tokio
- **High performance** web framework with Axum

## 📋 API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/` | GET | Welcome page with API documentation |
| `/ping` | GET | Simple ping-pong health check |
| `/healthz` | GET | Detailed health check with system metrics |
| `/info` | GET | Application and system information |
| `/version` | GET | Application version information |
| `/echo` | POST | Echo back the request body |
| `/metrics` | GET | Prometheus metrics endpoint |

## ✨ Features

- **RESTful API**: 7 endpoints with consistent JSON responses
- **Security**: Custom security headers middleware (CORS, CSP, etc.)
- **Monitoring**:
  - Prometheus metrics (`/metrics` endpoint)
  - OpenTelemetry tracing support (configure via `OTEL_EXPORTER_OTLP_ENDPOINT`)
- **Health Checks**: Detailed health information with system metrics
- **Production Ready**: Multi-stage Docker build, graceful shutdown, non-root user
- **Kubernetes Ready**: Helm charts with HTTPRoute, autoscaling, persistence options
- **CI/CD**: GitHub Actions with linting, testing, security scanning, e2e tests

## 🛠️ Quick Start

### Prerequisites

- Rust 1.83 or higher
- Docker (optional)
- make (optional, for using Makefile commands)

### Local Development

1. **Clone the repository**
   ```sh
   git clone https://github.com/dxas90/learn-rust.git
   cd learn-rust
   ```

2. **Install dependencies**
   ```sh
   cargo fetch
   ```

3. **Run the application**
   ```sh
   cargo run
   ```

4. **Access the API**
   ```sh
   curl http://localhost:8080/
   ```

## 🔧 Development Commands

```bash
# Run in development mode
make dev

# Run with release optimizations
make run

# Build the application
make build

# Run tests
make test

# Run tests with coverage
make test-coverage

# Format code
make fmt

# Run linter (clippy)
make clippy

# Run all checks
make check
```

## 🐳 Docker

### Build Docker Image

```bash
docker build -t learn-rust .
```

### Run Docker Container

```bash
docker run -p 8080:8080 learn-rust
```

### Using Make

```bash
make docker-build
make docker-run
```

## ☸️ Kubernetes Deployment

### Using Helm

```bash
# Install the chart
helm install learn-rust ./k8s/chart

# Upgrade the release
helm upgrade learn-rust ./k8s/chart

# Uninstall
helm uninstall learn-rust
```

### Custom Values

```bash
helm install learn-rust ./k8s/chart \
  --set image.repository=your-registry/learn-rust \
  --set image.tag=latest \
  --set replicaCount=3
```

## 📊 Monitoring

The application exposes several endpoints for monitoring:

- **Health Check**: `/healthz` - Returns application health status with system metrics
- **Info**: `/info` - Returns detailed application and system information
- **Version**: `/version` - Returns application version information
- **Metrics**: `/metrics` - Prometheus metrics endpoint

## 📊 Monitoring & Observability

### Prometheus Metrics

The `/metrics` endpoint exposes Prometheus-format metrics:

```bash
curl http://localhost:8080/metrics
```

Available metrics:
- `http_requests_total` - Total number of HTTP requests
- `http_request_duration_seconds` - Request duration histogram
- `process_cpu_seconds_total` - CPU time
- `process_resident_memory_bytes` - Memory usage
- `process_open_fds` - Open file descriptors

### OpenTelemetry Tracing

Configure OpenTelemetry by setting the OTLP endpoint:

```bash
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
cargo run
```

Without the endpoint configured, the application runs normally without tracing.

## 🧪 Testing

### Run Unit Tests

```bash
cargo test
```

### Run Tests with Coverage

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html --output-dir coverage
```

### Integration Tests

```bash
./scripts/integration-test.sh
```

### Smoke Tests

```bash
./scripts/smoke-test.sh
```

### E2E Tests

```bash
./scripts/e2e-test.sh
```

## �� Security

- **Non-root user** in Docker container
- **Security headers** applied to all responses
- **Minimal base image** (Alpine Linux)
- **Regular dependency updates** via Renovate
- **Vulnerability scanning** with Trivy
- **Security audits** with cargo-audit

## 📝 API Response Format

All JSON endpoints return responses in the following format:

```json
{
  "success": true,
  "data": {
    "message": "Response data here"
  },
  "timestamp": "2024-01-01T00:00:00Z"
}
```

Error responses:

```json
{
  "success": false,
  "error": "Error message here",
  "timestamp": "2024-01-01T00:00:00Z"
}
```

## 🏗️ Project Structure

```
learn-rust/
├── src/
│   ├── main.rs           # Application entry point
│   ├── handlers.rs       # HTTP request handlers
│   ├── middleware.rs     # Middleware functions
│   ├── models.rs         # Data models
│   └── tests.rs          # Test modules
├── k8s/
│   ├── chart/            # Helm chart
│   └── app/              # Kubernetes manifests
├── scripts/              # Testing scripts
├── Cargo.toml            # Rust dependencies
├── Dockerfile            # Multi-stage Docker build
├── Makefile              # Development commands
└── README.md             # This file
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Related Projects

- [learn-go](https://github.com/dxas90/learn-go) - Go version
- [learn-node](https://github.com/dxas90/learn-node) - Node.js version
- [learn-ruby](https://github.com/dxas90/learn-ruby) - Ruby version
- [learn-java](https://github.com/dxas90/learn-java) - Java version

## 📚 Documentation

For more detailed documentation, see:

- [DOCUMENTATION.md](DOCUMENTATION.md) - Detailed technical documentation
- [GITHUB_WORKFLOWS.md](GITHUB_WORKFLOWS.md) - CI/CD pipeline documentation

## 🛠️ Technologies Used

- **Axum** - Modern, ergonomic web framework
- **Tokio** - Async runtime for Rust
- **Serde** - Serialization/deserialization
- **Tower** - Modular middleware
- **Sysinfo** - System information
- **Tracing** - Application-level tracing

## 🎯 Learning Goals

This project demonstrates:

- Modern Rust async programming with Tokio
- RESTful API design with Axum
- Middleware implementation
- Error handling patterns
- Testing strategies
- Docker containerization
- Kubernetes deployment
- CI/CD with GitHub Actions
- Security best practices
