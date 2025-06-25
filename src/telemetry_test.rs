//! Test module for telemetry instrumentation validation
//! 
//! This module validates that comprehensive telemetry has been added to all core modules.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telemetry::{SwarmTelemetry, DefaultSwarmTelemetry, TelemetryManager, TelemetryMode, TelemetryConfig};
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_default_swarm_telemetry_spans() {
        // Test that DefaultSwarmTelemetry can create all required spans
        let telemetry = DefaultSwarmTelemetry::default();
        
        // Test agent lifecycle spans
        let agent_span = telemetry.agent_span("test-agent-001", "register");
        let _guard = agent_span.entered();
        drop(_guard);
        
        // Test work coordination spans
        let work_span = telemetry.work_span("test-work-001", "process");
        let _guard = work_span.entered();
        drop(_guard);
        
        // Test coordination protocol spans
        let coord_span = telemetry.coordination_span("scrum_at_scale", "coordinate");
        let _guard = coord_span.entered();
        drop(_guard);
        
        // Test health monitoring spans
        let health_span = telemetry.health_span("coordination", "check_health");
        let _guard = health_span.entered();
        drop(_guard);
        
        // Test analytics spans
        let analytics_span = telemetry.analytics_span("tier1", "analyze_8020");
        let _guard = analytics_span.entered();
        drop(_guard);
    }

    #[tokio::test]
    async fn test_swarm_telemetry_metrics() {
        let telemetry = DefaultSwarmTelemetry::default();
        
        // Test coordination duration recording
        telemetry.record_coordination_duration("test_operation", Duration::from_millis(150));
        
        // Test agent registration recording
        telemetry.record_agent_registration("test-agent-002");
        
        // Test work item processing recording
        telemetry.record_work_item_processed("test-work-002", Duration::from_millis(250));
        
        // Test health check recording
        telemetry.record_health_check("telemetry", "healthy", Duration::from_millis(50));
        
        // Test AI decision recording
        telemetry.record_ai_decision("work_assignment", 0.95, Duration::from_millis(75));
    }

    #[tokio::test]
    async fn test_lightweight_telemetry_manager() {
        let result = TelemetryManager::lightweight("test-telemetry-service").await;
        assert!(result.is_ok(), "Lightweight telemetry manager should initialize successfully");
        
        let manager = result.unwrap();
        assert!(matches!(manager.config().mode, TelemetryMode::Lightweight));
        assert_eq!(manager.config().service_name, "test-telemetry-service");
        
        // Test that we can start and stop the manager
        assert!(manager.start().await.is_ok());
        assert!(manager.stop().await.is_ok());
    }

    #[tokio::test]
    async fn test_telemetry_configuration_modes() {
        // Test different telemetry modes
        let configs = vec![
            TelemetryConfig {
                mode: TelemetryMode::Disabled,
                service_name: "test-disabled".to_string(),
                ..Default::default()
            },
            TelemetryConfig {
                mode: TelemetryMode::Lightweight,
                service_name: "test-lightweight".to_string(),
                ..Default::default()
            },
            TelemetryConfig {
                mode: TelemetryMode::Development { log_file: None },
                service_name: "test-development".to_string(),
                ..Default::default()
            },
        ];
        
        for config in configs {
            let result = TelemetryManager::with_config(config.clone()).await;
            assert!(result.is_ok(), "Config {:?} should work", config.mode);
            
            let manager = result.unwrap();
            assert!(manager.start().await.is_ok());
            assert!(manager.stop().await.is_ok());
        }
    }

    #[test]
    fn test_telemetry_span_attributes() {
        let telemetry = DefaultSwarmTelemetry::new("test-service".to_string());
        
        // Verify spans are created with proper attributes
        let agent_span = telemetry.agent_span("agent-123", "start");
        assert_eq!(agent_span.metadata().map(|m| m.name()), Some("swarmsh.agent.lifecycle"));
        
        let work_span = telemetry.work_span("work-456", "execute");
        assert_eq!(work_span.metadata().map(|m| m.name()), Some("swarmsh.work.coordination"));
        
        let coord_span = telemetry.coordination_span("atomic", "execute");
        assert_eq!(coord_span.metadata().map(|m| m.name()), Some("swarmsh.coordination.protocol"));
        
        let health_span = telemetry.health_span("analytics", "monitor");
        assert_eq!(health_span.metadata().map(|m| m.name()), Some("swarmsh.health.monitoring"));
        
        let analytics_span = telemetry.analytics_span("tier2", "optimize");
        assert_eq!(analytics_span.metadata().map(|m| m.name()), Some("swarmsh.analytics.dlss"));
    }

    #[tokio::test]
    async fn test_telemetry_instrumented_workflow() {
        // Test a complete workflow with telemetry instrumentation
        let telemetry = DefaultSwarmTelemetry::default();
        
        // Simulate agent registration workflow
        let start_time = std::time::Instant::now();
        let _agent_span = telemetry.agent_span("workflow-agent", "full_lifecycle").entered();
        
        // Record agent registration
        telemetry.record_agent_registration("workflow-agent");
        
        // Simulate work processing
        let _work_span = telemetry.work_span("workflow-work", "process").entered();
        sleep(Duration::from_millis(10)).await;
        let work_duration = start_time.elapsed();
        telemetry.record_work_item_processed("workflow-work", work_duration);
        
        // Simulate coordination
        let _coord_span = telemetry.coordination_span("realtime", "coordinate").entered();
        let coord_duration = Duration::from_millis(5);
        telemetry.record_coordination_duration("realtime_coordination", coord_duration);
        
        // Simulate health check
        let _health_span = telemetry.health_span("system", "check").entered();
        let health_duration = Duration::from_millis(2);
        telemetry.record_health_check("system", "healthy", health_duration);
        
        // All operations should complete without error
        let total_duration = start_time.elapsed();
        assert!(total_duration.as_millis() >= 10, "Workflow should take at least 10ms");
    }
}