//! SwarmSH v2 - observability-first agent coordination.
//!
//! The crate provides pull-based work coordination, health and analytics
//! components, OpenTelemetry instrumentation, optional AI-assisted decisions,
//! and a shell-export surface. Claims about behavior are scoped to the
//! executable verification described in `docs/COMPLETION_CONTRACT.md`.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::generated::metrics::SwarmMetrics;

pub mod analytics;
pub mod ai_integration;
pub mod auto_command;
pub mod coordination;
pub mod health;
pub mod scrum_at_scale_simulation;
pub mod shell_export;
pub mod telemetry;
pub mod weaver_forge;
pub mod worktree_manager;

#[cfg(feature = "ai-integration")]
pub mod ollama_weaver_pipeline;

#[cfg(test)]
pub mod phase1_critical_tests;
#[cfg(test)]
pub mod phase2_high_risk_tests;
#[cfg(test)]
pub mod phase3_medium_risk_tests;
#[cfg(test)]
pub mod phase4_production_hardening_tests;
#[cfg(test)]
pub mod telemetry_test;

/// Generated telemetry projections.
///
/// Generated files are committed projections. Reproduction is verified by the
/// Weaver completion gate; generated-file comments alone are not provenance.
pub mod generated {
    pub mod attributes;
    pub mod metrics;
    pub mod span_builders;
}

pub use ai_integration::{AIAnalysis, AIIntegration, AgentDecision};
pub use analytics::{AnalyticsEngine, OptimizationReport, ValueStreamAnalysis};
pub use auto_command::{AutoEngine, AutoMode, AutoResult, Feature, ValueDetectionConfig};
pub use coordination::{AgentCoordinator, AgentSpec, CoordinationPattern, WorkQueue};
pub use health::{HealthMonitor, HealthReport, HealthStatus};
pub use scrum_at_scale_simulation::{
    AgentRole, MeetingType, MotionStatus, ScrumAtScaleSimulation, SimulationMetrics,
};
pub use shell_export::{ExportConfig, ShellExporter};
pub use telemetry::{SwarmTelemetry, TelemetryManager};
pub use weaver_forge::{TemplateConfig, WeaverConfig, WeaverForge};
pub use worktree_manager::{WorktreeManager, WorktreeSpec, WorktreeState, WorktreeStatus};

/// Main SwarmSH coordination system.
#[derive(Clone)]
pub struct SwarmSystem {
    pub coordinator: Arc<AgentCoordinator>,
    pub work_queue: Arc<WorkQueue>,
    pub health_monitor: Arc<HealthMonitor>,
    pub analytics: Arc<AnalyticsEngine>,
    pub telemetry: Arc<TelemetryManager>,
    pub shell_exporter: Arc<ShellExporter>,
    pub worktree_manager: Arc<WorktreeManager>,
    pub ai_integration: Arc<AIIntegration>,
}

impl SwarmSystem {
    /// Create the full system.
    pub async fn new() -> Result<Self> {
        let telemetry = Arc::new(TelemetryManager::new().await?);
        let work_queue = Arc::new(WorkQueue::new(None).await?);
        let coordinator =
            Arc::new(AgentCoordinator::new(telemetry.clone(), work_queue.clone()).await?);
        let health_monitor = Arc::new(HealthMonitor::new(telemetry.clone()).await?);
        let analytics = Arc::new(AnalyticsEngine::new(telemetry.clone()).await?);
        let shell_exporter = Arc::new(ShellExporter::new().await?);
        let worktree_manager = Arc::new(
            WorktreeManager::new(std::env::current_dir()?.join("worktrees"), telemetry.clone())
                .await?,
        );
        let ai_integration = Arc::new(AIIntegration::new().await?);

        Ok(Self {
            coordinator,
            work_queue,
            health_monitor,
            analytics,
            telemetry,
            shell_exporter,
            worktree_manager,
            ai_integration,
        })
    }

    /// Start enabled subsystems.
    pub async fn start(&self) -> Result<()> {
        self.telemetry
            .start()
            .await
            .context("Failed to start telemetry")?;
        self.coordinator
            .start()
            .await
            .context("Failed to start coordinator")?;
        self.health_monitor
            .start()
            .await
            .context("Failed to start health monitor")?;
        self.analytics
            .start()
            .await
            .context("Failed to start analytics")?;

        println!("SwarmSH v2 started");
        println!("- coordination: Scrum at Scale, Roberts Rules, real-time, atomic");
        println!("- telemetry: OpenTelemetry instrumentation enabled by configuration");
        println!(
            "- coordinator AI assistance: {}",
            if self.coordinator.ai_enabled() {
                "enabled"
            } else {
                "disabled"
            }
        );
        println!("- shell export: available through swarmsh-exporter");

        Ok(())
    }

    /// Export the system to shell scripts.
    pub async fn export_to_shell(&self, config: ExportConfig) -> Result<()> {
        self.shell_exporter
            .export_system(self, config)
            .await
            .context("Failed to export system to shell scripts")
    }

    /// Stop started subsystems.
    pub async fn stop(&self) -> Result<()> {
        self.analytics
            .stop()
            .await
            .context("Failed to stop analytics")?;
        self.health_monitor
            .stop()
            .await
            .context("Failed to stop health monitor")?;
        self.coordinator
            .stop()
            .await
            .context("Failed to stop coordinator")?;
        self.telemetry
            .stop()
            .await
            .context("Failed to stop telemetry")?;

        println!("SwarmSH v2 stopped");
        Ok(())
    }

    /// Return generated metrics when a metrics backend is attached.
    pub fn metrics(&self) -> Option<SwarmMetrics> {
        None
    }

    pub fn create_agent_span(&self, agent_id: &str, operation: &str) -> tracing::Span {
        tracing::info_span!(
            "swarmsh.agent.lifecycle",
            agent_id = %agent_id,
            operation = %operation
        )
    }

    pub fn create_work_span(&self, work_id: &str, operation: &str) -> tracing::Span {
        tracing::info_span!(
            "swarmsh.work.coordination",
            work_id = %work_id,
            operation = %operation
        )
    }

    pub fn create_coordination_span(&self, pattern: &str, operation: &str) -> tracing::Span {
        tracing::info_span!(
            "swarmsh.coordination.protocol",
            pattern = %pattern,
            operation = %operation
        )
    }
}

pub type AgentId = String;
pub type WorkId = String;

/// Monotonic-in-process coordination epoch seed derived from UNIX time.
///
/// This is an identifier component, not a global uniqueness proof.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct CoordinationEpoch(pub u64);

impl CoordinationEpoch {
    pub fn new() -> Self {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock must be after UNIX_EPOCH")
            .as_nanos();
        Self(u64::try_from(nanos).unwrap_or(u64::MAX))
    }

    pub fn advance(&mut self) {
        self.0 = self.0.saturating_add(1);
    }
}

impl Default for CoordinationEpoch {
    fn default() -> Self {
        Self::new()
    }
}

/// Error types for SwarmSH operations.
#[derive(Debug, thiserror::Error)]
pub enum SwarmError {
    #[error("Coordination conflict detected")]
    CoordinationConflict,

    #[error("Agent not found: {agent_id}")]
    AgentNotFound { agent_id: String },

    #[error("Work item not found: {work_id}")]
    WorkNotFound { work_id: String },

    #[error("Lock acquisition failed")]
    LockFailed,

    #[error("Health check failed: {component}")]
    HealthCheckFailed { component: String },

    #[error("Resource already exists: {0}")]
    AlreadyExists(String),

    #[error("Telemetry error: {0}")]
    TelemetryError(#[from] opentelemetry::trace::TraceError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("System time error: {0}")]
    SystemTimeError(#[from] std::time::SystemTimeError),

    #[error("Git operation failed: {0}")]
    GitOperation(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Coordination error: {0}")]
    Coordination(String),

    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}

impl SwarmError {
    pub fn coordination_conflict() -> Self {
        Self::CoordinationConflict
    }

    pub fn agent_not_found(agent_id: impl Into<String>) -> Self {
        Self::AgentNotFound {
            agent_id: agent_id.into(),
        }
    }

    pub fn work_not_found(work_id: impl Into<String>) -> Self {
        Self::WorkNotFound {
            work_id: work_id.into(),
        }
    }

    pub fn lock_failed() -> Self {
        Self::LockFailed
    }

    pub fn health_check_failed(component: impl Into<String>) -> Self {
        Self::HealthCheckFailed {
            component: component.into(),
        }
    }
}

pub type SwarmResult<T> = Result<T, SwarmError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_id_type_is_nonempty_when_constructed_with_data() {
        let id: AgentId = "test_agent_123".to_string();
        assert!(!id.is_empty());
    }

    #[test]
    fn work_id_type_is_nonempty_when_constructed_with_data() {
        let id: WorkId = "test_work_456".to_string();
        assert!(!id.is_empty());
    }

    #[test]
    fn coordination_epoch_advances() {
        let mut epoch = CoordinationEpoch::new();
        let initial = epoch.0;
        epoch.advance();
        assert!(epoch.0 >= initial);
    }
}
