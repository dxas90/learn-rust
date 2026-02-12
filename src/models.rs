use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use utoipa::ToSchema;

/// Application state shared across handlers
pub struct AppState {
    pub app_info: AppInfo,
    pub start_time: SystemTime,
}

impl AppState {
    pub fn new(version: String, environment: String) -> Self {
        Self {
            app_info: AppInfo {
                name: "learn-rust".to_string(),
                version,
                environment,
                timestamp: Utc::now().to_rfc3339(),
            },
            start_time: SystemTime::now(),
        }
    }
}

/// Generic API response wrapper
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub timestamp: String,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: Utc::now().to_rfc3339(),
        }
    }

    #[allow(dead_code)]
    pub fn error(message: String) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            error: Some(message),
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}

/// Application information
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub environment: String,
    pub timestamp: String,
}

/// Welcome page data
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WelcomeData {
    pub message: String,
    pub description: String,
    pub documentation: Documentation,
    pub links: Links,
    pub endpoints: Vec<Endpoint>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Documentation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swagger: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postman: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Links {
    pub repository: String,
    pub issues: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Endpoint {
    pub path: String,
    pub method: String,
    pub description: String,
}

/// Health check data
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthData {
    pub status: String,
    pub uptime: f64,
    pub memory: MemoryInfo,
    pub system: SystemInfo,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MemoryInfo {
    pub total: u64,
    pub available: u64,
    pub used: u64,
    pub percent: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub cpu_count: usize,
    pub hostname: String,
}

/// System information data
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct InfoData {
    pub application: AppInfo,
    pub system: DetailedSystemInfo,
    pub environment: EnvironmentInfo,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DetailedSystemInfo {
    pub os: String,
    pub arch: String,
    pub hostname: String,
    pub cpu_count: usize,
    pub uptime: f64,
    pub memory: MemoryInfo,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EnvironmentInfo {
    pub rust_version: String,
    pub port: String,
    pub host: String,
}

/// Version information
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct VersionData {
    pub version: String,
    pub build_date: String,
    pub commit: String,
}

/// Echo request/response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EchoRequest {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EchoResponse {
    pub message: String,
    pub received_at: String,
}
