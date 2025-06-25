//! Integration tests for SwarmSH v2 coordination system
//! 
//! Tests the core coordination functionality including agent registration,
//! work claiming, and zero-conflict guarantees.

use anyhow::Result;
use swarmsh_v2::{SwarmSystem, AgentId, WorkId, coordination::AgentSpec};
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_system_startup_and_shutdown() -> Result<()> {
    // Test basic system lifecycle
    let system = SwarmSystem::new().await?;
    
    // Start system
    system.start().await?;
    
    // Verify all components are running
    let health_report = system.health_monitor.collect_health().await?;
    assert!(health_report.score > 50, "System should be healthy after startup");
    
    // Stop system
    system.stop().await?;
    
    Ok(())
}

#[tokio::test]
async fn test_agent_registration() -> Result<()> {
    let system = SwarmSystem::new().await?;
    system.start().await?;
    
    // Create agent specification
    let agent_spec = AgentSpec {
        id: AgentId::generate(),
        role: "worker".to_string(),
        capacity: 0.8,
        specializations: vec!["feature".to_string(), "bug".to_string()],
        work_capacity: Some(3),
    };
    
    // Register agent
    let result = system.coordinator.register_agent(agent_spec.clone()).await;
    assert!(result.is_ok(), "Agent registration should succeed");
    
    // Attempt to register same agent again (should fail due to zero-conflict guarantee)
    let duplicate_result = system.coordinator.register_agent(agent_spec).await;
    assert!(duplicate_result.is_err(), "Duplicate agent registration should fail");
    
    system.stop().await?;
    Ok(())
}

#[tokio::test]
async fn test_nanosecond_precision_ids() -> Result<()> {
    // Test that nanosecond-precision IDs are truly unique
    let mut ids = std::collections::HashSet::new();
    
    for _ in 0..1000 {
        let agent_id = AgentId::generate();
        let work_id = WorkId::generate();
        
        // Verify uniqueness
        assert!(ids.insert(agent_id.0.clone()), "Agent ID should be unique");
        assert!(ids.insert(work_id.0.clone()), "Work ID should be unique");
        
        // Verify format
        assert!(agent_id.0.starts_with("agent_"), "Agent ID should have correct prefix");
        assert!(work_id.0.starts_with("work_"), "Work ID should have correct prefix");
    }
    
    Ok(())
}

#[tokio::test]
async fn test_coordination_patterns() -> Result<()> {
    let system = SwarmSystem::new().await?;
    system.start().await?;
    
    // Test different coordination patterns
    let patterns = vec![
        swarmsh_v2::coordination::CoordinationPattern::ScrumAtScale,
        swarmsh_v2::coordination::CoordinationPattern::RobertsRules,
        swarmsh_v2::coordination::CoordinationPattern::Realtime,
        swarmsh_v2::coordination::CoordinationPattern::Atomic,
    ];
    
    for pattern in patterns {
        let result = system.coordinator.coordinate(pattern).await;
        assert!(result.is_ok(), "Coordination should succeed for all patterns");
    }
    
    system.stop().await?;
    Ok(())
}

#[tokio::test]
async fn test_8020_analytics() -> Result<()> {
    let system = SwarmSystem::new().await?;
    system.start().await?;
    
    // Run 8020 analysis
    let optimization_report = system.analytics.analyze_8020().await?;
    
    // Verify report structure
    assert!(optimization_report.value_ratio > 0.0, "Value ratio should be positive");
    assert!(optimization_report.roi_percentage >= 0.0, "ROI should be non-negative");
    assert!(!optimization_report.recommendations.is_empty(), "Should have recommendations");
    
    // Verify DLSS metrics
    assert!(
        optimization_report.value_stream.flow_efficiency >= 0.0 && 
        optimization_report.value_stream.flow_efficiency <= 100.0,
        "Flow efficiency should be between 0 and 100 percent"
    );
    
    system.stop().await?;
    Ok(())
}

#[tokio::test]
async fn test_health_monitoring() -> Result<()> {
    let system = SwarmSystem::new().await?;
    system.start().await?;
    
    // Collect health metrics
    let health_report = system.health_monitor.collect_health().await?;
    
    // Verify health report structure
    assert!(health_report.score <= 100, "Health score should not exceed 100");
    assert!(!health_report.component.is_empty(), "Component should be specified");
    
    // Test bottleneck detection
    let bottlenecks = system.health_monitor.detect_bottlenecks().await?;
    // Bottlenecks list can be empty in a healthy system
    
    system.stop().await?;
    Ok(())
}

#[tokio::test]
async fn test_shell_export() -> Result<()> {
    let system = SwarmSystem::new().await?;
    
    // Test shell export functionality
    let config = swarmsh_v2::shell_export::ExportConfig {
        output_dir: std::env::temp_dir().join("swarmsh-test-export"),
        include_telemetry: true,
        include_ai_integration: false, // Skip AI for tests
        optimization_level: 1,
    };
    
    let result = system.export_to_shell(config).await;
    assert!(result.is_ok(), "Shell export should succeed");
    
    Ok(())
}

#[tokio::test]
async fn test_concurrent_agent_operations() -> Result<()> {
    let system = SwarmSystem::new().await?;
    system.start().await?;
    
    // Test concurrent agent registrations (should all succeed due to nanosecond precision)
    let mut handles = vec![];
    
    for i in 0..10 {
        let system_clone = system.clone();
        let handle = tokio::spawn(async move {
            let agent_spec = AgentSpec {
                id: AgentId::generate(),
                role: format!("worker_{}", i),
                capacity: 0.7,
                specializations: vec!["test".to_string()],
                work_capacity: Some(1),
            };
            
            system_clone.coordinator.register_agent(agent_spec).await
        });
        handles.push(handle);
    }
    
    // Wait for all registrations to complete
    for handle in handles {
        let result = handle.await?;
        assert!(result.is_ok(), "Concurrent agent registration should succeed");
    }
    
    system.stop().await?;
    Ok(())
}

#[tokio::test]
async fn test_telemetry_generation() -> Result<()> {
    let system = SwarmSystem::new().await?;
    system.start().await?;
    
    // Create agent and perform operations to generate telemetry
    let agent_spec = AgentSpec {
        id: AgentId::generate(),
        role: "test_agent".to_string(),
        capacity: 1.0,
        specializations: vec!["telemetry_test".to_string()],
        work_capacity: Some(1),
    };
    
    // Register agent (should generate telemetry spans)
    system.coordinator.register_agent(agent_spec).await?;
    
    // Collect health (should generate health telemetry)
    system.health_monitor.collect_health().await?;
    
    // Run analytics (should generate analytics telemetry)
    system.analytics.analyze_8020().await?;
    
    // TODO: Verify telemetry spans were generated
    // This will be implemented when OTEL integration is complete
    
    system.stop().await?;
    Ok(())
}

#[tokio::test]
async fn test_waste_detection() -> Result<()> {
    let system = SwarmSystem::new().await?;
    system.start().await?;
    
    // Test waste detection in observability pipeline
    let detected_waste = system.analytics.detect_waste().await?;
    
    // Waste detection should complete without error
    // May or may not find waste depending on system state
    
    system.stop().await?;
    Ok(())
}

#[tokio::test]
async fn test_value_stream_mapping() -> Result<()> {
    let system = SwarmSystem::new().await?;
    system.start().await?;
    
    // Test value stream mapping functionality
    let value_stream_analysis = system.analytics.map_value_streams().await?;
    
    // Verify analysis structure
    assert!(value_stream_analysis.flow_efficiency >= 0.0, "Flow efficiency should be non-negative");
    assert!(value_stream_analysis.lead_time_ms > 0, "Lead time should be positive");
    assert!(value_stream_analysis.waste_percentage >= 0.0, "Waste percentage should be non-negative");
    
    system.stop().await?;
    Ok(())
}
