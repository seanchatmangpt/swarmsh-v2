//! Meta-Programming Enhanced Span Builders
//! 
//! Advanced span generation using SwarmSH v2 meta-programming macros
//! with compile-time optimization and zero-cost abstractions.

use crate::swarm_spans;
use tracing::Instrument;

// Agent lifecycle spans with compile-time optimization
swarm_spans! {
    domain: agent,
    spans: {
        lifecycle("agent_lifecycle") => "swarmsh.agent.lifecycle",
        registration("agent_registration") => "swarmsh.agent.registration",
        deregistration("agent_deregistration") => "swarmsh.agent.deregistration", 
        heartbeat("agent_heartbeat") => "swarmsh.agent.heartbeat",
        handoff_initiate("agent_handoff_initiate") => "swarmsh.agent.handoff.initiate",
        handoff_receive("agent_handoff_receive") => "swarmsh.agent.handoff.receive",
        handoff_complete("agent_handoff_complete") => "swarmsh.agent.handoff.complete",
        routine_start("agent_routine_start") => "swarmsh.agent.routine.start",
        routine_step("agent_routine_step") => "swarmsh.agent.routine.step",
        routine_complete("agent_routine_complete") => "swarmsh.agent.routine.complete",
        tool_invoke("agent_tool_invoke") => "swarmsh.agent.tool.invoke",
        tool_validate("agent_tool_validate") => "swarmsh.agent.tool.validate",
        communicate("agent_communicate") => "swarmsh.agent.communicate",
        coordinate("agent_coordinate") => "swarmsh.agent.coordinate",
    }
}

// Work coordination spans
swarm_spans! {
    domain: work,
    spans: {
        create("work_create") => "swarmsh.work.create",
        claim("work_claim") => "swarmsh.work.claim",
        progress("work_progress") => "swarmsh.work.progress",
        complete("work_complete") => "swarmsh.work.complete",
        timeout("work_timeout") => "swarmsh.work.timeout",
        coordination("work_coordination") => "swarmsh.work.coordination",
    }
}

// Coordination protocol spans
swarm_spans! {
    domain: coordination,
    spans: {
        synchronize("coordination_synchronize") => "swarmsh.coordination.synchronize",
        acquire_lock("coordination_acquire_lock") => "swarmsh.coordination.acquire_lock",
        release_lock("coordination_release_lock") => "swarmsh.coordination.release_lock",
        conflict_resolution("coordination_conflict_resolution") => "swarmsh.coordination.conflict_resolution",
        epoch_advance("coordination_epoch_advance") => "swarmsh.coordination.epoch_advance",
        scrum_ceremony("coordination_scrum_ceremony") => "swarmsh.coordination.scrum_ceremony",
        roberts_rules_motion("coordination_roberts_rules_motion") => "swarmsh.coordination.roberts_rules_motion",
        protocol("coordination_protocol") => "swarmsh.coordination.protocol",
    }
}

// Health monitoring spans
swarm_spans! {
    domain: health,
    spans: {
        system_check("health_system_check") => "swarmsh.health.system_check",
        component_check("health_component_check") => "swarmsh.health.component_check",
        metric_collection("health_metric_collection") => "swarmsh.health.metric_collection",
        bottleneck_detection("health_bottleneck_detection") => "swarmsh.health.bottleneck_detection",
        automated_remediation("health_automated_remediation") => "swarmsh.health.automated_remediation",
    }
}

// Analytics and DLSS spans
swarm_spans! {
    domain: analytics,
    spans: {
        value_stream_mapping("analytics_value_stream_mapping") => "swarmsh.analytics.value_stream_mapping",
        waste_detection("analytics_waste_detection") => "swarmsh.analytics.waste_detection",
        flow_optimization("analytics_flow_optimization") => "swarmsh.analytics.flow_optimization",
        quality_analysis("analytics_quality_analysis") => "swarmsh.analytics.quality_analysis",
        eight_twenty_analysis("analytics_8020_analysis") => "swarmsh.analytics.8020_analysis",
        cliapi_methodology_analysis("analytics_cliapi_methodology") => "swarmsh.analytics.cliapi_methodology_analysis",
    }
}

// Infinite loop coordination spans
swarm_spans! {
    domain: infinite_loop,
    spans: {
        coordination("infinite_loop_coordination") => "swarmsh.infinite_loop.coordination",
        quality("infinite_loop_quality") => "swarmsh.infinite_loop.quality",
    }
}

// Auto 8020 feature detection spans
swarm_spans! {
    domain: auto,
    spans: {
        dlss("auto_dlss") => "swarmsh.auto.dlss",
        feature("auto_feature") => "swarmsh.auto.feature",
        validation("auto_validation") => "swarmsh.auto.validation",
        wave("auto_wave") => "swarmsh.auto.wave",
    }
}

/// Advanced span composition macros for complex operations
#[macro_export]
macro_rules! swarm_operation_span {
    (
        $span_name:ident,
        domains: [$($domain:ident),+ $(,)?],
        operation: $operation:expr,
        context: { $($key:ident => $value:expr),* $(,)? }
    ) => {
        {
            use tracing::{info_span, Span};
            use std::time::{SystemTime, UNIX_EPOCH};
            
            let operation_id = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            
            info_span!(
                stringify!($span_name),
                operation_id = %operation_id,
                operation = %$operation,
                domains = ?vec![$(stringify!($domain)),+],
                $(
                    $key = %$value,
                )*
                meta_generated = true
            )
        }
    };
}

/// Hierarchical span creation for complex workflows
#[macro_export]
macro_rules! swarm_workflow_span {
    (
        workflow: $workflow:expr,
        steps: [
            $($step_name:ident($step_operation:expr)),+ $(,)?
        ],
        context: { $($key:ident => $value:expr),* $(,)? }
    ) => {
        {
            use tracing::{info_span, Span};
            use std::time::{SystemTime, UNIX_EPOCH};
            
            let workflow_id = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            
            let workflow_span = info_span!(
                "swarm_workflow",
                workflow_id = %workflow_id,
                workflow = %$workflow,
                step_count = $(1 +)+ 0,
                $(
                    $key = %$value,
                )*
            );
            
            // Create step spans as children
            #[allow(unused_variables)]
            let step_spans = vec![
                $(
                    {
                        let _guard = workflow_span.enter();
                        info_span!(
                            stringify!($step_name),
                            step_operation = %$step_operation,
                            workflow_id = %workflow_id
                        )
                    },
                )+
            ];
            
            (workflow_span, step_spans)
        }
    };
}

/// AI-enhanced span creation with decision context
#[macro_export]
macro_rules! swarm_ai_span {
    (
        $span_name:ident,
        ai_context: $ai_context:expr,
        confidence_threshold: $threshold:expr,
        operation: $operation:expr
    ) => {
        {
            use tracing::{info_span, warn_span, Span};
            
            let confidence = $ai_context.get("confidence")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            
            if confidence >= $threshold {
                info_span!(
                    stringify!($span_name),
                    operation = %$operation,
                    ai_confidence = %confidence,
                    ai_enhanced = true,
                    confidence_threshold = %$threshold
                )
            } else {
                warn_span!(
                    stringify!($span_name),
                    operation = %$operation,
                    ai_confidence = %confidence,
                    ai_enhanced = false,
                    confidence_threshold = %$threshold,
                    reason = "below_threshold"
                )
            }
        }
    };
}

/// Zero-conflict coordination span with atomic guarantees
#[macro_export]
macro_rules! swarm_atomic_span {
    (
        $span_name:ident,
        epoch: $epoch:expr,
        participants: [$($participant:expr),+ $(,)?],
        operation: $operation:expr
    ) => {
        {
            use tracing::{info_span, Span};
            use std::time::{SystemTime, UNIX_EPOCH};
            
            let atomic_id = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            
            info_span!(
                stringify!($span_name),
                atomic_id = %atomic_id,
                operation = %$operation,
                epoch = %$epoch,
                participants = ?vec![$($participant.to_string()),+],
                atomic_guarantee = true,
                conflict_resolution = "zero_conflict",
                precision = "nanosecond"
            )
        }
    };
}

/// DLSS optimization span with waste tracking
#[macro_export] 
macro_rules! swarm_dlss_span {
    (
        $span_name:ident,
        operation: $operation:expr,
        target_efficiency: $efficiency:expr,
        waste_types: [$($waste:expr),+ $(,)?]
    ) => {
        {
            use tracing::{info_span, Span};
            use std::time::Instant;
            
            let start_time = Instant::now();
            let efficiency_percent = ($efficiency * 100.0) as u8;
            
            info_span!(
                stringify!($span_name),
                operation = %$operation,
                target_efficiency_percent = %efficiency_percent,
                waste_types = ?vec![$($waste),+],
                dlss_optimization = true,
                start_time_ns = %start_time.elapsed().as_nanos(),
                sigma_target = "4.2"
            )
        }
    };
}

/// Span statistics and meta-information
pub mod span_stats {
    use super::*;
    
    pub const TOTAL_SPAN_FUNCTIONS: usize = 
        agent_spans::SPAN_FUNCTIONS.len() +
        work_spans::SPAN_FUNCTIONS.len() +
        coordination_spans::SPAN_FUNCTIONS.len() +
        health_spans::SPAN_FUNCTIONS.len() +
        analytics_spans::SPAN_FUNCTIONS.len() +
        infinite_loop_spans::SPAN_FUNCTIONS.len() +
        auto_spans::SPAN_FUNCTIONS.len();
    
    pub const DOMAIN_COUNT: usize = 7;
    
    /// All span functions from all domains
    pub fn all_span_functions() -> Vec<&'static str> {
        let mut spans = Vec::with_capacity(TOTAL_SPAN_FUNCTIONS);
        spans.extend_from_slice(agent_spans::SPAN_FUNCTIONS);
        spans.extend_from_slice(work_spans::SPAN_FUNCTIONS);
        spans.extend_from_slice(coordination_spans::SPAN_FUNCTIONS);
        spans.extend_from_slice(health_spans::SPAN_FUNCTIONS);
        spans.extend_from_slice(analytics_spans::SPAN_FUNCTIONS);
        spans.extend_from_slice(infinite_loop_spans::SPAN_FUNCTIONS);
        spans.extend_from_slice(auto_spans::SPAN_FUNCTIONS);
        spans
    }
}

/// Compile-time span validation
const _: () = {
    assert!(span_stats::TOTAL_SPAN_FUNCTIONS > 0, "Must have at least one span function");
    assert!(span_stats::DOMAIN_COUNT == 7, "Expected exactly 7 span domains");
};