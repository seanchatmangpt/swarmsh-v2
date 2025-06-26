//! Meta-Programming Enhanced Attribute Generation
//! 
//! Uses SwarmSH v2 meta-programming macros to generate OTEL attributes
//! with compile-time optimization and validation.

use crate::swarm_attributes;

// Agent domain attributes with compile-time generation
swarm_attributes! {
    domain: agent,
    group: "swarmsh.agent",
    attributes: {
        agent_id => "agent.id",
        agent_role => "agent.role", 
        agent_status => "agent.status",
        agent_capacity => "agent.capacity",
        agent_framework_pattern => "agent.framework_pattern",
        agent_coordination_epoch => "agent.coordination_epoch",
        agent_hierarchy_level => "agent.hierarchy.level",
        agent_specializations => "agent.specializations",
        agent_work_capacity => "agent.work_capacity",
        agent_last_heartbeat => "agent.last_heartbeat",
        agent_communication_message_type => "agent.communication.message_type",
        agent_handoff_source => "agent.handoff.source",
        agent_handoff_target => "agent.handoff.target",
        agent_handoff_context_id => "agent.handoff.context_id",
        agent_handoff_reason => "agent.handoff.reason",
        agent_routine_name => "agent.routine.name",
        agent_routine_step => "agent.routine.step",
        agent_routine_step_name => "agent.routine.step_name",
        agent_tool_name => "agent.tool.name",
        agent_tool_execution_result => "agent.tool.execution_result",
        agent_tool_validation_status => "agent.tool.validation_status",
    }
}

// Work coordination attributes  
swarm_attributes! {
    domain: work,
    group: "swarmsh.work",
    attributes: {
        work_id => "work.id",
        work_type => "work.type",
        work_status => "work.status",
        work_priority => "work.priority",
        work_description => "work.description",
        work_specification => "work.specification",
        work_result => "work.result",
        work_created_at => "work.created_at",
        work_completed_at => "work.completed_at",
        work_claimed_by => "work.claimed_by",
        work_claim_timestamp_ns => "work.claim_timestamp_ns",
        work_estimated_duration_ms => "work.estimated_duration_ms",
        work_progress_percentage => "work.progress_percentage",
        work_tags => "work.tags",
    }
}

// Coordination protocol attributes
swarm_attributes! {
    domain: coordination,
    group: "swarmsh.coordination",
    attributes: {
        coordination_id => "coordination.coordination_id",
        coordination_operation => "coordination.operation",
        coordination_pattern => "coordination.pattern",
        coordination_epoch => "coordination.epoch",
        coordination_participants => "coordination.participants",
        coordination_agent_count => "coordination.agent_count",
        coordination_success => "coordination.success",
        coordination_error_type => "coordination.error_type",
        coordination_retry_count => "coordination.retry_count",
        coordination_atomic_operation => "coordination.atomic_operation",
        coordination_lock_acquired => "coordination.lock_acquired",
        coordination_lock_duration_ms => "coordination.lock_duration_ms",
        coordination_conflict_detected => "coordination.conflict_detected",
        coordination_scrum_context => "coordination.scrum_context",
        coordination_roberts_rules_action => "coordination.roberts_rules_action",
    }
}

// Health monitoring attributes
swarm_attributes! {
    domain: health,
    group: "swarmsh.health",
    attributes: {
        health_status => "health.status",
        health_score => "health.score",
        health_component => "health.component",
        health_check_duration_ms => "health.check_duration_ms",
        health_bottleneck => "health.bottleneck",
        health_recommendation => "health.recommendation",
        health_automated_action => "health.automated_action",
        health_monitoring_tier => "health.monitoring_tier",
        health_threshold_warning => "health.threshold_warning",
        health_threshold_critical => "health.threshold_critical",
        health_metric_name => "health.metric_name",
        health_metric_value => "health.metric_value",
        health_trend => "health.trend",
    }
}

// Analytics and DLSS attributes
swarm_attributes! {
    domain: analytics,
    group: "swarmsh.analytics",
    attributes: {
        analytics_tier => "analytics.tier",
        analytics_optimization_type => "analytics.optimization_type",
        analytics_value_ratio => "analytics.value_ratio",
        analytics_waste_type => "analytics.waste_type",
        analytics_waste_percentage => "analytics.waste_percentage",
        analytics_flow_efficiency => "analytics.flow_efficiency",
        analytics_throughput => "analytics.throughput",
        analytics_lead_time_ms => "analytics.lead_time_ms",
        analytics_cycle_time_ms => "analytics.cycle_time_ms",
        analytics_defect_rate_dpmo => "analytics.defect_rate_dpmo",
        analytics_sigma_level => "analytics.sigma_level",
        analytics_roi_percentage => "analytics.roi_percentage",
        analytics_baseline_value => "analytics.baseline_value",
        analytics_current_value => "analytics.current_value",
        analytics_recommendation => "analytics.recommendation",
        analytics_bottleneck => "analytics.bottleneck",
        analytics_cliapi_integration => "analytics.cliapi_integration",
    }
}

// Infinite loop coordination attributes
swarm_attributes! {
    domain: infinite_loop,
    group: "swarmsh.infinite_loop",
    attributes: {
        loop_id => "loop_id",
        loop_mode => "loop_mode",
        iteration_count => "iteration_count",
        coordination_strategy => "coordination_strategy",
        conflict_resolution => "conflict_resolution",
        quality_score => "quality_score",
        validation_method => "validation_method",
        specification_path => "specification_path",
        output_directory => "output_directory",
    }
}

// Auto 8020 feature detection attributes
swarm_attributes! {
    domain: auto,
    group: "swarmsh.auto",
    attributes: {
        feature_id => "feature_id",
        feature_name => "feature_name",
        analysis_id => "analysis_id",
        project_path => "project_path",
        features_detected => "features_detected",
        features_selected => "features_selected",
        impact_score => "impact_score",
        complexity_score => "complexity_score",
        value_ratio => "value_ratio",
        implementation_status => "implementation_status",
        quality_gate_passed => "quality_gate_passed",
        rollback_triggered => "rollback_triggered",
        validation_type => "validation_type",
        performance_impact => "performance_impact",
        coordination_method => "coordination_method",
        parallelism_factor => "parallelism_factor",
        wave_id => "wave_id",
        wave_completion_rate => "wave_completion_rate",
        bottleneck_identified => "bottleneck_identified",
        defect_density => "defect_density",
        flow_efficiency => "flow_efficiency",
        sigma_level => "sigma_level",
        waste_type => "waste_type",
        value_score_threshold => "value_score_threshold",
    }
}

/// Compile-time attribute statistics
pub mod stats {
    use super::*;
    
    pub const TOTAL_ATTRIBUTES: usize = 
        agent_attributes::ATTRIBUTE_COUNT +
        work_attributes::ATTRIBUTE_COUNT +
        coordination_attributes::ATTRIBUTE_COUNT +
        health_attributes::ATTRIBUTE_COUNT +
        analytics_attributes::ATTRIBUTE_COUNT +
        infinite_loop_attributes::ATTRIBUTE_COUNT +
        auto_attributes::ATTRIBUTE_COUNT;
    
    pub const DOMAIN_COUNT: usize = 7;
    
    /// All attribute domains
    pub const DOMAINS: &[&str] = &[
        "agent",
        "work", 
        "coordination",
        "health",
        "analytics",
        "infinite_loop",
        "auto"
    ];
    
    /// Get all attributes from all domains
    pub fn all_attributes() -> Vec<&'static str> {
        let mut attrs = Vec::with_capacity(TOTAL_ATTRIBUTES);
        attrs.extend_from_slice(agent_attributes::ALL_ATTRIBUTES);
        attrs.extend_from_slice(work_attributes::ALL_ATTRIBUTES);
        attrs.extend_from_slice(coordination_attributes::ALL_ATTRIBUTES);
        attrs.extend_from_slice(health_attributes::ALL_ATTRIBUTES);
        attrs.extend_from_slice(analytics_attributes::ALL_ATTRIBUTES);
        attrs.extend_from_slice(infinite_loop_attributes::ALL_ATTRIBUTES);
        attrs.extend_from_slice(auto_attributes::ALL_ATTRIBUTES);
        attrs
    }
}

/// Compile-time validation
const _: () = {
    assert!(stats::TOTAL_ATTRIBUTES > 0, "Must have at least one attribute");
    assert!(stats::DOMAIN_COUNT == 7, "Expected exactly 7 domains");
};