use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Learn-Rust API",
        version = "0.0.1",
        description = "A simple Rust microservice for learning and demonstration"
    ),
    servers(
        (url = "http://localhost:8080", description = "Local server")
    ),
    paths(
        crate::handlers::index,
        crate::handlers::ping,
        crate::handlers::healthz,
        crate::handlers::info,
        crate::handlers::version_handler,
        crate::handlers::echo,
    ),
    components(
        schemas(
            crate::models::ApiResponse<crate::models::WelcomeData>,
            crate::models::ApiResponse<crate::models::HealthData>,
            crate::models::ApiResponse<crate::models::InfoData>,
            crate::models::ApiResponse<crate::models::VersionData>,
            crate::models::ApiResponse<crate::models::EchoResponse>,
            crate::models::WelcomeData,
            crate::models::HealthData,
            crate::models::InfoData,
            crate::models::VersionData,
            crate::models::EchoRequest,
            crate::models::EchoResponse,
            crate::models::AppInfo,
            crate::models::Documentation,
            crate::models::Links,
            crate::models::Endpoint,
            crate::models::MemoryInfo,
            crate::models::SystemInfo,
            crate::models::DetailedSystemInfo,
            crate::models::EnvironmentInfo,
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "info", description = "Information endpoints"),
        (name = "utility", description = "Utility endpoints")
    )
)]
pub struct ApiDoc;

pub fn get_openapi_json() -> String {
    ApiDoc::openapi().to_pretty_json().unwrap()
}
