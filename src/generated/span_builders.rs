// Generated span builders from SwarmSH v2 semantic conventions

use opentelemetry::global::BoxedSpan;
use opentelemetry::trace::Tracer;

// Core span builders expected by lib.rs
pub fn agent_lifecycle_span(tracer: &impl Tracer) -> BoxedSpan {
    tracer.start("swarmsh.agent.lifecycle")
}

pub fn work_coordination_span(tracer: &impl Tracer) -> BoxedSpan {
    tracer.start("swarmsh.work.coordination")
}

pub fn coordination_protocol_span(tracer: &impl Tracer) -> BoxedSpan {
    tracer.start("swarmsh.coordination.protocol")
}

// Generated span builders for all groups
pub fn create_swarmsh_agent_span(tracer: &impl Tracer) -> BoxedSpan {
    tracer.start("swarmsh.agent")
}

pub fn create_swarmsh_work_span(tracer: &impl Tracer) -> BoxedSpan {
    tracer.start("swarmsh.work")
}

pub fn create_swarmsh_coordination_span(tracer: &impl Tracer) -> BoxedSpan {
    tracer.start("swarmsh.coordination")
}

pub fn create_swarmsh_health_span(tracer: &impl Tracer) -> BoxedSpan {
    tracer.start("swarmsh.health")
}

pub fn create_swarmsh_analytics_span(tracer: &impl Tracer) -> BoxedSpan {
    tracer.start("swarmsh.analytics")
}

pub fn create_swarmsh_infinite_loop_span(tracer: &impl Tracer) -> BoxedSpan {
    tracer.start("swarmsh.infinite_loop")
}

pub fn create_swarmsh_infinite_loop_quality_span(tracer: &impl Tracer) -> BoxedSpan {
    tracer.start("swarmsh.infinite_loop.quality")
}

pub fn create_swarmsh_infinite_loop_coordination_span(tracer: &impl Tracer) -> BoxedSpan {
    tracer.start("swarmsh.infinite_loop.coordination")
}

pub fn create_swarmsh_worktree_span(tracer: &impl Tracer) -> BoxedSpan {
    tracer.start("swarmsh.worktree")
}

pub fn create_swarmsh_worktree_lifecycle_span(tracer: &impl Tracer) -> BoxedSpan {
    tracer.start("swarmsh.worktree.lifecycle")
}

pub fn create_swarmsh_worktree_coordination_span(tracer: &impl Tracer) -> BoxedSpan {
    tracer.start("swarmsh.worktree.coordination")
}

pub fn create_swarmsh_worktree_ai_integration_span(tracer: &impl Tracer) -> BoxedSpan {
    tracer.start("swarmsh.worktree.ai_integration")
}

pub fn create_swarmsh_auto_span(tracer: &impl Tracer) -> BoxedSpan {
    tracer.start("swarmsh.auto")
}

pub fn create_swarmsh_auto_feature_span(tracer: &impl Tracer) -> BoxedSpan {
    tracer.start("swarmsh.auto.feature")
}

pub fn create_swarmsh_auto_dlss_span(tracer: &impl Tracer) -> BoxedSpan {
    tracer.start("swarmsh.auto.dlss")
}

pub fn create_swarmsh_auto_wave_span(tracer: &impl Tracer) -> BoxedSpan {
    tracer.start("swarmsh.auto.wave")
}

pub fn create_swarmsh_auto_validation_span(tracer: &impl Tracer) -> BoxedSpan {
    tracer.start("swarmsh.auto.validation")
}
