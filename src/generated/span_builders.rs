// Generated span builders from SwarmSH v2 semantic conventions
// Simplified for compatibility

/// Create agent lifecycle span using tracing macros
#[macro_export]
macro_rules! agent_lifecycle_span {
    ($operation:expr) => {
        tracing::info_span!("swarmsh.agent.lifecycle", operation = %$operation)
    };
}

/// Create work coordination span using tracing macros
#[macro_export]
macro_rules! work_coordination_span {
    ($operation:expr) => {
        tracing::info_span!("swarmsh.work.coordination", operation = %$operation)
    };
}

/// Create coordination protocol span using tracing macros
#[macro_export]
macro_rules! coordination_protocol_span {
    ($operation:expr) => {
        tracing::info_span!("swarmsh.coordination.protocol", operation = %$operation)
    };
}

// Simplified span creation functions
pub fn create_swarmsh_agent_span(name: &str) -> tracing::Span {
    tracing::info_span!("swarmsh.agent", span_name = %name)
}

pub fn create_swarmsh_work_span(name: &str) -> tracing::Span {
    tracing::info_span!("swarmsh.work", span_name = %name)
}

pub fn create_swarmsh_coordination_span(name: &str) -> tracing::Span {
    tracing::info_span!("swarmsh.coordination", span_name = %name)
}

pub fn create_swarmsh_health_span(name: &str) -> tracing::Span {
    tracing::info_span!("swarmsh.health", span_name = %name)
}

pub fn create_swarmsh_analytics_span(name: &str) -> tracing::Span {
    tracing::info_span!("swarmsh.analytics", span_name = %name)
}

pub fn create_swarmsh_infinite_loop_span(name: &str) -> tracing::Span {
    tracing::info_span!("swarmsh.infinite_loop", span_name = %name)
}

pub fn create_swarmsh_worktree_span(name: &str) -> tracing::Span {
    tracing::info_span!("swarmsh.worktree", span_name = %name)
}

pub fn create_swarmsh_auto_span(name: &str) -> tracing::Span {
    tracing::info_span!("swarmsh.auto", span_name = %name)
}
