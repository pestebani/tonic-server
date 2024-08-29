use std::error::Error;
use opentelemetry_sdk::trace::Tracer;
use tracing_loki::BackgroundTask;
use tracing_loki::url::Url;
use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub enum LogLayer {
    Loki(tracing_loki::Layer, BackgroundTask),
    Stdout,
}

fn init_loki_log_provider() -> Result<LogLayer, Box<dyn Error>> {
    let (layer, task) = tracing_loki::layer(
        Url::parse(
            std::env::var("OTEL_EXPORTER_OTLP_LOGS_ENDPOINT")
                .unwrap_or("http://127.0.0.1:3100".to_string())
                .as_str()
        ).unwrap(),
        vec![("service".into(), "tonic-server".into())].into_iter().collect(),
        vec![].into_iter().collect(),
    )?;
    Ok(LogLayer::Loki(layer, task))
}

fn init_sdk_log_provider() -> Result<LogLayer, Box<dyn Error>> {
    Ok(LogLayer::Stdout)
}


pub fn get_logger() -> Result<LogLayer, Box<dyn Error>> {
    let log_type = std::env::var("OTEL_EXPORTER_LOGS").unwrap_or("stdout".to_string());

    match log_type.as_str() {
        "loki" => init_loki_log_provider(),
        _ => init_sdk_log_provider(),
    }
}

pub fn set_logger(log_layer: LogLayer, tracer: Tracer) -> Result<(), Box<dyn Error>> {

    match log_layer {
        LogLayer::Loki(layer, task) =>{
            let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
            tokio::spawn(task);
            tracing_subscriber::registry()
                .with(EnvFilter::from_default_env())
                .with(layer)
                // .with(fmt::layer()) // Remove this comment if you want to also print logs to stdout
                .with(telemetry)
                .init();
        },
        _ => {
            let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
            tracing_subscriber::registry().with(EnvFilter::from_default_env()).with(fmt::layer()).with(telemetry).init();
        }
    };

    Ok(())
}
