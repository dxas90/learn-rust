#[cfg(feature = "telemetry")]
use opentelemetry::global;
#[cfg(feature = "telemetry")]
use opentelemetry_otlp::WithExportConfig;
#[cfg(feature = "telemetry")]
use opentelemetry_sdk::Resource;

#[cfg(feature = "telemetry")]
static TRACER_PROVIDER: std::sync::OnceLock<opentelemetry_sdk::trace::SdkTracerProvider> =
    std::sync::OnceLock::new();

#[cfg(feature = "telemetry")]
pub fn init_tracer() -> Result<(), Box<dyn std::error::Error>> {
    let otlp_endpoint =
        std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT").unwrap_or_else(|_| String::new());

    if otlp_endpoint.is_empty() {
        tracing::info!("[INFO] OpenTelemetry: OTEL_EXPORTER_OTLP_ENDPOINT not set, skipping OTLP configuration");
        return Ok(());
    }

    tracing::info!(
        "[INFO] OpenTelemetry: Configuring OTLP exporter with endpoint: {}",
        otlp_endpoint
    );

    use opentelemetry_otlp::SpanExporter;
    use opentelemetry_sdk::trace::SdkTracerProvider;

    let exporter = SpanExporter::builder()
        .with_tonic()
        .with_endpoint(otlp_endpoint)
        .build()?;

    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(Resource::builder().build())
        .build();

    global::set_tracer_provider(provider.clone());
    TRACER_PROVIDER.set(provider).ok();

    tracing::info!("[INFO] OpenTelemetry: Tracer initialized successfully");
    Ok(())
}

#[cfg(not(feature = "telemetry"))]
pub fn init_tracer() -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("[INFO] OpenTelemetry: Telemetry feature not enabled");
    Ok(())
}

pub fn shutdown_tracer() {
    #[cfg(feature = "telemetry")]
    {
        if let Some(provider) = TRACER_PROVIDER.get() {
            if let Err(e) = provider.shutdown() {
                tracing::warn!("[WARN] OpenTelemetry: Error shutting down tracer: {}", e);
            }
        }
    }
}
