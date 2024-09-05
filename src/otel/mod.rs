mod tracer;
mod logger;

use std::error::Error;
use opentelemetry::trace::TracerProvider;
use crate::otel::logger::{get_logger, set_logger};
use crate::otel::tracer::get_tracer_provider;

pub fn init_tracer_and_logger() -> Result<(), Box<dyn Error>> {
    let exporter = get_tracer_provider()?;
    
    let log_layer = get_logger()?;

    let tracer = exporter.tracer("tonic-server");

    set_logger(log_layer, tracer)?;

    Ok(())
}

pub fn stop_tracer_and_logger() {
    opentelemetry::global::shutdown_tracer_provider();
}


#[cfg(test)]
mod tests {
    use tracing::{debug, error, info, span, warn, Level};
    use super::*;

    #[test]
    fn test_tracer_and_logger() {
        let result = init_tracer_and_logger();

        assert!(result.is_ok());
        let my_span = span!(Level::DEBUG, "my_span_debug").entered();

        debug!("test_debug");
        info!("test_info");
        warn!("test_warn");
        error!("test_error");

        my_span.exit();

        stop_tracer_and_logger();
    }
}
