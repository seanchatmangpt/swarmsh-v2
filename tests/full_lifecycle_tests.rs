//! Full lifecycle unit tests for SwarmSH v2 coordination system
//! 
//! Comprehensive tests covering complete agent and work coordination lifecycles
//! with OTEL Weaver validation and zero-conflict guarantees.

use anyhow::Result;
use swarmsh_v2::{
    SwarmSystem, AgentId, WorkId, 
    coordination::{AgentSpec, CoordinationPattern, WorkSpec, WorkState},
    health::{HealthMetrics, BottleneckReport},
    analytics::{OptimizationReport, ValueStreamAnalysis},
    shell_export::ExportConfig,
};
use tokio::time::{sleep, Duration, Instant};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

/// Test complete agent lifecycle from registration to deregistration
#[tokio::test]
async fn test_complete_agent_lifecycle() -> Result<()> {
    let system = SwarmSystem::new().await?;
    system.start().await?;
    
    // Phase 1: Agent Registration
    let agent_spec = AgentSpec {
        id: AgentId::generate(),
        role: "full_lifecycle_worker".to_string(),
        capacity: 0.9,
        specializations: vec!["feature".to_string(), "optimization".to_string()],
        work_capacity: Some(5),
    };
    
    let agent_id = agent_spec.id.clone();
    
    // Register agent and verify registration
    let registration_result = system.coordinator.register_agent(agent_spec).await?;
    assert!(registration_result.success, "Agent registration should succeed");
    assert_eq!(registration_result.agent_id, agent_id, "Agent ID should match");
    
    // Phase 2: Agent Work Claiming and Execution
    let work_spec = WorkSpec {
        id: WorkId::generate(),
        work_type: "feature".to_string(),
        priority: "high".to_string(),
        description: "Test work item for lifecycle validation".to_string(),
        specification: "Complete feature implementation with telemetry".to_string(),
        tags: vec!["test".to_string(), "lifecycle".to_string()],
        estimated_effort: Some(3.0),
    };
    
    // Submit work
    let work_id = work_spec.id.clone();
    system.coordinator.submit_work(work_spec).await?;
    
    // Agent claims work
    let claim_result = system.coordinator.claim_work(agent_id.clone(), vec!["feature".to_string()]).await?;
    assert!(!claim_result.claimed_work.is_empty(), "Agent should claim work");
    assert_eq!(claim_result.claimed_work[0].id, work_id, "Should claim the correct work");
    
    // Simulate work execution
    sleep(Duration::from_millis(100)).await;
    
    // Agent completes work
    let completion_result = system.coordinator.complete_work(agent_id.clone(), work_id.clone()).await?;
    assert!(completion_result.success, "Work completion should succeed");
    
    // Phase 3: Health Monitoring During Lifecycle
    let health_report = system.health_monitor.collect_health().await?;
    assert!(health_report.score > 70, "System should be healthy during normal operation");
    
    // Phase 4: Analytics Collection
    let analytics_report = system.analytics.analyze_8020().await?;
    assert!(analytics_report.value_ratio > 0.0, "Analytics should show positive value ratio");
    
    // Phase 5: Agent Deregistration
    let deregistration_result = system.coordinator.deregister_agent(agent_id.clone()).await?;
    assert!(deregistration_result.success, "Agent deregistration should succeed");
    
    // Verify agent is no longer registered
    let agents = system.coordinator.list_agents().await?;
    assert!(!agents.iter().any(|a| a.id == agent_id), "Agent should no longer be registered");
    
    system.stop().await?;
    Ok(())
}

/// Test work lifecycle from creation to completion with multiple agents
#[tokio::test]
async fn test_complete_work_lifecycle() -> Result<()> {
    let system = SwarmSystem::new().await?;
    system.start().await?;
    
    // Create multiple agents with different specializations
    let agent_specs = vec![
        AgentSpec {
            id: AgentId::generate(),
            role: "feature_specialist".to_string(),
            capacity: 1.0,
            specializations: vec!["feature".to_string()],
            work_capacity: Some(3),
        },
        AgentSpec {
            id: AgentId::generate(),
            role: "bug_specialist".to_string(),
            capacity: 0.8,
            specializations: vec!["bug".to_string()],
            work_capacity: Some(5),
        },
        AgentSpec {
            id: AgentId::generate(),
            role: "optimization_specialist".to_string(),
            capacity: 0.9,
            specializations: vec!["optimization".to_string()],
            work_capacity: Some(2),
        },
    ];
    
    // Register all agents
    let mut agent_ids = Vec::new();
    for spec in agent_specs {
        let agent_id = spec.id.clone();
        system.coordinator.register_agent(spec).await?;
        agent_ids.push(agent_id);
    }
    
    // Create diverse work items
    let work_specs = vec![
        WorkSpec {
            id: WorkId::generate(),
            work_type: "feature".to_string(),
            priority: "high".to_string(),
            description: "Implement new user authentication".to_string(),
            specification: "Add OAuth2 integration with telemetry".to_string(),
            tags: vec!["auth".to_string(), "security".to_string()],
            estimated_effort: Some(5.0),
        },
        WorkSpec {
            id: WorkId::generate(),
            work_type: "bug".to_string(),
            priority: "critical".to_string(),
            description: "Fix memory leak in coordination module".to_string(),
            specification: "Identify and resolve memory leak causing performance degradation".to_string(),
            tags: vec!["performance".to_string(), "memory".to_string()],
            estimated_effort: Some(3.0),
        },
        WorkSpec {
            id: WorkId::generate(),
            work_type: "optimization".to_string(),
            priority: "medium".to_string(),
            description: "Optimize telemetry collection performance".to_string(),
            specification: "Reduce telemetry overhead by 25%".to_string(),
            tags: vec!["performance".to_string(), "telemetry".to_string()],
            estimated_effort: Some(4.0),
        },
    ];
    
    // Submit all work items
    let mut work_ids = Vec::new();
    for spec in work_specs {
        let work_id = spec.id.clone();
        system.coordinator.submit_work(spec).await?;
        work_ids.push(work_id);
    }
    
    // Agents claim work based on specializations
    let mut claimed_work = HashMap::new();
    for (i, agent_id) in agent_ids.iter().enumerate() {
        let specialization = match i {
            0 => vec!["feature".to_string()],
            1 => vec!["bug".to_string()],
            2 => vec!["optimization".to_string()],
            _ => vec![],
        };
        
        let claim_result = system.coordinator.claim_work(agent_id.clone(), specialization).await?;
        if !claim_result.claimed_work.is_empty() {
            claimed_work.insert(agent_id.clone(), claim_result.claimed_work);
        }
    }
    
    // Verify all work was claimed
    let total_claimed: usize = claimed_work.values().map(|w| w.len()).sum();
    assert_eq!(total_claimed, work_ids.len(), "All work should be claimed");
    
    // Simulate work execution and completion
    for (agent_id, work_items) in claimed_work {
        for work_item in work_items {
            // Simulate work execution time
            sleep(Duration::from_millis(50)).await;
            
            // Complete work
            let completion_result = system.coordinator.complete_work(agent_id.clone(), work_item.id).await?;
            assert!(completion_result.success, "Work completion should succeed");
        }
    }
    
    // Verify all work is completed
    let work_states = system.coordinator.get_work_states(work_ids.clone()).await?;
    for state in work_states {
        assert_eq!(state, WorkState::Completed, "All work should be completed");
    }
    
    system.stop().await?;
    Ok(())
}

/// Test coordination patterns lifecycle with zero-conflict guarantees
#[tokio::test]
async fn test_coordination_patterns_lifecycle() -> Result<()> {
    let system = SwarmSystem::new().await?;
    system.start().await?;
    
    // Test all coordination patterns
    let patterns = vec![
        CoordinationPattern::ScrumAtScale,
        CoordinationPattern::RobertsRules,
        CoordinationPattern::Realtime,
        CoordinationPattern::Atomic,
    ];
    
    for pattern in patterns {
        // Initialize pattern
        let init_result = system.coordinator.initialize_pattern(pattern.clone()).await?;
        assert!(init_result.success, "Pattern initialization should succeed");
        
        // Execute coordination with pattern
        let coord_result = system.coordinator.coordinate_with_pattern(pattern.clone()).await?;
        assert!(coord_result.conflicts == 0, "Zero conflicts should be maintained");
        assert!(coord_result.nanosecond_precision, "Nanosecond precision should be maintained");
        
        // Verify pattern-specific behavior
        match pattern {
            CoordinationPattern::ScrumAtScale => {
                assert!(coord_result.scrum_events_executed > 0, "Scrum events should be executed");
            },
            CoordinationPattern::RobertsRules => {
                assert!(coord_result.parliamentary_procedures > 0, "Parliamentary procedures should be executed");
            },
            CoordinationPattern::Realtime => {
                assert!(coord_result.latency_ms < 10, "Real-time coordination should have low latency");
            },
            CoordinationPattern::Atomic => {
                assert!(coord_result.atomic_operations > 0, "Atomic operations should be executed");
            },
        }
        
        // Cleanup pattern
        let cleanup_result = system.coordinator.cleanup_pattern(pattern).await?;
        assert!(cleanup_result.success, "Pattern cleanup should succeed");
    }
    
    system.stop().await?;
    Ok(())
}

/// Test health monitoring throughout system lifecycle
#[tokio::test]
async fn test_health_monitoring_lifecycle() -> Result<()> {
    let system = SwarmSystem::new().await?;
    system.start().await?;
    
    // Phase 1: Initial Health Check
    let initial_health = system.health_monitor.collect_health().await?;
    assert!(initial_health.score > 50, "Initial health should be reasonable");
    
    // Phase 2: Load System with Agents and Work
    let mut agent_ids = Vec::new();
    for i in 0..10 {
        let agent_spec = AgentSpec {
            id: AgentId::generate(),
            role: format!("health_test_agent_{}", i),
            capacity: 0.8,
            specializations: vec!["test".to_string()],
            work_capacity: Some(2),
        };
        
        let agent_id = agent_spec.id.clone();
        system.coordinator.register_agent(agent_spec).await?;
        agent_ids.push(agent_id);
    }
    
    // Add work load
    for i in 0..20 {
        let work_spec = WorkSpec {
            id: WorkId::generate(),
            work_type: "test".to_string(),
            priority: "normal".to_string(),
            description: format!("Health test work item {}", i),
            specification: "Test work for health monitoring".to_string(),
            tags: vec!["health_test".to_string()],
            estimated_effort: Some(1.0),
        };
        
        system.coordinator.submit_work(work_spec).await?;
    }
    
    // Phase 3: Monitor Health Under Load
    let loaded_health = system.health_monitor.collect_health().await?;
    // Health might decrease under load but should remain functional
    assert!(loaded_health.score > 30, "Health should remain functional under load");
    
    // Phase 4: Bottleneck Detection
    let bottlenecks = system.health_monitor.detect_bottlenecks().await?;
    // System may or may not have bottlenecks depending on load
    
    // Phase 5: Health Recovery
    // Remove load by completing work
    for agent_id in &agent_ids {
        let claim_result = system.coordinator.claim_work(agent_id.clone(), vec!["test".to_string()]).await?;
        for work_item in claim_result.claimed_work {
            system.coordinator.complete_work(agent_id.clone(), work_item.id).await?;
        }
    }
    
    // Wait for system to stabilize
    sleep(Duration::from_millis(500)).await;
    
    let recovery_health = system.health_monitor.collect_health().await?;
    assert!(recovery_health.score >= loaded_health.score, "Health should recover after load reduction");
    
    system.stop().await?;
    Ok(())
}

/// Test analytics lifecycle with value stream mapping
#[tokio::test]
async fn test_analytics_lifecycle() -> Result<()> {
    let system = SwarmSystem::new().await?;
    system.start().await?;
    
    // Phase 1: Baseline Analytics
    let baseline_analytics = system.analytics.analyze_8020().await?;
    let baseline_value_stream = system.analytics.map_value_streams().await?;
    
    // Phase 2: Generate System Activity
    let agent_spec = AgentSpec {
        id: AgentId::generate(),
        role: "analytics_test_agent".to_string(),
        capacity: 1.0,
        specializations: vec!["analytics_test".to_string()],
        work_capacity: Some(3),
    };
    
    let agent_id = agent_spec.id.clone();
    system.coordinator.register_agent(agent_spec).await?;
    
    // Create work with different priorities to test value stream analysis
    let work_specs = vec![
        WorkSpec {
            id: WorkId::generate(),
            work_type: "analytics_test".to_string(),
            priority: "high".to_string(),
            description: "High priority analytics test work".to_string(),
            specification: "Test high-value work for analytics".to_string(),
            tags: vec!["high_value".to_string()],
            estimated_effort: Some(2.0),
        },
        WorkSpec {
            id: WorkId::generate(),
            work_type: "analytics_test".to_string(),
            priority: "low".to_string(),
            description: "Low priority analytics test work".to_string(),
            specification: "Test low-value work for analytics".to_string(),
            tags: vec!["low_value".to_string()],
            estimated_effort: Some(1.0),
        },
    ];
    
    // Submit and process work
    for spec in work_specs {
        let work_id = spec.id.clone();
        system.coordinator.submit_work(spec).await?;
        
        let claim_result = system.coordinator.claim_work(agent_id.clone(), vec!["analytics_test".to_string()]).await?;
        if !claim_result.claimed_work.is_empty() {
            sleep(Duration::from_millis(100)).await; // Simulate work
            system.coordinator.complete_work(agent_id.clone(), work_id).await?;
        }
    }
    
    // Phase 3: Post-Activity Analytics
    let activity_analytics = system.analytics.analyze_8020().await?;
    let activity_value_stream = system.analytics.map_value_streams().await?;
    
    // Verify analytics captured activity
    assert!(activity_analytics.total_operations > baseline_analytics.total_operations, 
            "Analytics should capture increased activity");
    assert!(activity_value_stream.lead_time_ms > 0, "Value stream should show lead time");
    
    // Phase 4: Waste Detection
    let detected_waste = system.analytics.detect_waste().await?;
    // Waste detection should complete successfully
    
    // Phase 5: Continuous Improvement Recommendations
    let improvement_recommendations = system.analytics.generate_improvement_recommendations().await?;
    assert!(!improvement_recommendations.is_empty(), "Should generate improvement recommendations");
    
    system.stop().await?;
    Ok(())
}

/// Test shell export lifecycle with validation
#[tokio::test]
async fn test_shell_export_lifecycle() -> Result<()> {
    let system = SwarmSystem::new().await?;
    
    // Phase 1: Generate Shell Export
    let export_config = ExportConfig {
        output_dir: std::env::temp_dir().join("swarmsh-lifecycle-test"),
        include_telemetry: true,
        include_ai_integration: false, // Skip AI for tests
        optimization_level: 2,
    };
    
    let export_result = system.export_to_shell(export_config.clone()).await?;
    assert!(export_result.success, "Shell export should succeed");
    assert!(!export_result.generated_files.is_empty(), "Should generate shell files");
    
    // Phase 2: Validate Generated Shell Scripts
    for file_path in &export_result.generated_files {
        // Verify file exists
        assert!(file_path.exists(), "Generated shell file should exist");
        
        // Verify file is executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = std::fs::metadata(file_path)?;
            let permissions = metadata.permissions();
            assert!(permissions.mode() & 0o111 != 0, "Shell file should be executable");
        }
        
        // Verify shell syntax (basic check)
        let content = std::fs::read_to_string(file_path)?;
        assert!(content.starts_with("#!/"), "Shell file should have shebang");
        assert!(content.contains("# SwarmSH v2"), "Shell file should have SwarmSH header");
    }
    
    // Phase 3: Test Shell Script Functionality
    let coordination_script = export_result.generated_files.iter()
        .find(|p| p.file_name().unwrap().to_str().unwrap().contains("coordination"))
        .expect("Should have coordination script");
    
    // Test shell script execution (basic validation)
    let output = std::process::Command::new("bash")
        .arg("-n") // Syntax check only
        .arg(coordination_script)
        .output()?;
    
    assert!(output.status.success(), "Shell script should pass syntax check");
    
    // Cleanup
    std::fs::remove_dir_all(&export_config.output_dir)?;
    
    Ok(())
}

/// Test concurrent operations with zero-conflict guarantees
#[tokio::test]
async fn test_concurrent_lifecycle_operations() -> Result<()> {
    let system = SwarmSystem::new().await?;
    system.start().await?;
    
    let conflicts = Arc::new(Mutex::new(0));
    let successful_operations = Arc::new(Mutex::new(0));
    
    // Create concurrent operations
    let mut handles = Vec::new();
    
    for i in 0..20 {
        let system_clone = system.clone();
        let conflicts_clone = conflicts.clone();
        let successful_operations_clone = successful_operations.clone();
        
        let handle = tokio::spawn(async move {
            let agent_spec = AgentSpec {
                id: AgentId::generate(),
                role: format!("concurrent_agent_{}", i),
                capacity: 0.8,
                specializations: vec!["concurrent_test".to_string()],
                work_capacity: Some(1),
            };
            
            let agent_id = agent_spec.id.clone();
            
            // Agent registration
            match system_clone.coordinator.register_agent(agent_spec).await {
                Ok(_) => {
                    // Create and claim work
                    let work_spec = WorkSpec {
                        id: WorkId::generate(),
                        work_type: "concurrent_test".to_string(),
                        priority: "normal".to_string(),
                        description: format!("Concurrent test work {}", i),
                        specification: "Test concurrent operations".to_string(),
                        tags: vec!["concurrent".to_string()],
                        estimated_effort: Some(1.0),
                    };
                    
                    let work_id = work_spec.id.clone();
                    
                    if let Ok(_) = system_clone.coordinator.submit_work(work_spec).await {
                        if let Ok(claim_result) = system_clone.coordinator.claim_work(
                            agent_id.clone(), 
                            vec!["concurrent_test".to_string()]
                        ).await {
                            if !claim_result.claimed_work.is_empty() {
                                // Complete work
                                if let Ok(_) = system_clone.coordinator.complete_work(agent_id, work_id).await {
                                    let mut ops = successful_operations_clone.lock().unwrap();
                                    *ops += 1;
                                }
                            }
                        }
                    }
                },
                Err(_) => {
                    let mut conf = conflicts_clone.lock().unwrap();
                    *conf += 1;
                }
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all operations to complete
    for handle in handles {
        handle.await?;
    }
    
    let final_conflicts = *conflicts.lock().unwrap();
    let final_successful_operations = *successful_operations.lock().unwrap();
    
    // Verify zero-conflict guarantee
    assert_eq!(final_conflicts, 0, "Should have zero conflicts in concurrent operations");
    assert!(final_successful_operations > 0, "Should have successful operations");
    
    system.stop().await?;
    Ok(())
}

/// Test system resilience and error recovery
#[tokio::test]
async fn test_error_recovery_lifecycle() -> Result<()> {
    let system = SwarmSystem::new().await?;
    system.start().await?;
    
    // Phase 1: Normal Operation
    let agent_spec = AgentSpec {
        id: AgentId::generate(),
        role: "resilience_test_agent".to_string(),
        capacity: 1.0,
        specializations: vec!["resilience_test".to_string()],
        work_capacity: Some(1),
    };
    
    let agent_id = agent_spec.id.clone();
    system.coordinator.register_agent(agent_spec).await?;
    
    // Phase 2: Simulate Error Conditions
    // Try to register agent with same ID (should fail gracefully)
    let duplicate_spec = AgentSpec {
        id: agent_id.clone(),
        role: "duplicate_agent".to_string(),
        capacity: 0.5,
        specializations: vec!["duplicate".to_string()],
        work_capacity: Some(1),
    };
    
    let duplicate_result = system.coordinator.register_agent(duplicate_spec).await;
    assert!(duplicate_result.is_err(), "Duplicate agent registration should fail");
    
    // Try to claim non-existent work
    let invalid_claim = system.coordinator.claim_work(agent_id.clone(), vec!["nonexistent".to_string()]).await?;
    assert!(invalid_claim.claimed_work.is_empty(), "Should not claim non-existent work");
    
    // Phase 3: Verify System Recovery
    let health_after_errors = system.health_monitor.collect_health().await?;
    assert!(health_after_errors.score > 50, "System should recover from errors");
    
    // Phase 4: Normal Operation After Recovery
    let work_spec = WorkSpec {
        id: WorkId::generate(),
        work_type: "resilience_test".to_string(),
        priority: "normal".to_string(),
        description: "Recovery test work".to_string(),
        specification: "Test system recovery".to_string(),
        tags: vec!["recovery".to_string()],
        estimated_effort: Some(1.0),
    };
    
    let work_id = work_spec.id.clone();
    system.coordinator.submit_work(work_spec).await?;
    
    let claim_result = system.coordinator.claim_work(agent_id.clone(), vec!["resilience_test".to_string()]).await?;
    assert!(!claim_result.claimed_work.is_empty(), "Should claim work after recovery");
    
    let completion_result = system.coordinator.complete_work(agent_id, work_id).await?;
    assert!(completion_result.success, "Should complete work after recovery");
    
    system.stop().await?;
    Ok(())
}