//! Generated attribute constants for SWARMSH v2 telemetry
//! 
//! This code is generated by OTEL Weaver from semantic conventions.
//! DO NOT EDIT MANUALLY.

/// Agent attributes
pub mod agent {
    pub const ID: &str = "swarmsh.agent.id";
    pub const ROLE: &str = "swarmsh.agent.role";
    pub const STATUS: &str = "swarmsh.agent.status";
    pub const CAPACITY: &str = "swarmsh.agent.capacity";
    pub const SPECIALIZATIONS: &str = "swarmsh.agent.specializations";
}

/// Coordination attributes
pub mod coordination {
    pub const PATTERN: &str = "swarmsh.coordination.pattern";
    pub const EPOCH: &str = "swarmsh.coordination.epoch";
    pub const PARTICIPANT_COUNT: &str = "swarmsh.coordination.participant_count";
    pub const LATENCY_MS: &str = "swarmsh.coordination.latency_ms";
    pub const CONFLICT_RESOLUTION: &str = "swarmsh.coordination.conflict_resolution";
}

/// Work attributes
pub mod work {
    pub const ID: &str = "swarmsh.work.id";
    pub const STATUS: &str = "swarmsh.work.status";
    pub const PRIORITY: &str = "swarmsh.work.priority";
    pub const ASSIGNED_AGENT: &str = "swarmsh.work.assigned_agent";
    pub const ESTIMATED_DURATION_MS: &str = "swarmsh.work.estimated_duration_ms";
}

/// Health attributes
pub mod health {
    pub const COMPONENT: &str = "swarmsh.health.component";
    pub const STATUS: &str = "swarmsh.health.status";
    pub const BOTTLENECK_DETECTED: &str = "swarmsh.health.bottleneck_detected";
    pub const TIER: &str = "swarmsh.health.tier";
}

/// Analytics attributes
pub mod analytics {
    pub const TIER: &str = "swarmsh.analytics.tier";
    pub const WASTE_TYPE: &str = "swarmsh.analytics.waste_type";
    pub const FLOW_EFFICIENCY: &str = "swarmsh.analytics.flow_efficiency";
    pub const OPTIMIZATION_TYPE: &str = "swarmsh.analytics.optimization_type";
    pub const SIGMA_LEVEL: &str = "swarmsh.analytics.sigma_level";
}

/// AI attributes
pub mod ai {
    pub const PROVIDER: &str = "swarmsh.ai.provider";
    pub const MODEL: &str = "swarmsh.ai.model";
    pub const DECISION_TYPE: &str = "swarmsh.ai.decision_type";
    pub const CONFIDENCE: &str = "swarmsh.ai.confidence";
    pub const STREAMING: &str = "swarmsh.ai.streaming";
}
