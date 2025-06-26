//! Global tracing configuration and initialization for SwarmSH v2
//! 
//! Simplified for e2e testing compatibility

use anyhow::{Context, Result};
use opentelemetry::{global, KeyValue};
use opentelemetry_sdk::{
    propagation::TraceContextPropagator,
    trace::{self, Sampler},
    Resource,
};
use std::time::Duration;
use tracing::{Level, Subscriber};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{
    filter::EnvFilter,
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    registry::LookupSpan,
    Layer, Registry,
};
use uuid::Uuid;

/// Global tracing configuration
#[derive(Debug, Clone)]
pub struct TracingConfig {
    /// Service name for telemetry
    pub service_name: String,
    /// Service version
    pub service_version: String,
    /// Minimum log level
    pub log_level: Level,
    /// Enable OTLP export
    pub enable_otlp: bool,
    /// Enable Jaeger export
    pub enable_jaeger: bool,
    /// Enable console output
    pub enable_console: bool,
    /// Enable JSON formatting
    pub json_output: bool,
    /// Batch timeout for spans
    pub batch_timeout_ms: u64,
    /// Maximum queue size
    pub max_queue_size: usize,
    /// Sampling ratio (0.0 to 1.0)
    pub sampling_ratio: f64,
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self {
            service_name: "swarmsh-v2".to_string(),
            service_version: env!("CARGO_PKG_VERSION").to_string(),
            log_level: Level::INFO,
            enable_otlp: false, // Simplified for e2e testing
            enable_jaeger: false, // Simplified for e2e testing
            enable_console: true,
            json_output: false,
            batch_timeout_ms: 5000,
            max_queue_size: 2048,
            sampling_ratio: 1.0,
        }
    }
}

/// Initialize global tracing with comprehensive configuration
pub fn init_global_tracing(config: TracingConfig) -> Result<()> {
    // Set global error handler for OpenTelemetry
    global::set_error_handler(|error| {
        eprintln!("OpenTelemetry error occurred: {:?}", error);
    })?;

    // Set global propagator
    global::set_text_map_propagator(TraceContextPropagator::new());

    // Build the tracing subscriber with layers
    let subscriber = Registry::default()
        .with(env_filter_layer(&config))
        .with(fmt_layer(&config));

    // Add simplified OpenTelemetry layer for testing
    let subscriber = if config.enable_console {
        let otel_layer = build_simple_otel_layer(&config)?;
        subscriber.with(Some(otel_layer))
    } else {
        subscriber.with(None)
    };

    // Set as global default
    tracing::subscriber::set_global_default(subscriber)
        .context("Failed to set global tracing subscriber")?;

    Ok(())
}

/// Initialize minimal tracing for testing
pub fn init_test_tracing() -> Result<()> {
    let config = TracingConfig {
        log_level: Level::DEBUG,
        enable_console: true,
        json_output: false,
        ..Default::default()
    };
    
    init_global_tracing(config)
}

/// Create environment filter layer
fn env_filter_layer(config: &TracingConfig) -> EnvFilter {
    EnvFilter::builder()
        .with_default_directive(config.log_level.into())
        .from_env_lossy()
}

/// Create formatting layer
fn fmt_layer<S>(config: &TracingConfig) -> Box<dyn Layer<S> + Send + Sync>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE);

    if config.json_output {
        Box::new(layer.json())
    } else {
        Box::new(layer.compact())
    }
}

/// Build simplified OpenTelemetry layer for testing
fn build_simple_otel_layer<S>(config: &TracingConfig) -> Result<OpenTelemetryLayer<S, opentelemetry_sdk::trace::Tracer>>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    let resource = Resource::new(vec![
        KeyValue::new("service.name", config.service_name.clone()),
        KeyValue::new("service.version", config.service_version.clone()),
    ]);

    // Use stdout exporter for testing
    #[cfg(feature = "stdout")]
    let exporter = opentelemetry_stdout::SpanExporter::default();
    
    #[cfg(not(feature = "stdout"))]
    let exporter = opentelemetry_sdk::export::trace::NoopSpanExporter::new();

    let tracer_provider = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_simple_exporter(exporter)
        .with_config(
            opentelemetry_sdk::trace::Config::default()
                .with_resource(resource)
        )
        .build();

    let tracer = tracer_provider.tracer("swarmsh-v2-tracer");

    // Set as global provider
    global::set_tracer_provider(tracer_provider);

    Ok(tracing_opentelemetry::layer().with_tracer(tracer))
}

/// Generate correlation ID for request tracing
pub fn generate_correlation_id() -> String {
    Uuid::new_v4().to_string()
}

/// Create span with correlation context
#[macro_export]
macro_rules! traced_span {
    ($level:expr, $name:expr, $($field:tt)*) => {
        {
            let correlation_id = $crate::tracing_config::generate_correlation_id();
            tracing::span!($level, $name, correlation_id = %correlation_id, $($field)*)
        }
    };
}

/// Initialize tracing for specific context
pub fn init_context_tracing(context: &str) -> Result<()> {
    let config = TracingConfig {
        service_name: format!("swarmsh-v2-{}", context),
        log_level: Level::DEBUG,
        enable_console: true,
        ..Default::default()
    };
    
    init_global_tracing(config)
}

/// Shutdown global tracing
pub fn shutdown_global_tracing() {
    global::shutdown_tracer_provider();
}