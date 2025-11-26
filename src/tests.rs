#[cfg(test)]
mod tests {
    use crate::handlers;
    use crate::models::AppState;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::{get, post},
        Router,
    };
    use std::sync::Arc;
    use tower::ServiceExt;
    use http_body_util::BodyExt;
    use serde_json::Value;

    async fn setup_app() -> Router {
        // Initialize metrics for tests
        crate::metrics::init_metrics();
        
        let state = Arc::new(AppState::new("0.0.1".to_string(), "test".to_string()));

        Router::new()
            .route("/", get(handlers::index))
            .route("/ping", get(handlers::ping))
            .route("/healthz", get(handlers::healthz))
            .route("/info", get(handlers::info))
            .route("/version", get(handlers::version_handler))
            .route("/echo", post(handlers::echo))
            .route("/metrics", get(crate::metrics::metrics_handler))
            .with_state(state)
    }

    #[tokio::test]
    async fn test_index() {
        let app = setup_app().await;

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_ping() {
        let app = setup_app().await;

        let response = app
            .oneshot(Request::builder().uri("/ping").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"pong");
    }

    #[tokio::test]
    async fn test_healthz() {
        let app = setup_app().await;

        let response = app
            .oneshot(Request::builder().uri("/healthz").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_info() {
        let app = setup_app().await;

        let response = app
            .oneshot(Request::builder().uri("/info").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_version() {
        let app = setup_app().await;

        let response = app
            .oneshot(Request::builder().uri("/version").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_echo() {
        let app = setup_app().await;

        let request = Request::builder()
            .uri("/echo")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"message": "test"}"#))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["success"], true);
        assert_eq!(json["data"]["message"], "test");
    }

    #[tokio::test]
    async fn test_metrics() {
        let app = setup_app().await;

        let request = Request::builder()
            .uri("/metrics")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        
        // Check for prometheus metrics format
        assert!(body_str.contains("http_requests_total"));
        assert!(body_str.contains("http_request_duration_seconds"));
    }
}
