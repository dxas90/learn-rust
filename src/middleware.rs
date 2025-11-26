use axum::{extract::Request, http::header, middleware::Next, response::Response};
use std::time::Instant;

/// Security headers middleware
pub async fn security_headers(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;

    let headers = response.headers_mut();
    headers.insert(
        header::HeaderName::from_static("x-content-type-options"),
        header::HeaderValue::from_static("nosniff"),
    );
    headers.insert(
        header::HeaderName::from_static("x-frame-options"),
        header::HeaderValue::from_static("DENY"),
    );
    headers.insert(
        header::HeaderName::from_static("x-xss-protection"),
        header::HeaderValue::from_static("1; mode=block"),
    );
    headers.insert(
        header::HeaderName::from_static("referrer-policy"),
        header::HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    headers.insert(
        header::HeaderName::from_static("content-security-policy"),
        header::HeaderValue::from_static("default-src 'self'"),
    );

    response
}

/// Metrics middleware - tracks request counts and duration
pub async fn metrics_middleware(request: Request, next: Next) -> Response {
    let start = Instant::now();

    // Increment request counter
    crate::metrics::HTTP_REQUESTS_TOTAL.inc();

    let response = next.run(request).await;

    // Record request duration
    let duration = start.elapsed();
    crate::metrics::HTTP_REQUEST_DURATION_SECONDS.observe(duration.as_secs_f64());

    response
}
