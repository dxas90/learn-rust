use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;
use sysinfo::System;
use utoipa;

use crate::models::*;

/// Root endpoint handler - Returns welcome message with API documentation
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Welcome message with API endpoints", body = ApiResponse<WelcomeData>)
    ),
    tag = "info"
)]
pub async fn index(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    let welcome = WelcomeData {
        message: "Welcome to learn-rust API".to_string(),
        description: "A simple Rust microservice for learning and demonstration".to_string(),
        documentation: Documentation {
            swagger: None,
            postman: None,
        },
        links: Links {
            repository: "https://github.com/dxas90/learn-rust".to_string(),
            issues: "https://github.com/dxas90/learn-rust/issues".to_string(),
        },
        endpoints: vec![
            Endpoint {
                path: "/".to_string(),
                method: "GET".to_string(),
                description: "API welcome and documentation".to_string(),
            },
            Endpoint {
                path: "/ping".to_string(),
                method: "GET".to_string(),
                description: "Simple ping-pong response".to_string(),
            },
            Endpoint {
                path: "/healthz".to_string(),
                method: "GET".to_string(),
                description: "Health check endpoint".to_string(),
            },
            Endpoint {
                path: "/info".to_string(),
                method: "GET".to_string(),
                description: "Application and system information".to_string(),
            },
            Endpoint {
                path: "/version".to_string(),
                method: "GET".to_string(),
                description: "Application version information".to_string(),
            },
            Endpoint {
                path: "/echo".to_string(),
                method: "POST".to_string(),
                description: "Echo back the request body".to_string(),
            },
            Endpoint {
                path: "/metrics".to_string(),
                method: "GET".to_string(),
                description: "Prometheus metrics endpoint".to_string(),
            },
            Endpoint {
                path: "/openapi.json".to_string(),
                method: "GET".to_string(),
                description: "OpenAPI specification".to_string(),
            },
        ],
    };

    Json(ApiResponse::success(welcome))
}

/// Ping endpoint - Simple health check
#[utoipa::path(
    get,
    path = "/ping",
    responses(
        (status = 200, description = "Pong response", body = String, content_type = "text/plain")
    ),
    tag = "health"
)]
pub async fn ping() -> impl IntoResponse {
    (StatusCode::OK, "pong")
}

/// Health check endpoint - Returns detailed health information
#[utoipa::path(
    get,
    path = "/healthz",
    responses(
        (status = 200, description = "Health status with system metrics", body = ApiResponse<HealthData>)
    ),
    tag = "health"
)]
pub async fn healthz(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut sys = System::new_all();
    sys.refresh_all();

    let uptime = state
        .start_time
        .elapsed()
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0);

    let total_memory = sys.total_memory();
    let available_memory = sys.available_memory();
    let used_memory = total_memory - available_memory;
    let memory_percent = if total_memory > 0 {
        (used_memory as f64 / total_memory as f64) * 100.0
    } else {
        0.0
    };

    let health = HealthData {
        status: "healthy".to_string(),
        uptime,
        memory: MemoryInfo {
            total: total_memory,
            available: available_memory,
            used: used_memory,
            percent: memory_percent,
        },
        system: SystemInfo {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            cpu_count: sys.cpus().len(),
            hostname: System::host_name().unwrap_or_else(|| "unknown".to_string()),
        },
    };

    Json(ApiResponse::success(health))
}

/// Info endpoint - Returns application and system information
#[utoipa::path(
    get,
    path = "/info",
    responses(
        (status = 200, description = "Detailed system and application info", body = ApiResponse<InfoData>)
    ),
    tag = "info"
)]
pub async fn info(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut sys = System::new_all();
    sys.refresh_all();

    let uptime = state
        .start_time
        .elapsed()
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0);

    let total_memory = sys.total_memory();
    let available_memory = sys.available_memory();
    let used_memory = total_memory - available_memory;
    let memory_percent = if total_memory > 0 {
        (used_memory as f64 / total_memory as f64) * 100.0
    } else {
        0.0
    };

    let info = InfoData {
        application: state.app_info.clone(),
        system: DetailedSystemInfo {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            hostname: System::host_name().unwrap_or_else(|| "unknown".to_string()),
            cpu_count: sys.cpus().len(),
            uptime,
            memory: MemoryInfo {
                total: total_memory,
                available: available_memory,
                used: used_memory,
                percent: memory_percent,
            },
        },
        environment: EnvironmentInfo {
            rust_version: rustc_version_runtime::version().to_string(),
            port: std::env::var("PORT").unwrap_or_else(|_| "8080".to_string()),
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
        },
    };

    Json(ApiResponse::success(info))
}

/// Version endpoint - Returns version information
#[utoipa::path(
    get,
    path = "/version",
    responses(
        (status = 200, description = "Application version information", body = ApiResponse<VersionData>)
    ),
    tag = "info"
)]
pub async fn version_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let version = VersionData {
        version: state.app_info.version.clone(),
        build_date: option_env!("BUILD_DATE").unwrap_or("unknown").to_string(),
        commit: option_env!("VCS_REF").unwrap_or("unknown").to_string(),
    };

    Json(ApiResponse::success(version))
}

/// Echo endpoint - Echoes back the request body
#[utoipa::path(
    post,
    path = "/echo",
    request_body = EchoRequest,
    responses(
        (status = 200, description = "Echoed request data", body = ApiResponse<EchoResponse>),
        (status = 400, description = "Invalid request body")
    ),
    tag = "utility"
)]
pub async fn echo(Json(payload): Json<EchoRequest>) -> impl IntoResponse {
    let response = EchoResponse {
        message: payload.message,
        received_at: chrono::Utc::now().to_rfc3339(),
    };

    Json(ApiResponse::success(response))
}
