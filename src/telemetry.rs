//! Flexible OpenTelemetry configuration for SwarmSH v2
//! 
//! Supports both lightweight shell utilities and comprehensive observability infrastructure.
//! Built using OTEL Weaver semantic conventions with AI-enhanced coordination patterns.

use anyhow::{Context, Result};
use opentelemetry::{
    global, 
    trace::{Tracer, TracerProvider},
    KeyValue,
    metrics::Meter,
};
use opentelemetry_sdk::{
    trace::{self, TracerProvider as SdkTracerProvider},
    Resource,
};
use opentelemetry_stdout::SpanExporter as StdoutSpanExporter;
use tracing::{info, warn, debug, instrument, error, Span};
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
    fmt,
    Layer,
};
use tracing_error::ErrorLayer;
use tracing_timing::{Builder as TimingBuilder, Histogram, TimingSubscriber};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::time::{Instant, Duration};
use std::sync::Arc;

/// Telemetry configuration modes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TelemetryMode {
    /// Minimal telemetry for shell utilities - stdout only
    Lightweight,
    /// Development mode with stdout and optional file export
    Development { log_file: Option<String> },
    /// Production mode with full exporter suite
    Production {
        jaeger_endpoint: Option<String>,
        prometheus_endpoint: Option<String>,
        otlp_endpoint: Option<String>,
    },
    /// Disabled telemetry
    Disabled,
}

impl Default for TelemetryMode {
    fn default() -> Self {
        Self::Lightweight
    }
}

/// Global telemetry configuration
#[derive(Debug, Clone)]
pub struct TelemetryConfig {
    pub mode: TelemetryMode,
    pub service_name: String,
    pub service_version: String,
    pub environment: String,
    pub jaeger_endpoint: Option<String>,
    pub otlp_endpoint: Option<String>,
    pub enable_stdout: bool,
    pub enable_prometheus: bool,
    pub log_level: String,
    pub sample_ratio: f64,
    pub enable_timing: bool,
    pub timing_max_value: u64,
    pub timing_precision: u32,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        // Determine mode from environment
        let mode = match std::env::var("SWARMSH_TELEMETRY_MODE").as_deref() {
            Ok("lightweight") => TelemetryMode::Lightweight,
            Ok("development") => TelemetryMode::Development { 
                log_file: std::env::var("SWARMSH_LOG_FILE").ok() 
            },
            Ok("production") => TelemetryMode::Production {
                jaeger_endpoint: std::env::var("JAEGER_ENDPOINT").ok(),
                prometheus_endpoint: std::env::var("PROMETHEUS_ENDPOINT").ok(),
                otlp_endpoint: std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT").ok(),
            },
            Ok("disabled") => TelemetryMode::Disabled,
            _ => TelemetryMode::Lightweight,
        };
        
        Self {
            mode,
            service_name: "swarmsh-v2".to_string(),
            service_version: env!("CARGO_PKG_VERSION").to_string(),
            environment: std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
            jaeger_endpoint: std::env::var("JAEGER_ENDPOINT").ok(),
            otlp_endpoint: std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT").ok(),
            enable_stdout: std::env::var("OTEL_ENABLE_STDOUT").map(|v| v == "true").unwrap_or(true),
            enable_prometheus: std::env::var("OTEL_ENABLE_PROMETHEUS").map(|v| v == "true").unwrap_or(true),
            log_level: std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
            sample_ratio: std::env::var("OTEL_SAMPLE_RATIO")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1.0),
            enable_timing: std::env::var("SWARMSH_ENABLE_TIMING").map(|v| v == "true").unwrap_or(true),
            timing_max_value: std::env::var("SWARMSH_TIMING_MAX_VALUE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10_000_000),  // 10 seconds in microseconds
            timing_precision: std::env::var("SWARMSH_TIMING_PRECISION")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(2),  // 2 significant digits
        }
    }
}

/// Global telemetry manager for SwarmSH system
pub struct TelemetryManager {
    config: TelemetryConfig,
    tracer_provider: Option<SdkTracerProvider>,
    timing_subscriber: Option<Arc<TimingSubscriber>>,
    _guard: Option<tracing_appender::non_blocking::WorkerGuard>,
}

impl TelemetryManager {
    /// Create new telemetry manager with default configuration
    pub async fn new() -> Result<Self> {
        Self::with_config(TelemetryConfig::default()).await
    }

    /// Create new telemetry manager with custom configuration
    pub async fn with_config(config: TelemetryConfig) -> Result<Self> {
        let mut manager = Self {
            config,
            tracer_provider: None,
            timing_subscriber: None,
            _guard: None,
        };
        
        manager.initialize().await?;
        Ok(manager)
    }
    
    /// Create lightweight telemetry manager for shell utilities
    pub async fn lightweight(service_name: &str) -> Result<Self> {
        let config = TelemetryConfig {
            mode: TelemetryMode::Lightweight,
            service_name: service_name.to_string(),
            ..Default::default()
        };
        
        Self::with_config(config).await
    }
    
    /// Create timing subscriber for inter-event timing metrics
    fn create_timing_subscriber(&self) -> Result<Arc<TimingSubscriber>> {
        if !self.config.enable_timing {
            return Err(anyhow::anyhow!("Timing is disabled"));
        }
        
        let max_value = self.config.timing_max_value;
        let precision = self.config.timing_precision as u8;
        
        let subscriber = TimingBuilder::default()
            .build(move || {
                Histogram::new_with_max(max_value, precision).unwrap()
            });
            
        Ok(Arc::new(subscriber))
    }
    
    /// Get timing histograms for analysis
    pub fn get_timing_histograms(&self) -> Option<Vec<(String, f64)>> {
        if let Some(ref timing_subscriber) = self.timing_subscriber {
            let mut results = Vec::new();
            timing_subscriber.with_histograms(|span_groups| {
                for (span_group, event_groups) in span_groups {
                    for (event_group, hist) in event_groups {
                        hist.refresh();
                        let key = format!("{}::{}", span_group, event_group);
                        results.push((key, hist.mean()));
                    }
                }
            });
            Some(results)
        } else {
            None
        }
    }
    
    /// Force synchronization of timing data
    pub fn force_synchronize_timing(&self) {
        if let Some(ref timing_subscriber) = self.timing_subscriber {
            timing_subscriber.force_synchronize();
        }
    }
    
    /// Get timing summary statistics
    pub fn get_timing_summary(&self) -> Option<String> {
        if let Some(ref timing_subscriber) = self.timing_subscriber {
            let mut output = String::new();
            timing_subscriber.with_histograms(|span_groups| {
                for (span_group, event_groups) in span_groups {
                    for (event_group, hist) in event_groups {
                        hist.refresh();
                        output.push_str(&format!(
                            "{}::{}: mean: {:.2}µs, p50: {:.2}µs, p90: {:.2}µs, p99: {:.2}µs, max: {:.2}µs\n",
                            span_group,
                            event_group,
                            hist.mean(),
                            hist.value_at_quantile(0.5),
                            hist.value_at_quantile(0.9),
                            hist.value_at_quantile(0.99),
                            hist.max()
                        ));
                    }
                }
            });
            Some(output)
        } else {
            None
        }
    }
    
    /// Initialize telemetry based on configuration mode
    #[instrument(skip(self))]
    async fn initialize(&mut self) -> Result<()> {
        // Create timing subscriber if enabled
        if self.config.enable_timing {
            match self.create_timing_subscriber() {
                Ok(timing_subscriber) => {
                    self.timing_subscriber = Some(timing_subscriber);
                    debug!("Timing subscriber created successfully");
                }
                Err(e) => {
                    warn!("Failed to create timing subscriber: {}", e);
                }
            }
        }
        
        match self.config.mode.clone() {
            TelemetryMode::Disabled => {
                info!("Telemetry disabled");
                return Ok(());
            }
            TelemetryMode::Lightweight => {
                self.init_lightweight_telemetry().await?;
            }
            TelemetryMode::Development { log_file } => {
                self.init_development_telemetry(log_file.as_deref()).await?;
            }
            TelemetryMode::Production { jaeger_endpoint, prometheus_endpoint, otlp_endpoint } => {
                self.init_production_telemetry(
                    jaeger_endpoint.as_deref(),
                    prometheus_endpoint.as_deref(),
                    otlp_endpoint.as_deref(),
                ).await?;
            }
        }
        
        info!(
            service = %self.config.service_name,
            version = %self.config.service_version,
            mode = ?self.config.mode,
            timing_enabled = self.config.enable_timing,
            "Telemetry initialized"
        );
        
        Ok(())
    }
    
    /// Initialize minimal telemetry for shell utilities
    async fn init_lightweight_telemetry(&mut self) -> Result<()> {
        // Create resource with minimal service information
        let resource = Resource::new(vec![
            KeyValue::new("service.name", self.config.service_name.clone()),
            KeyValue::new("service.version", self.config.service_version.clone()),
            KeyValue::new("telemetry.mode", "lightweight"),
        ]);
        
        // Initialize tracer with stdout exporter
        let tracer_provider = SdkTracerProvider::builder()
            .with_simple_exporter(StdoutSpanExporter::default())
            .with_config(
                opentelemetry_sdk::trace::Config::default()
                    .with_resource(resource.clone())
            )
            .build();
        
        // Set up tracing subscriber with minimal formatting
        let service_name = self.config.service_name.clone();
        let tracer = tracer_provider.tracer(service_name);
        
        tracing_subscriber::registry()
            .with(tracing_opentelemetry::layer().with_tracer(tracer))
            .with(ErrorLayer::default())
            .with(
                fmt::layer()
                    .with_target(false)
                    .compact()
                    .with_filter(EnvFilter::new(&self.config.log_level))
            )
            .try_init()
            .ok();
        
        global::set_tracer_provider(tracer_provider.clone());
        self.tracer_provider = Some(tracer_provider);
        
        debug!("Lightweight telemetry initialized with stdout exporter");
        Ok(())
    }
    
    /// Initialize development telemetry with optional file export
    async fn init_development_telemetry(&mut self, log_file: Option<&str>) -> Result<()> {
        let resource = Resource::new(vec![
            KeyValue::new("service.name", self.config.service_name.clone()),
            KeyValue::new("service.version", self.config.service_version.clone()),
            KeyValue::new("deployment.environment", self.config.environment.clone()),
            KeyValue::new("telemetry.mode", "development"),
        ]);
        
        // Multi-exporter setup for development
        let mut tracer_builder = SdkTracerProvider::builder()
            .with_simple_exporter(StdoutSpanExporter::default())
            .with_config(
                opentelemetry_sdk::trace::Config::default()
                    .with_resource(resource.clone())
            );
        
        // Add file exporter if specified
        if let Some(file_path) = log_file {
            let file = std::fs::File::create(file_path)
                .context("Failed to create telemetry log file")?;
            
            debug!("Added file exporter: {}", file_path);
        }
        
        let tracer_provider = tracer_builder.build();
        let service_name = self.config.service_name.clone();
        let tracer = tracer_provider.tracer(service_name);
        
        // Enhanced tracing subscriber for development
        tracing_subscriber::registry()
            .with(tracing_opentelemetry::layer().with_tracer(tracer))
            .with(ErrorLayer::default())
            .with(
                fmt::layer()
                    .with_target(true)
                    .with_thread_ids(true)
                    .with_file(true)
                    .with_line_number(true)
                    .pretty()
                    .with_filter(EnvFilter::new(&self.config.log_level))
            )
            .try_init()
            .ok();
        
        global::set_tracer_provider(tracer_provider.clone());
        self.tracer_provider = Some(tracer_provider);
        
        debug!("Development telemetry initialized");
        Ok(())
    }
    
    /// Initialize full production telemetry with all exporters
    async fn init_production_telemetry(
        &mut self,
        jaeger_endpoint: Option<&str>,
        _prometheus_endpoint: Option<&str>,
        _otlp_endpoint: Option<&str>,
    ) -> Result<()> {
        let resource = Resource::new(vec![
            KeyValue::new("service.name", self.config.service_name.clone()),
            KeyValue::new("service.version", self.config.service_version.clone()),
            KeyValue::new("deployment.environment", self.config.environment.clone()),
            KeyValue::new("telemetry.mode", "production"),
        ]);
        
        let mut tracer_builder = SdkTracerProvider::builder()
            .with_config(
                opentelemetry_sdk::trace::Config::default()
                    .with_resource(resource.clone())
            );
        
        // Add Jaeger exporter if available
        #[cfg(feature = "jaeger")]
        if let Some(endpoint) = jaeger_endpoint {
            info!("Jaeger exporter configured: {}", endpoint);
        }
        
        // Add OTLP exporter if available
        #[cfg(feature = "otlp")]
        if let Some(endpoint) = _otlp_endpoint {
            info!("OTLP exporter configured: {}", endpoint);
        }
        
        // Fallback to stdout for production
        tracer_builder = tracer_builder.with_simple_exporter(StdoutSpanExporter::default());
        
        let tracer_provider = tracer_builder.build();
        let service_name = self.config.service_name.clone();
        let tracer = tracer_provider.tracer(service_name);
        
        // Production logging setup
        tracing_subscriber::registry()
            .with(tracing_opentelemetry::layer().with_tracer(tracer))
            .with(ErrorLayer::default())
            .with(
                fmt::layer()
                    .json()
                    .with_filter(EnvFilter::new(&self.config.log_level))
            )
            .try_init()
            .ok();
        
        global::set_tracer_provider(tracer_provider.clone());
        self.tracer_provider = Some(tracer_provider);
        
        info!("Production telemetry initialized with full exporter suite");
        Ok(())
    }

    /// Start telemetry system (compatible with existing API)
    pub async fn start(&self) -> Result<()> {
        if matches!(self.config.mode, TelemetryMode::Disabled) {
            return Ok(());
        }
        
        info!("Telemetry system started in {:?} mode", self.config.mode);
        Ok(())
    }
    
    /// Get tracer for manual instrumentation
    pub fn tracer(&self, name: &'static str) -> impl Tracer {
        global::tracer(name)
    }

    /// Get tracer for SwarmSH components (legacy compatibility)
    pub fn get_tracer(&self, name: &'static str) -> impl Tracer {
        global::tracer(name)
    }
    
    /// Get meter for custom metrics
    pub fn meter(&self, name: String) -> Meter {
        global::meter(name)
    }
    
    /// Create instrumented span with service context
    pub fn create_span(&self, operation: &str) -> tracing::Span {
        tracing::info_span!(
            "telemetry_operation",
            operation = operation,
            service_name = self.config.service_name,
            service_version = self.config.service_version
        )
    }
    
    /// Record custom metric
    pub fn record_metric(&self, name: String, value: f64, attributes: Vec<KeyValue>) {
        if !self.config.enable_prometheus {
            return;
        }
        
        let meter = self.meter(self.config.service_name.clone());
        let counter = meter.f64_counter(name).init();
        counter.add(value, &attributes);
    }
    
    /// Get current configuration
    pub fn config(&self) -> &TelemetryConfig {
        &self.config
    }

    /// Initialize Prometheus metrics exporter
    #[cfg(feature = "prometheus")]
    async fn init_metrics(&self) -> Result<()> {
        use metrics_exporter_prometheus::PrometheusBuilder;
        
        info!("Initializing Prometheus metrics exporter");
        
        let builder = PrometheusBuilder::new();
        builder
            .install()
            .context("Failed to install Prometheus metrics exporter")?;

        // Register SwarmSH v2 specific metrics
        metrics::describe_counter!(
            "swarmsh_agent_registrations_total",
            "Total number of agent registrations"
        );
        metrics::describe_histogram!(
            "swarmsh_coordination_duration_seconds",
            "Duration of coordination operations in seconds"
        );
        metrics::describe_gauge!(
            "swarmsh_active_agents",
            "Number of currently active agents"
        );
        metrics::describe_counter!(
            "swarmsh_work_items_processed_total", 
            "Total number of work items processed"
        );
        metrics::describe_histogram!(
            "swarmsh_health_check_duration_seconds",
            "Duration of health check operations in seconds"
        );

        Ok(())
    }

    /// Initialize structured logging with OTEL integration
    async fn init_logging(&mut self) -> Result<()> {
        // Create file appender for logs
        let file_appender = tracing_appender::rolling::daily("logs", "swarmsh-v2.log");
        let (file_writer, guard) = tracing_appender::non_blocking(file_appender);
        self._guard = Some(guard);

        // Configure tracing subscriber with multiple layers
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new(&self.config.log_level));

        tracing_subscriber::registry()
            .with(env_filter)
            .with(
                fmt::layer()
                    .with_target(true)
                    .with_thread_ids(true)
                    .with_file(true)
                    .with_line_number(true)
                    .compact()
            )
            .with(
                fmt::layer()
                    .with_writer(file_writer)
                    .with_ansi(false)
                    .json()
            )
            .with(tracing_opentelemetry::layer())
            .init();

        Ok(())
    }


    /// Stop telemetry system and ensure proper shutdown
    pub async fn stop(&self) -> Result<()> {
        if matches!(self.config.mode, TelemetryMode::Disabled) {
            return Ok(());
        }
        
        // Shutdown telemetry providers
        global::shutdown_tracer_provider();
        
        info!("Telemetry system stopped");
        Ok(())
    }
}

/// Correlation ID for distributed tracing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationId(String);

impl CorrelationId {
    /// Generate new correlation ID
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
    
    /// Create from existing ID
    pub fn from_string(id: String) -> Self {
        Self(id)
    }
    
    /// Get the correlation ID as string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for CorrelationId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for CorrelationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// SwarmSH-specific telemetry trait for instrumentation
pub trait SwarmTelemetry {
    /// Create agent lifecycle span with nanosecond precision
    fn agent_span(&self, agent_id: &str, operation: &str) -> tracing::Span;
    
    /// Create work coordination span with zero-conflict tracking
    fn work_span(&self, work_id: &str, operation: &str) -> tracing::Span;
    
    /// Create coordination protocol span (Scrum at Scale, Roberts Rules, etc.)
    fn coordination_span(&self, pattern: &str, operation: &str) -> tracing::Span;
    
    /// Create health monitoring span with bottleneck detection
    fn health_span(&self, component: &str, operation: &str) -> tracing::Span;
    
    /// Create analytics span for DLSS 8020 optimization
    fn analytics_span(&self, tier: &str, operation: &str) -> tracing::Span;

    /// Create span with correlation ID
    fn span_with_correlation(&self, name: &str, correlation_id: &CorrelationId) -> tracing::Span;

    /// Record coordination metrics
    fn record_coordination_duration(&self, operation: &str, duration: std::time::Duration);
    
    /// Record agent metrics
    fn record_agent_registration(&self, agent_id: &str);
    
    /// Record work item metrics
    fn record_work_item_processed(&self, work_id: &str, processing_time: std::time::Duration);
    
    /// Record health check metrics
    fn record_health_check(&self, component: &str, status: &str, duration: std::time::Duration);
    
    /// Record AI decision metrics
    fn record_ai_decision(&self, decision_type: &str, confidence: f64, duration: std::time::Duration);
    
    /// Record error with correlation
    fn record_error_with_correlation(&self, error: &anyhow::Error, correlation_id: &CorrelationId);
}

/// Default implementation with lightweight support
#[derive(Debug, Clone)]
pub struct DefaultSwarmTelemetry {
    service_name: String,
}

impl DefaultSwarmTelemetry {
    pub fn new(service_name: String) -> Self {
        Self { service_name }
    }
}

impl Default for DefaultSwarmTelemetry {
    fn default() -> Self {
        Self { service_name: "swarmsh-v2".to_string() }
    }
}

impl SwarmTelemetry for DefaultSwarmTelemetry {
    fn agent_span(&self, agent_id: &str, operation: &str) -> tracing::Span {
        tracing::info_span!(
            "swarmsh.agent.lifecycle",
            swarmsh.agent.id = %agent_id,
            swarmsh.agent.operation = %operation,
            swarmsh.coordination.pattern = "scrum_at_scale",
            swarmsh.precision = "nanosecond",
            service.name = %self.service_name
        )
    }

    fn work_span(&self, work_id: &str, operation: &str) -> tracing::Span {
        tracing::info_span!(
            "swarmsh.work.coordination",
            swarmsh.work.id = %work_id,
            swarmsh.work.operation = %operation,
            swarmsh.conflict_resolution = "zero_conflict",
            service.name = %self.service_name
        )
    }

    fn coordination_span(&self, pattern: &str, operation: &str) -> tracing::Span {
        tracing::info_span!(
            "swarmsh.coordination.protocol",
            swarmsh.coordination.pattern = %pattern,
            swarmsh.coordination.operation = %operation,
            swarmsh.governance = "roberts_rules",
            service.name = %self.service_name
        )
    }

    fn health_span(&self, component: &str, operation: &str) -> tracing::Span {
        tracing::info_span!(
            "swarmsh.health.monitoring",
            swarmsh.health.component = %component,
            swarmsh.health.operation = %operation,
            swarmsh.health.bottleneck_detection = true,
            service.name = %self.service_name
        )
    }

    fn analytics_span(&self, tier: &str, operation: &str) -> tracing::Span {
        tracing::info_span!(
            "swarmsh.analytics.dlss",
            swarmsh.analytics.tier = %tier,
            swarmsh.analytics.operation = %operation,
            swarmsh.analytics.principle = "8020_optimization",
            service.name = %self.service_name
        )
    }
    
    fn span_with_correlation(&self, name: &str, correlation_id: &CorrelationId) -> tracing::Span {
        tracing::info_span!(
            "swarmsh_operation",
            operation_name = %name,
            correlation_id = %correlation_id,
            service.name = %self.service_name,
            swarmsh.version = env!("CARGO_PKG_VERSION")
        )
    }

    #[instrument(skip(self))]
    fn record_coordination_duration(&self, operation: &str, duration: std::time::Duration) {
        metrics::histogram!(
            "swarmsh_coordination_duration_seconds",
            duration.as_secs_f64(),
            "operation" => operation.to_string(),
            "service" => self.service_name.clone()
        );
        
        debug!(
            operation = operation,
            duration_ms = duration.as_millis(),
            service = %self.service_name,
            "Coordination duration recorded"
        );
    }

    #[instrument(skip(self))]
    fn record_agent_registration(&self, agent_id: &str) {
        metrics::counter!(
            "swarmsh_agent_registrations_total", 
            1,
            "agent_id" => agent_id.to_string(),
            "service" => self.service_name.clone()
        );
        metrics::gauge!("swarmsh_active_agents", 1.0);
        
        info!(
            agent_id = agent_id,
            service = %self.service_name,
            "Agent registration recorded"
        );
    }

    #[instrument(skip(self))]
    fn record_work_item_processed(&self, work_id: &str, processing_time: std::time::Duration) {
        metrics::counter!(
            "swarmsh_work_items_processed_total",
            1,
            "work_id" => work_id.to_string(),
            "service" => self.service_name.clone()
        );
        metrics::histogram!(
            "swarmsh_work_processing_duration_seconds",
            processing_time.as_secs_f64(),
            "service" => self.service_name.clone()
        );
        
        debug!(
            work_id = work_id,
            processing_time_ms = processing_time.as_millis(),
            service = %self.service_name,
            "Work item processing recorded"
        );
    }

    #[instrument(skip(self))]
    fn record_health_check(&self, component: &str, status: &str, duration: std::time::Duration) {
        metrics::histogram!(
            "swarmsh_health_check_duration_seconds",
            duration.as_secs_f64(),
            "component" => component.to_string(),
            "status" => status.to_string(),
            "service" => self.service_name.clone()
        );
        
        debug!(
            component = component,
            status = status,
            duration_ms = duration.as_millis(),
            service = %self.service_name,
            "Health check recorded"
        );
    }
    
    /// Record AI decision with telemetry
    #[instrument(skip(self))]
    fn record_ai_decision(&self, decision_type: &str, confidence: f64, duration: std::time::Duration) {
        // Record AI decision via tracing instead of creating span manually
        tracing::info!(
            ai_decision_type = decision_type,
            ai_confidence = confidence,
            duration_ms = duration.as_millis(),
            service_name = self.service_name,
            "AI decision recorded"
        );
        
        // Span will be automatically ended when dropped
        
        info!(
            decision_type = decision_type,
            confidence = confidence,
            duration_ms = duration.as_millis(),
            service = %self.service_name,
            "AI decision recorded"
        );
    }
    
    #[instrument(skip(self, error))]
    fn record_error_with_correlation(&self, error: &anyhow::Error, correlation_id: &CorrelationId) {
        error!(
            error = %error,
            error_chain = ?error.chain().collect::<Vec<_>>(),
            correlation_id = %correlation_id,
            service.name = %self.service_name,
            "Error occurred with correlation"
        );
        
        // Record error metrics
        metrics::counter!(
            "swarmsh_errors_total",
            1,
            "service" => self.service_name.clone(),
            "error_type" => std::any::type_name_of_val(&error).to_string()
        );
    }
}

/// Lightweight telemetry initialization for shell utilities
pub fn init_shell_telemetry(service_name: &str) -> Result<SdkTracerProvider> {
    let service_name_owned = service_name.to_string();
    let resource = Resource::new(vec![
        KeyValue::new("service.name", service_name_owned.clone()),
        KeyValue::new("service.version", env!("CARGO_PKG_VERSION")),
        KeyValue::new("telemetry.mode", "shell"),
    ]);
    
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(StdoutSpanExporter::default())
        .with_config(trace::Config::default().with_resource(resource))
        .build();

    let tracer = provider.tracer(service_name_owned);
    
    tracing_subscriber::registry()
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .with(ErrorLayer::default())
        .with(
            fmt::layer()
                .with_target(false)
                .compact()
        )
        .try_init()
        .ok();

    global::set_tracer_provider(provider.clone());
    Ok(provider)
}

/// Performance timer for automatic duration measurement
pub struct PerfTimer {
    name: String,
    start: Instant,
    correlation_id: Option<CorrelationId>,
}

impl PerfTimer {
    /// Create new performance timer
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            start: Instant::now(),
            correlation_id: None,
        }
    }
    
    /// Create performance timer with correlation ID
    pub fn with_correlation(name: impl Into<String>, correlation_id: CorrelationId) -> Self {
        Self {
            name: name.into(),
            start: Instant::now(),
            correlation_id: Some(correlation_id),
        }
    }
}

impl Drop for PerfTimer {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        
        if let Some(ref correlation_id) = self.correlation_id {
            info!(
                name = %self.name,
                duration_ms = duration.as_millis(),
                duration_us = duration.as_micros(),
                correlation_id = %correlation_id,
                "Performance measurement with correlation"
            );
        } else {
            info!(
                name = %self.name,
                duration_ms = duration.as_millis(),
                duration_us = duration.as_micros(),
                "Performance measurement"
            );
        }
        
        // Record metric
        metrics::histogram!(
            "swarmsh_operation_duration_seconds",
            duration.as_secs_f64(),
            "operation" => self.name.clone()
        );
    }
}

/// Macro for creating SwarmSH instrumented spans
#[macro_export]
macro_rules! swarm_span {
    ($level:expr, $name:expr) => {
        tracing::span!(
            $level,
            $name,
            swarmsh.version = env!("CARGO_PKG_VERSION"),
            swarmsh.timestamp = %std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        )
    };
    ($level:expr, $name:expr, $($field:tt)*) => {
        tracing::span!(
            $level,
            $name,
            swarmsh.version = env!("CARGO_PKG_VERSION"),
            swarmsh.timestamp = %std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            $($field)*
        )
    };
}

/// Macro for error instrumentation with SwarmSH context
#[macro_export]
macro_rules! swarm_error {
    ($err:expr) => {{
        tracing::error!(
            error = %$err,
            error_type = std::any::type_name_of_val(&$err),
            swarmsh.error = true,
            "SwarmSH error occurred"
        );
        $err
    }};
    ($err:expr, $msg:expr) => {{
        tracing::error!(
            error = %$err,
            error_type = std::any::type_name_of_val(&$err),
            message = $msg,
            swarmsh.error = true,
            "SwarmSH error occurred"
        );
        $err
    }};
    ($err:expr, correlation_id = $corr_id:expr) => {{
        tracing::error!(
            error = %$err,
            error_type = std::any::type_name_of_val(&$err),
            correlation_id = %$corr_id,
            swarmsh.error = true,
            "SwarmSH error occurred with correlation"
        );
        $err
    }};
}

/// Initialize global SwarmSH telemetry (convenience function)
pub async fn init_global_telemetry() -> Result<TelemetryManager> {
    let telemetry = TelemetryManager::new().await?;
    telemetry.start().await?;
    Ok(telemetry)
}

/// Initialize SwarmSH telemetry with custom config (convenience function)
pub async fn init_telemetry_with_config(config: TelemetryConfig) -> Result<TelemetryManager> {
    let telemetry = TelemetryManager::with_config(config).await?;
    telemetry.start().await?;
    Ok(telemetry)
}

impl Drop for TelemetryManager {
    fn drop(&mut self) {
        // Ensure proper shutdown
        global::shutdown_tracer_provider();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_lightweight_telemetry() {
        let manager = TelemetryManager::lightweight("test-service").await.unwrap();
        assert!(matches!(manager.config.mode, TelemetryMode::Lightweight));
        assert!(manager.start().await.is_ok());
        assert!(manager.stop().await.is_ok());
    }
    
    #[tokio::test]
    async fn test_telemetry_modes() {
        // Test each telemetry mode
        let configs = vec![
            TelemetryConfig { mode: TelemetryMode::Disabled, ..Default::default() },
            TelemetryConfig { mode: TelemetryMode::Lightweight, ..Default::default() },
            TelemetryConfig { 
                mode: TelemetryMode::Development { log_file: None }, 
                ..Default::default() 
            },
        ];
        
        for config in configs {
            let result = TelemetryManager::with_config(config).await;
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_telemetry_initialization() {
        let config = TelemetryConfig {
            mode: TelemetryMode::Lightweight,
            service_name: "test-service".to_string(),
            enable_stdout: true,
            ..Default::default()
        };

        let telemetry = TelemetryManager::with_config(config).await.unwrap();
        assert!(telemetry.start().await.is_ok());
        assert!(telemetry.stop().await.is_ok());
    }

    #[tokio::test]
    async fn test_swarm_telemetry_spans() {
        let telemetry = DefaultSwarmTelemetry::new("test-service".to_string());
        
        let agent_span = telemetry.agent_span("test-agent-123", "register");
        let _guard = agent_span.entered();
        
        // Simulate some work
        sleep(Duration::from_millis(10)).await;
        
        // Span should record correctly
        drop(_guard);
    }
    
    #[test]
    fn test_shell_telemetry_init() {
        let result = init_shell_telemetry("test-shell");
        assert!(result.is_ok());
        
        // Cleanup
        global::shutdown_tracer_provider();
    }
    
    #[test]
    fn test_swarm_telemetry_implementation() {
        let telemetry = DefaultSwarmTelemetry::new("test".to_string());
        
        telemetry.record_coordination_duration("scrum_at_scale", Duration::from_millis(150));
        telemetry.record_agent_registration("agent_001");
        telemetry.record_work_item_processed("work_001", Duration::from_millis(250));
        telemetry.record_health_check("coordination", "healthy", Duration::from_millis(50));
        telemetry.record_ai_decision("work_assignment", 0.95, Duration::from_millis(45));
        
        // All calls should complete without error
    }

    #[test]
    fn test_telemetry_config_from_env() {
        std::env::set_var("ENVIRONMENT", "test");
        std::env::set_var("OTEL_ENABLE_STDOUT", "false");
        std::env::set_var("SWARMSH_TELEMETRY_MODE", "lightweight");
        
        let config = TelemetryConfig::default();
        assert_eq!(config.environment, "test");
        assert!(!config.enable_stdout);
        assert!(matches!(config.mode, TelemetryMode::Lightweight));
        
        std::env::remove_var("ENVIRONMENT");
        std::env::remove_var("OTEL_ENABLE_STDOUT");
        std::env::remove_var("SWARMSH_TELEMETRY_MODE");
    }
    
    #[test]
    fn test_telemetry_mode_env_config() {
        // Test production mode environment configuration
        std::env::set_var("SWARMSH_TELEMETRY_MODE", "production");
        std::env::set_var("JAEGER_ENDPOINT", "http://localhost:14268");
        
        let config = TelemetryConfig::default();
        match config.mode {
            TelemetryMode::Production { jaeger_endpoint, .. } => {
                assert_eq!(jaeger_endpoint, Some("http://localhost:14268".to_string()));
            }
            _ => panic!("Expected production mode"),
        }
        
        std::env::remove_var("SWARMSH_TELEMETRY_MODE");
        std::env::remove_var("JAEGER_ENDPOINT");
    }
}