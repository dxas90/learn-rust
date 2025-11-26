# Build stage
FROM rust:1.83-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev openssl-dev

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock* ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release || true && \
    rm -rf src

# Copy source code
COPY src ./src

# Build application with optimizations
RUN cargo build --release && \
    strip target/release/learn-rust

# Runtime stage
FROM alpine:3.21

# Install runtime dependencies
RUN apk add --no-cache ca-certificates tzdata wget

# Create non-root user
RUN addgroup -g 1001 -S appgroup && \
    adduser -u 1001 -S appuser -G appgroup

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/learn-rust /app/learn-rust

# Change ownership
RUN chown -R appuser:appgroup /app

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 8080

# Set environment variables
ENV PORT=8080 \
    HOST=0.0.0.0 \
    RUST_LOG=info

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:8080/healthz || exit 1

# Run the application
CMD ["/app/learn-rust"]

# Labels
ARG BUILD_DATE
ARG VCS_REF
ARG VERSION

LABEL org.opencontainers.image.created="${BUILD_DATE}" \
      org.opencontainers.image.authors="dxas90" \
      org.opencontainers.image.url="https://github.com/dxas90/learn-rust" \
      org.opencontainers.image.documentation="https://github.com/dxas90/learn-rust/blob/main/README.md" \
      org.opencontainers.image.source="https://github.com/dxas90/learn-rust" \
      org.opencontainers.image.version="${VERSION}" \
      org.opencontainers.image.revision="${VCS_REF}" \
      org.opencontainers.image.vendor="dxas90" \
      org.opencontainers.image.licenses="MIT" \
      org.opencontainers.image.title="learn-rust" \
      org.opencontainers.image.description="A simple Rust microservice for learning Kubernetes and Docker"
