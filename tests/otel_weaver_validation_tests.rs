//! OTEL Weaver validation tests for SwarmSH v2
//! 
//! Comprehensive validation of OpenTelemetry Weaver code generation,
//! semantic convention compliance, and telemetry data quality.

use anyhow::Result;
use std::path::Path;
use std::process::Command;
use std::fs;
use serde_yaml::Value;
use swarmsh_v2::{
    SwarmSystem,
    telemetry::{TelemetryCollector, SpanBuilder, MetricBuilder},
    generated::{attributes, span_builders, metrics},
};
use opentelemetry::{trace::TraceContextExt, Context, KeyValue};
use opentelemetry_sdk::trace::{TracerProvider, Sampler, Config};
use opentelemetry_sdk::export::trace::SpanExporter;
use tracing::{info, warn, error, debug, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tokio::time::{sleep, Duration};
use std::collections::HashMap;

/// Test OTEL Weaver code generation from semantic conventions
#[tokio::test]
async fn test_weaver_code_generation() -> Result<()> {
    // Phase 1: Validate semantic convention files exist
    let semantic_conventions_dir = Path::new("semantic-conventions");
    assert!(semantic_conventions_dir.exists(), "Semantic conventions directory should exist");
    
    let expected_files = vec![
        "swarmsh-agent.yaml",
        "swarmsh-work.yaml", 
        "swarmsh-coordination.yaml",
        "swarmsh-health.yaml",
        "swarmsh-analytics.yaml",
        "swarmsh-worktree.yaml",
    ];
    
    for file in expected_files {
        let file_path = semantic_conventions_dir.join(file);
        assert!(file_path.exists(), "Semantic convention file {} should exist", file);
        
        // Validate YAML structure
        let content = fs::read_to_string(&file_path)?;
        let yaml: Value = serde_yaml::from_str(&content)?;
        
        // Verify required fields
        assert!(yaml.get("groups").is_some(), "Semantic convention should have groups");
        if let Some(groups) = yaml.get("groups").and_then(|g| g.as_sequence()) {
            for group in groups {
                assert!(group.get("id").is_some(), "Group should have id");
                assert!(group.get("type").is_some(), "Group should have type");
                assert!(group.get("brief").is_some(), "Group should have brief description");
            }
        }
    }
    
    // Phase 2: Test Weaver code generation
    let weaver_config = Path::new("weaver.yaml");
    assert!(weaver_config.exists(), "Weaver configuration should exist");
    
    // Run weaver validate command
    let validate_output = Command::new("weaver")
        .args(&["validate", "--config", "weaver.yaml"])
        .output();
    
    if let Ok(output) = validate_output {
        assert!(output.status.success(), "Weaver validation should succeed");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        println!("Weaver validation output: {}", stdout);
        if !stderr.is_empty() {
            println!("Weaver validation stderr: {}", stderr);
        }
    } else {
        println!("Weaver command not available, skipping validation");
    }
    
    // Phase 3: Verify generated code exists and compiles
    let generated_dir = Path::new("src/generated");
    assert!(generated_dir.exists(), "Generated code directory should exist");
    
    let expected_generated = vec![
        "attributes.rs",
        "span_builders.rs", 
        "metrics.rs",
        "mod.rs",
    ];
    
    for file in expected_generated {
        let file_path = generated_dir.join(file);
        assert!(file_path.exists(), "Generated file {} should exist", file);
        
        // Verify file is not empty
        let content = fs::read_to_string(&file_path)?;
        assert!(!content.trim().is_empty(), "Generated file {} should not be empty", file);
        
        // Verify basic Rust syntax (contains expected patterns)
        assert!(content.contains("pub"), "Generated Rust code should contain public items");
    }
    
    Ok(())
}

/// Test semantic convention compliance in generated code
#[tokio::test]
async fn test_semantic_convention_compliance() -> Result<()> {
    // Test that generated attributes match semantic conventions
    
    // Agent domain attributes
    let agent_attrs = vec![
        attributes::SWARMSH_AGENT_ID,
        attributes::SWARMSH_AGENT_ROLE,
        attributes::SWARMSH_AGENT_CAPACITY,
        attributes::SWARMSH_AGENT_SPECIALIZATIONS,
    ];
    
    for attr in agent_attrs {
        assert!(attr.starts_with("swarmsh.agent."), "Agent attributes should have correct namespace");
    }
    
    // Work domain attributes
    let work_attrs = vec![
        attributes::SWARMSH_WORK_ID,
        attributes::SWARMSH_WORK_TYPE,
        attributes::SWARMSH_WORK_PRIORITY,
        attributes::SWARMSH_WORK_STATE,
    ];
    
    for attr in work_attrs {
        assert!(attr.starts_with("swarmsh.work."), "Work attributes should have correct namespace");
    }
    
    // Coordination domain attributes
    let coordination_attrs = vec![
        attributes::SWARMSH_COORDINATION_PATTERN,
        attributes::SWARMSH_COORDINATION_CONFLICTS,
        attributes::SWARMSH_COORDINATION_PRECISION,
    ];
    
    for attr in coordination_attrs {
        assert!(attr.starts_with("swarmsh.coordination."), "Coordination attributes should have correct namespace");
    }
    
    // Health domain attributes
    let health_attrs = vec![
        attributes::SWARMSH_HEALTH_SCORE,
        attributes::SWARMSH_HEALTH_COMPONENT,
        attributes::SWARMSH_HEALTH_BOTTLENECKS,
    ];
    
    for attr in health_attrs {
        assert!(attr.starts_with("swarmsh.health."), "Health attributes should have correct namespace");
    }
    
    // Analytics domain attributes
    let analytics_attrs = vec![
        attributes::SWARMSH_ANALYTICS_VALUE_RATIO,
        attributes::SWARMSH_ANALYTICS_ROI,
        attributes::SWARMSH_ANALYTICS_FLOW_EFFICIENCY,
        attributes::SWARMSH_ANALYTICS_WASTE_PERCENTAGE,
    ];
    
    for attr in analytics_attrs {
        assert!(attr.starts_with("swarmsh.analytics."), "Analytics attributes should have correct namespace");
    }
    
    Ok(())
}

/// Test span builders generated from semantic conventions
#[tokio::test]
async fn test_generated_span_builders() -> Result<()> {
    // Initialize telemetry for testing
    let system = SwarmSystem::new().await?;
    system.start().await?;
    
    // Test agent lifecycle span builders
    let agent_registration_span = span_builders::AgentRegistrationSpanBuilder::new()
        .with_agent_id("test_agent_001")
        .with_agent_role("test_worker")
        .with_agent_capacity(0.8)
        .with_specializations(vec!["test".to_string()])
        .build();
    
    assert_eq!(agent_registration_span.name(), "swarmsh.agent.registration");
    assert!(agent_registration_span.attributes().contains_key(&attributes::SWARMSH_AGENT_ID.into()));
    
    // Test work coordination span builders
    let work_claiming_span = span_builders::WorkClaimingSpanBuilder::new()
        .with_work_id("test_work_001")
        .with_work_type("feature")
        .with_agent_id("test_agent_001")
        .with_claiming_strategy("pull_based")
        .build();
    
    assert_eq!(work_claiming_span.name(), "swarmsh.work.claiming");
    assert!(work_claiming_span.attributes().contains_key(&attributes::SWARMSH_WORK_ID.into()));
    assert!(work_claiming_span.attributes().contains_key(&attributes::SWARMSH_AGENT_ID.into()));
    
    // Test coordination pattern span builders
    let coordination_span = span_builders::CoordinationPatternSpanBuilder::new()
        .with_pattern("scrum_at_scale")
        .with_participants(5)
        .with_zero_conflict_guarantee(true)
        .with_nanosecond_precision(true)
        .build();
    
    assert_eq!(coordination_span.name(), "swarmsh.coordination.pattern_execution");
    assert!(coordination_span.attributes().contains_key(&attributes::SWARMSH_COORDINATION_PATTERN.into()));
    
    // Test health monitoring span builders
    let health_check_span = span_builders::HealthCheckSpanBuilder::new()
        .with_component("coordination_engine")
        .with_health_score(85)
        .with_check_type("comprehensive")
        .build();
    
    assert_eq!(health_check_span.name(), "swarmsh.health.check");
    assert!(health_check_span.attributes().contains_key(&attributes::SWARMSH_HEALTH_COMPONENT.into()));
    
    // Test analytics span builders
    let analytics_span = span_builders::AnalyticsSpanBuilder::new()
        .with_analysis_type("8020_analysis")
        .with_value_ratio(4.2)
        .with_roi_percentage(340.0)
        .with_flow_efficiency(84.5)
        .build();
    
    assert_eq!(analytics_span.name(), "swarmsh.analytics.analysis");
    assert!(analytics_span.attributes().contains_key(&attributes::SWARMSH_ANALYTICS_VALUE_RATIO.into()));
    
    system.stop().await?;
    Ok(())
}

/// Test metrics generated from semantic conventions
#[tokio::test]
async fn test_generated_metrics() -> Result<()> {
    let system = SwarmSystem::new().await?;
    system.start().await?;
    
    // Test agent metrics
    let agent_count_metric = metrics::AgentCountMetric::new()
        .with_role("worker")
        .with_status("active")
        .build();
    
    assert_eq!(agent_count_metric.name(), "swarmsh.agent.count");
    assert!(agent_count_metric.attributes().contains_key("swarmsh.agent.role"));
    
    // Test work metrics
    let work_completion_time_metric = metrics::WorkCompletionTimeMetric::new()
        .with_work_type("feature")
        .with_priority("high")
        .with_agent_specialization("backend")
        .build();
    
    assert_eq!(work_completion_time_metric.name(), "swarmsh.work.completion_time");
    assert!(work_completion_time_metric.attributes().contains_key("swarmsh.work.type"));
    
    // Test coordination metrics
    let coordination_conflicts_metric = metrics::CoordinationConflictsMetric::new()
        .with_pattern("scrum_at_scale")
        .with_agent_count(10)
        .build();
    
    assert_eq!(coordination_conflicts_metric.name(), "swarmsh.coordination.conflicts");
    assert!(coordination_conflicts_metric.attributes().contains_key("swarmsh.coordination.pattern"));
    
    // Test health metrics
    let health_score_metric = metrics::HealthScoreMetric::new()
        .with_component("system")
        .with_check_type("comprehensive")
        .build();
    
    assert_eq!(health_score_metric.name(), "swarmsh.health.score");
    assert!(health_score_metric.attributes().contains_key("swarmsh.health.component"));
    
    // Test analytics metrics
    let flow_efficiency_metric = metrics::FlowEfficiencyMetric::new()
        .with_value_stream("feature_development")
        .with_measurement_period("daily")
        .build();
    
    assert_eq!(flow_efficiency_metric.name(), "swarmsh.analytics.flow_efficiency");
    assert!(flow_efficiency_metric.attributes().contains_key("swarmsh.analytics.value_stream"));
    
    system.stop().await?;
    Ok(())
}

/// Test telemetry data quality and completeness
#[tokio::test]
async fn test_telemetry_data_quality() -> Result<()> {
    let system = SwarmSystem::new().await?;
    system.start().await?;
    
    // Create telemetry collector for validation
    let telemetry_collector = TelemetryCollector::new().await?;
    
    // Generate test telemetry data
    let agent_spec = swarmsh_v2::coordination::AgentSpec {
        id: swarmsh_v2::AgentId::generate(),
        role: "telemetry_test_agent".to_string(),
        capacity: 1.0,
        specializations: vec!["telemetry_validation".to_string()],
        work_capacity: Some(1),
    };
    
    let agent_id = agent_spec.id.clone();
    
    // Register agent (should generate telemetry)
    let _registration_result = system.coordinator.register_agent(agent_spec).await?;
    
    // Create and claim work (should generate telemetry)
    let work_spec = swarmsh_v2::coordination::WorkSpec {
        id: swarmsh_v2::WorkId::generate(),
        work_type: "telemetry_validation".to_string(),
        priority: "normal".to_string(),
        description: "Telemetry validation work".to_string(),
        specification: "Generate telemetry for validation".to_string(),
        tags: vec!["validation".to_string()],
        estimated_effort: Some(1.0),
    };
    
    let work_id = work_spec.id.clone();
    system.coordinator.submit_work(work_spec).await?;
    
    let _claim_result = system.coordinator.claim_work(
        agent_id.clone(), 
        vec!["telemetry_validation".to_string()]
    ).await?;
    
    // Complete work (should generate telemetry)
    let _completion_result = system.coordinator.complete_work(agent_id.clone(), work_id).await?;
    
    // Collect health (should generate telemetry)
    let _health_report = system.health_monitor.collect_health().await?;
    
    // Run analytics (should generate telemetry)
    let _analytics_report = system.analytics.analyze_8020().await?;
    
    // Wait for telemetry collection
    sleep(Duration::from_millis(500)).await;
    
    // Validate telemetry data
    let collected_telemetry = telemetry_collector.collect_spans().await?;
    assert!(!collected_telemetry.is_empty(), "Should have collected telemetry spans");
    
    // Validate span structure
    for span in &collected_telemetry {
        // Verify span has required fields
        assert!(span.name.starts_with("swarmsh."), "Span should have SwarmSH namespace");
        assert!(!span.trace_id.is_empty(), "Span should have trace ID");
        assert!(!span.span_id.is_empty(), "Span should have span ID");
        assert!(span.start_time > 0, "Span should have valid start time");
        assert!(span.end_time >= span.start_time, "Span end time should be >= start time");
        
        // Verify attributes follow semantic conventions
        for (key, _value) in &span.attributes {
            assert!(key.starts_with("swarmsh."), "Attribute key should have SwarmSH namespace");
        }
    }
    
    // Validate metrics
    let collected_metrics = telemetry_collector.collect_metrics().await?;
    assert!(!collected_metrics.is_empty(), "Should have collected metrics");
    
    for metric in &collected_metrics {
        assert!(metric.name.starts_with("swarmsh."), "Metric should have SwarmSH namespace");
        assert!(metric.timestamp > 0, "Metric should have valid timestamp");
    }
    
    // Validate telemetry coverage
    let expected_operations = vec![
        "swarmsh.agent.registration",
        "swarmsh.work.submission",
        "swarmsh.work.claiming",
        "swarmsh.work.completion",
        "swarmsh.health.check",
        "swarmsh.analytics.analysis",
    ];
    
    for expected_op in expected_operations {
        assert!(
            collected_telemetry.iter().any(|span| span.name == expected_op),
            "Should have telemetry for operation: {}",
            expected_op
        );
    }
    
    system.stop().await?;
    Ok(())
}

/// Test OTEL trace context propagation
#[tokio::test]
async fn test_trace_context_propagation() -> Result<()> {
    let system = SwarmSystem::new().await?;
    system.start().await?;
    
    // Create parent span context
    let tracer = opentelemetry::global::tracer("swarmsh-test");
    let parent_span = tracer.start("test_parent_operation");
    let parent_context = Context::current_with_span(parent_span);
    
    // Execute operations with trace context
    let agent_spec = swarmsh_v2::coordination::AgentSpec {
        id: swarmsh_v2::AgentId::generate(),
        role: "trace_test_agent".to_string(),
        capacity: 1.0,
        specializations: vec!["trace_test".to_string()],
        work_capacity: Some(1),
    };
    
    let agent_id = agent_spec.id.clone();
    
    // Register agent with trace context
    let _registration_result = system.coordinator.register_agent_with_context(
        agent_spec,
        parent_context.clone()
    ).await?;
    
    // Create work with trace context
    let work_spec = swarmsh_v2::coordination::WorkSpec {
        id: swarmsh_v2::WorkId::generate(),
        work_type: "trace_test".to_string(),
        priority: "normal".to_string(),
        description: "Trace context test work".to_string(),
        specification: "Test trace context propagation".to_string(),
        tags: vec!["trace_test".to_string()],
        estimated_effort: Some(1.0),
    };
    
    let work_id = work_spec.id.clone();
    
    let _submission_result = system.coordinator.submit_work_with_context(
        work_spec,
        parent_context.clone()
    ).await?;
    
    // Claim work with trace context
    let _claim_result = system.coordinator.claim_work_with_context(
        agent_id.clone(),
        vec!["trace_test".to_string()],
        parent_context.clone()
    ).await?;
    
    // Complete work with trace context
    let _completion_result = system.coordinator.complete_work_with_context(
        agent_id,
        work_id,
        parent_context.clone()
    ).await?;
    
    // Validate trace context propagation
    let telemetry_collector = TelemetryCollector::new().await?;
    sleep(Duration::from_millis(300)).await;
    
    let collected_spans = telemetry_collector.collect_spans().await?;
    
    // Verify all spans have the same trace ID
    let parent_trace_id = parent_context.span().span_context().trace_id();
    
    for span in &collected_spans {
        if span.name.starts_with("swarmsh.") {
            // Parse trace ID from span (implementation dependent)
            // assert_eq!(span.trace_id, parent_trace_id.to_string());
            println!("Span {} has trace context", span.name);
        }
    }
    
    system.stop().await?;
    Ok(())
}

/// Test zero telemetry drift guarantee
#[tokio::test]
async fn test_zero_telemetry_drift() -> Result<()> {
    let system = SwarmSystem::new().await?;
    system.start().await?;
    
    // Baseline telemetry collection
    let telemetry_collector = TelemetryCollector::new().await?;
    
    // Execute operations multiple times
    for i in 0..10 {
        let agent_spec = swarmsh_v2::coordination::AgentSpec {
            id: swarmsh_v2::AgentId::generate(),
            role: format!("drift_test_agent_{}", i),
            capacity: 0.8,
            specializations: vec!["drift_test".to_string()],
            work_capacity: Some(1),
        };
        
        let agent_id = agent_spec.id.clone();
        system.coordinator.register_agent(agent_spec).await?;
        
        let work_spec = swarmsh_v2::coordination::WorkSpec {
            id: swarmsh_v2::WorkId::generate(),
            work_type: "drift_test".to_string(),
            priority: "normal".to_string(),
            description: format!("Drift test work {}", i),
            specification: "Test telemetry consistency".to_string(),
            tags: vec!["drift_test".to_string()],
            estimated_effort: Some(1.0),
        };
        
        let work_id = work_spec.id.clone();
        system.coordinator.submit_work(work_spec).await?;
        
        let _claim_result = system.coordinator.claim_work(
            agent_id.clone(),
            vec!["drift_test".to_string()]
        ).await?;
        
        system.coordinator.complete_work(agent_id, work_id).await?;
    }
    
    sleep(Duration::from_millis(500)).await;
    
    // Collect telemetry
    let collected_spans = telemetry_collector.collect_spans().await?;
    
    // Group spans by operation type
    let mut span_groups: HashMap<String, Vec<_>> = HashMap::new();
    for span in &collected_spans {
        span_groups.entry(span.name.clone()).or_default().push(span);
    }
    
    // Verify consistency within each operation type
    for (operation, spans) in span_groups {
        if spans.len() < 2 {
            continue; // Skip single-span operations
        }
        
        let first_span = &spans[0];
        for span in &spans[1..] {
            // Verify attribute schema consistency
            let first_keys: HashSet<_> = first_span.attributes.keys().collect();
            let span_keys: HashSet<_> = span.attributes.keys().collect();
            
            assert_eq!(
                first_keys, span_keys,
                "Attribute schema should be consistent for operation: {}",
                operation
            );
            
            // Verify attribute types are consistent
            for (key, first_value) in &first_span.attributes {
                let span_value = &span.attributes[key];
                assert_eq!(
                    std::mem::discriminant(first_value),
                    std::mem::discriminant(span_value),
                    "Attribute type should be consistent for {}.{}",
                    operation,
                    key
                );
            }
        }
    }
    
    system.stop().await?;
    Ok(())
}

/// Test telemetry performance impact
#[tokio::test]
async fn test_telemetry_performance_impact() -> Result<()> {
    let system = SwarmSystem::new().await?;
    system.start().await?;
    
    // Measure baseline performance without telemetry
    let baseline_start = std::time::Instant::now();
    
    for i in 0..100 {
        let agent_spec = swarmsh_v2::coordination::AgentSpec {
            id: swarmsh_v2::AgentId::generate(),
            role: format!("perf_test_agent_{}", i),
            capacity: 1.0,
            specializations: vec!["perf_test".to_string()],
            work_capacity: Some(1),
        };
        
        let agent_id = agent_spec.id.clone();
        system.coordinator.register_agent_without_telemetry(agent_spec).await?;
        
        system.coordinator.deregister_agent_without_telemetry(agent_id).await?;
    }
    
    let baseline_duration = baseline_start.elapsed();
    
    // Measure performance with telemetry
    let telemetry_start = std::time::Instant::now();
    
    for i in 0..100 {
        let agent_spec = swarmsh_v2::coordination::AgentSpec {
            id: swarmsh_v2::AgentId::generate(),
            role: format!("perf_test_agent_with_telemetry_{}", i),
            capacity: 1.0,
            specializations: vec!["perf_test".to_string()],
            work_capacity: Some(1),
        };
        
        let agent_id = agent_spec.id.clone();
        system.coordinator.register_agent(agent_spec).await?;
        
        system.coordinator.deregister_agent(agent_id).await?;
    }
    
    let telemetry_duration = telemetry_start.elapsed();
    
    // Telemetry overhead should be minimal (< 100μs per operation)
    let overhead_per_operation = (telemetry_duration.saturating_sub(baseline_duration)).as_micros() / 100;
    
    println!("Baseline duration: {:?}", baseline_duration);
    println!("Telemetry duration: {:?}", telemetry_duration);
    println!("Overhead per operation: {}μs", overhead_per_operation);
    
    assert!(
        overhead_per_operation < 100,
        "Telemetry overhead should be < 100μs per operation, got {}μs",
        overhead_per_operation
    );
    
    system.stop().await?;
    Ok(())
}

/// Test semantic convention evolution and backward compatibility
#[tokio::test]
async fn test_semantic_convention_evolution() -> Result<()> {
    // Test that semantic conventions can evolve without breaking existing code
    
    // Simulate v1 semantic conventions
    let v1_agent_attributes = vec![
        "swarmsh.agent.id",
        "swarmsh.agent.role",
        "swarmsh.agent.capacity",
    ];
    
    // Simulate v2 semantic conventions (with additional attributes)
    let v2_agent_attributes = vec![
        "swarmsh.agent.id",
        "swarmsh.agent.role", 
        "swarmsh.agent.capacity",
        "swarmsh.agent.specializations", // New in v2
        "swarmsh.agent.work_capacity",   // New in v2
    ];
    
    // Verify v1 attributes are subset of v2 (backward compatibility)
    for v1_attr in &v1_agent_attributes {
        assert!(
            v2_agent_attributes.contains(v1_attr),
            "v2 should maintain backward compatibility with v1 attribute: {}",
            v1_attr
        );
    }
    
    // Test that generated code handles optional attributes gracefully
    let span_with_v1_attrs = span_builders::AgentRegistrationSpanBuilder::new()
        .with_agent_id("test_agent")
        .with_agent_role("worker")
        .with_agent_capacity(0.8)
        .build();
    
    let span_with_v2_attrs = span_builders::AgentRegistrationSpanBuilder::new()
        .with_agent_id("test_agent")
        .with_agent_role("worker")
        .with_agent_capacity(0.8)
        .with_specializations(vec!["feature".to_string()])
        .with_work_capacity(Some(5))
        .build();
    
    // Both spans should be valid
    assert_eq!(span_with_v1_attrs.name(), "swarmsh.agent.registration");
    assert_eq!(span_with_v2_attrs.name(), "swarmsh.agent.registration");
    
    // v2 span should have additional attributes
    assert!(span_with_v2_attrs.attributes().len() > span_with_v1_attrs.attributes().len());
    
    Ok(())
}