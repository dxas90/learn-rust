use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;
mod metrics;
mod middleware;
mod models;
mod openapi;
mod telemetry;

#[cfg(test)]
mod tests;

use handlers::*;
use models::AppState;

/// OpenAPI specification handler
async fn openapi_handler() -> impl axum::response::IntoResponse {
    use axum::http::StatusCode;
    (
        StatusCode::OK,
        [("content-type", "application/json")],
        openapi::get_openapi_json(),
    )
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "learn_rust=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize OpenTelemetry tracer
    if let Err(e) = telemetry::init_tracer() {
        tracing::warn!("[WARN] Failed to initialize OpenTelemetry tracer: {}", e);
    }

    // Initialize Prometheus metrics
    metrics::init_metrics();
    info!("[INFO] Prometheus metrics initialized");

    // Get configuration from environment
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let version = std::env::var("APP_VERSION").unwrap_or_else(|_| "0.0.1".to_string());
    let environment = std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string());

    // Create application state
    let state = Arc::new(AppState::new(version, environment));

    // Build CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build application routes
    let app = Router::new()
        .route("/", get(index))
        .route("/ping", get(ping))
        .route("/healthz", get(healthz))
        .route("/info", get(info))
        .route("/version", get(version_handler))
        .route("/echo", post(echo))
        .route("/metrics", get(metrics::metrics_handler))
        .route("/openapi.json", get(openapi_handler))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .layer(axum::middleware::from_fn(middleware::security_headers))
        .layer(axum::middleware::from_fn(middleware::metrics_middleware))
        .with_state(state);

    // Build address
    let addr = format!("{}:{}", host, port);
    info!("🚀 Server starting at http://{}/", addr);
    info!(
        "📊 Environment: {}",
        std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
    );
    info!(
        "📦 Version: {}",
        std::env::var("APP_VERSION").unwrap_or_else(|_| "0.0.1".to_string())
    );
    info!("🕐 Started at: {}", chrono::Utc::now().to_rfc3339());

    // Create listener
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| panic!("Failed to bind to {}: {}", addr, e));

    info!("Listening on {}", addr);

    // Start server with graceful shutdown
    let server = axum::serve(listener, app);

    // Handle graceful shutdown
    let shutdown_signal = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C signal handler");
        info!("Shutting down gracefully...");
        telemetry::shutdown_tracer();
    };

    tokio::select! {
        result = server => {
            if let Err(e) = result {
                tracing::error!("Server error: {}", e);
            }
        }
        _ = shutdown_signal => {
            info!("Shutdown signal received");
        }
    }
}
