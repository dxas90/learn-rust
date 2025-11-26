use axum::{http::StatusCode, response::IntoResponse};
use lazy_static::lazy_static;
use prometheus::{Counter, Encoder, Histogram, HistogramOpts, Opts, Registry, TextEncoder};
use std::sync::Once;

static INIT: Once = Once::new();

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    pub static ref HTTP_REQUESTS_TOTAL: Counter = Counter::with_opts(Opts::new(
        "http_requests_total",
        "Total number of HTTP requests"
    ))
    .expect("metric can be created");
    pub static ref HTTP_REQUEST_DURATION_SECONDS: Histogram =
        Histogram::with_opts(HistogramOpts::new(
            "http_request_duration_seconds",
            "HTTP request duration in seconds"
        ))
        .expect("metric can be created");
}

pub fn init_metrics() {
    INIT.call_once(|| {
        REGISTRY
            .register(Box::new(HTTP_REQUESTS_TOTAL.clone()))
            .expect("collector can be registered");

        REGISTRY
            .register(Box::new(HTTP_REQUEST_DURATION_SECONDS.clone()))
            .expect("collector can be registered");

        // Register process metrics
        let process_collector = prometheus::process_collector::ProcessCollector::for_self();
        REGISTRY
            .register(Box::new(process_collector))
            .expect("process collector can be registered");
    });
}
/// Metrics endpoint handler
pub async fn metrics_handler() -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.gather();
    let mut buffer = vec![];

    match encoder.encode(&metric_families, &mut buffer) {
        Ok(_) => (
            StatusCode::OK,
            [("content-type", "text/plain; version=0.0.4")],
            buffer,
        ),
        Err(e) => {
            tracing::error!("Failed to encode metrics: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                [("content-type", "text/plain; version=0.0.4")],
                format!("Failed to encode metrics: {}", e).into_bytes(),
            )
        }
    }
}
