use once_cell::sync::Lazy;
use opentelemetry::KeyValue;
use opentelemetry::trace::TraceError;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{runtime, Resource};
use opentelemetry_sdk::trace::Config;
use opentelemetry_sdk::trace::TracerProvider as SDKTracerProvider;
use opentelemetry_stdout as stdout;


static RESOURCE: Lazy<Resource> = Lazy::new(|| {
    Resource::new(vec![KeyValue::new(
        opentelemetry_semantic_conventions::resource::SERVICE_NAME,
        "tonic-server",
    )])
});

fn init_otlp_tracer_provider() -> Result<SDKTracerProvider, TraceError> {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(
                    std::env::var("OTEL_EXPORTER_OTLP_TRACES_ENDPOINT")
                        .unwrap_or("http://localhost:4317".to_string())
                ),
        )
        .with_trace_config(Config::default().with_resource(RESOURCE.clone()))
        .install_batch(runtime::Tokio)
}

fn init_sdk_tracer_provider() -> Result<SDKTracerProvider, TraceError> {
    Ok(SDKTracerProvider::builder()
        .with_simple_exporter(stdout::SpanExporter::default())
        .build())
}

pub fn get_tracer_provider() -> Result<SDKTracerProvider, TraceError> {
    let db_type = std::env::var("OTEL_EXPORTER_TRACES").unwrap_or("stdout".to_string());

    match db_type.as_str() {
        "otlp" => init_otlp_tracer_provider(),
        _ => init_sdk_tracer_provider()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracer_provider() {
        let result = get_tracer_provider();

        assert!(result.is_ok());
    }

    #[test]
    fn test_tracer_provider_unknown() {
        std::env::set_var("OTEL_EXPORTER_TRACES", "unknown");

        let result = init_sdk_tracer_provider();

        assert!(result.is_ok());
    }
}
