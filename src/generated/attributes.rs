// Generated from SwarmSH v2 semantic conventions
// 80/20 implementation - core attributes only

// Agent lifecycle and management operations in SwarmSH coordination system
pub mod swarmsh_agent {
    pub const AGENT_1719123456789012345: &str = "swarmsh.agent.agent_1719123456789012345";
    pub const AGENT_1719123456789012346: &str = "swarmsh.agent.agent_1719123456789012346";
    pub const AGENT_1719123456789012347: &str = "swarmsh.agent.agent_1719123456789012347";
    pub const HANDOFF_1719123456789012347: &str = "swarmsh.agent.handoff_1719123456789012347";
    pub const AGENT_CAPACITY: &str = "swarmsh.agent.agent.capacity";
    pub const AGENT_COMMUNICATION_MESSAGE_TYPE: &str = "swarmsh.agent.agent.communication.message_type";
    pub const AGENT_COORDINATION_EPOCH: &str = "swarmsh.agent.agent.coordination_epoch";
    pub const AGENT_FRAMEWORK_PATTERN: &str = "swarmsh.agent.agent.framework_pattern";
    pub const AGENT_HANDOFF_CONTEXT_ID: &str = "swarmsh.agent.agent.handoff.context_id";
    pub const AGENT_HANDOFF_REASON: &str = "swarmsh.agent.agent.handoff.reason";
    pub const AGENT_HANDOFF_SOURCE: &str = "swarmsh.agent.agent.handoff.source";
    pub const AGENT_HANDOFF_TARGET: &str = "swarmsh.agent.agent.handoff.target";
    pub const AGENT_HIERARCHY_LEVEL: &str = "swarmsh.agent.agent.hierarchy.level";
    pub const AGENT_ID: &str = "swarmsh.agent.agent.id";
    pub const AGENT_LAST_HEARTBEAT: &str = "swarmsh.agent.agent.last_heartbeat";
    pub const AGENT_ROLE: &str = "swarmsh.agent.agent.role";
    pub const AGENT_ROUTINE_NAME: &str = "swarmsh.agent.agent.routine.name";
    pub const AGENT_ROUTINE_STEP: &str = "swarmsh.agent.agent.routine.step";
    pub const AGENT_ROUTINE_STEP_NAME: &str = "swarmsh.agent.agent.routine.step_name";
    pub const AGENT_SPECIALIZATIONS: &str = "swarmsh.agent.agent.specializations";
    pub const AGENT_STATUS: &str = "swarmsh.agent.agent.status";
    pub const AGENT_TOOL_EXECUTION_RESULT: &str = "swarmsh.agent.agent.tool.execution_result";
    pub const AGENT_TOOL_NAME: &str = "swarmsh.agent.agent.tool.name";
    pub const AGENT_TOOL_VALIDATION_STATUS: &str = "swarmsh.agent.agent.tool.validation_status";
    pub const AGENT_WORK_CAPACITY: &str = "swarmsh.agent.agent.work_capacity";
    pub const SWARMSH_AGENT_COMMUNICATE: &str = "swarmsh.agent.swarmsh.agent.communicate";
    pub const SWARMSH_AGENT_COORDINATE: &str = "swarmsh.agent.swarmsh.agent.coordinate";
    pub const SWARMSH_AGENT_DEREGISTER: &str = "swarmsh.agent.swarmsh.agent.deregister";
    pub const SWARMSH_AGENT_HANDOFF_COMPLETE: &str = "swarmsh.agent.swarmsh.agent.handoff.complete";
    pub const SWARMSH_AGENT_HANDOFF_INITIATE: &str = "swarmsh.agent.swarmsh.agent.handoff.initiate";
    pub const SWARMSH_AGENT_HANDOFF_RECEIVE: &str = "swarmsh.agent.swarmsh.agent.handoff.receive";
    pub const SWARMSH_AGENT_HEARTBEAT: &str = "swarmsh.agent.swarmsh.agent.heartbeat";
    pub const SWARMSH_AGENT_REGISTER: &str = "swarmsh.agent.swarmsh.agent.register";
    pub const SWARMSH_AGENT_ROUTINE_COMPLETE: &str = "swarmsh.agent.swarmsh.agent.routine.complete";
    pub const SWARMSH_AGENT_ROUTINE_START: &str = "swarmsh.agent.swarmsh.agent.routine.start";
    pub const SWARMSH_AGENT_ROUTINE_STEP: &str = "swarmsh.agent.swarmsh.agent.routine.step";
    pub const SWARMSH_AGENT_TOOL_INVOKE: &str = "swarmsh.agent.swarmsh.agent.tool.invoke";
    pub const SWARMSH_AGENT_TOOL_VALIDATE: &str = "swarmsh.agent.swarmsh.agent.tool.validate";
}

// Work item coordination and execution operations
pub mod swarmsh_work {
    pub const WORK_1719123456789012345: &str = "swarmsh.work.work_1719123456789012345";
    pub const SWARMSH_WORK_CLAIM: &str = "swarmsh.work.swarmsh.work.claim";
    pub const SWARMSH_WORK_COMPLETE: &str = "swarmsh.work.swarmsh.work.complete";
    pub const SWARMSH_WORK_CREATE: &str = "swarmsh.work.swarmsh.work.create";
    pub const SWARMSH_WORK_PROGRESS: &str = "swarmsh.work.swarmsh.work.progress";
    pub const SWARMSH_WORK_TIMEOUT: &str = "swarmsh.work.swarmsh.work.timeout";
    pub const WORK_CLAIM_TIMESTAMP_NS: &str = "swarmsh.work.work.claim_timestamp_ns";
    pub const WORK_CLAIMED_BY: &str = "swarmsh.work.work.claimed_by";
    pub const WORK_COMPLETED_AT: &str = "swarmsh.work.work.completed_at";
    pub const WORK_CREATED_AT: &str = "swarmsh.work.work.created_at";
    pub const WORK_DESCRIPTION: &str = "swarmsh.work.work.description";
    pub const WORK_ESTIMATED_DURATION_MS: &str = "swarmsh.work.work.estimated_duration_ms";
    pub const WORK_ID: &str = "swarmsh.work.work.id";
    pub const WORK_PRIORITY: &str = "swarmsh.work.work.priority";
    pub const WORK_PROGRESS_PERCENTAGE: &str = "swarmsh.work.work.progress_percentage";
    pub const WORK_RESULT: &str = "swarmsh.work.work.result";
    pub const WORK_SPECIFICATION: &str = "swarmsh.work.work.specification";
    pub const WORK_STATUS: &str = "swarmsh.work.work.status";
    pub const WORK_TAGS: &str = "swarmsh.work.work.tags";
    pub const WORK_TYPE: &str = "swarmsh.work.work.type";
}

// Coordination protocol operations and conflict resolution
pub mod swarmsh_coordination {
    pub const COORD_1719123456789012345: &str = "swarmsh.coordination.coord_1719123456789012345";
    pub const COORD_1719123456789012346: &str = "swarmsh.coordination.coord_1719123456789012346";
    pub const COORD_1719123456789012347: &str = "swarmsh.coordination.coord_1719123456789012347";
    pub const COORD_1719123456789012348: &str = "swarmsh.coordination.coord_1719123456789012348";
    pub const COORDINATION_AGENT_COUNT: &str = "swarmsh.coordination.coordination.agent_count";
    pub const COORDINATION_ATOMIC_OPERATION: &str = "swarmsh.coordination.coordination.atomic_operation";
    pub const COORDINATION_CONFLICT_DETECTED: &str = "swarmsh.coordination.coordination.conflict_detected";
    pub const COORDINATION_COORDINATION_ID: &str = "swarmsh.coordination.coordination.coordination_id";
    pub const COORDINATION_EPOCH: &str = "swarmsh.coordination.coordination.epoch";
    pub const COORDINATION_ERROR_TYPE: &str = "swarmsh.coordination.coordination.error_type";
    pub const COORDINATION_LOCK_ACQUIRED: &str = "swarmsh.coordination.coordination.lock_acquired";
    pub const COORDINATION_LOCK_DURATION_MS: &str = "swarmsh.coordination.coordination.lock_duration_ms";
    pub const COORDINATION_OPERATION: &str = "swarmsh.coordination.coordination.operation";
    pub const COORDINATION_PARTICIPANTS: &str = "swarmsh.coordination.coordination.participants";
    pub const COORDINATION_PATTERN: &str = "swarmsh.coordination.coordination.pattern";
    pub const COORDINATION_RETRY_COUNT: &str = "swarmsh.coordination.coordination.retry_count";
    pub const COORDINATION_ROBERTS_RULES_ACTION: &str = "swarmsh.coordination.coordination.roberts_rules_action";
    pub const COORDINATION_SCRUM_CONTEXT: &str = "swarmsh.coordination.coordination.scrum_context";
    pub const COORDINATION_SUCCESS: &str = "swarmsh.coordination.coordination.success";
    pub const SWARMSH_COORDINATION_ACQUIRE_LOCK: &str = "swarmsh.coordination.swarmsh.coordination.acquire_lock";
    pub const SWARMSH_COORDINATION_CONFLICT_RESOLUTION: &str = "swarmsh.coordination.swarmsh.coordination.conflict_resolution";
    pub const SWARMSH_COORDINATION_EPOCH_ADVANCE: &str = "swarmsh.coordination.swarmsh.coordination.epoch_advance";
    pub const SWARMSH_COORDINATION_RELEASE_LOCK: &str = "swarmsh.coordination.swarmsh.coordination.release_lock";
    pub const SWARMSH_COORDINATION_ROBERTS_RULES_MOTION: &str = "swarmsh.coordination.swarmsh.coordination.roberts_rules_motion";
    pub const SWARMSH_COORDINATION_SCRUM_CEREMONY: &str = "swarmsh.coordination.swarmsh.coordination.scrum_ceremony";
    pub const SWARMSH_COORDINATION_SYNCHRONIZE: &str = "swarmsh.coordination.swarmsh.coordination.synchronize";
}

// System health monitoring and bottleneck detection operations
pub mod swarmsh_health {
    pub const HEALTH_AUTOMATED_ACTION: &str = "swarmsh.health.health.automated_action";
    pub const HEALTH_BOTTLENECK: &str = "swarmsh.health.health.bottleneck";
    pub const HEALTH_CHECK_DURATION_MS: &str = "swarmsh.health.health.check_duration_ms";
    pub const HEALTH_COMPONENT: &str = "swarmsh.health.health.component";
    pub const HEALTH_METRIC_NAME: &str = "swarmsh.health.health.metric_name";
    pub const HEALTH_METRIC_VALUE: &str = "swarmsh.health.health.metric_value";
    pub const HEALTH_MONITORING_TIER: &str = "swarmsh.health.health.monitoring_tier";
    pub const HEALTH_RECOMMENDATION: &str = "swarmsh.health.health.recommendation";
    pub const HEALTH_SCORE: &str = "swarmsh.health.health.score";
    pub const HEALTH_STATUS: &str = "swarmsh.health.health.status";
    pub const HEALTH_THRESHOLD_CRITICAL: &str = "swarmsh.health.health.threshold_critical";
    pub const HEALTH_THRESHOLD_WARNING: &str = "swarmsh.health.health.threshold_warning";
    pub const HEALTH_TREND: &str = "swarmsh.health.health.trend";
    pub const SWARMSH_HEALTH_AUTOMATED_REMEDIATION: &str = "swarmsh.health.swarmsh.health.automated_remediation";
    pub const SWARMSH_HEALTH_BOTTLENECK_DETECTION: &str = "swarmsh.health.swarmsh.health.bottleneck_detection";
    pub const SWARMSH_HEALTH_COMPONENT_CHECK: &str = "swarmsh.health.swarmsh.health.component_check";
    pub const SWARMSH_HEALTH_METRIC_COLLECTION: &str = "swarmsh.health.swarmsh.health.metric_collection";
    pub const SWARMSH_HEALTH_SYSTEM_CHECK: &str = "swarmsh.health.swarmsh.health.system_check";
}

// 8020 analysis, optimization, and value stream operations
pub mod swarmsh_analytics {
    pub const ANALYTICS_BASELINE_VALUE: &str = "swarmsh.analytics.analytics.baseline_value";
    pub const ANALYTICS_BOTTLENECK: &str = "swarmsh.analytics.analytics.bottleneck";
    pub const ANALYTICS_CLIAPI_INTEGRATION: &str = "swarmsh.analytics.analytics.cliapi_integration";
    pub const ANALYTICS_CURRENT_VALUE: &str = "swarmsh.analytics.analytics.current_value";
    pub const ANALYTICS_CYCLE_TIME_MS: &str = "swarmsh.analytics.analytics.cycle_time_ms";
    pub const ANALYTICS_DEFECT_RATE_DPMO: &str = "swarmsh.analytics.analytics.defect_rate_dpmo";
    pub const ANALYTICS_FLOW_EFFICIENCY: &str = "swarmsh.analytics.analytics.flow_efficiency";
    pub const ANALYTICS_LEAD_TIME_MS: &str = "swarmsh.analytics.analytics.lead_time_ms";
    pub const ANALYTICS_OPTIMIZATION_TYPE: &str = "swarmsh.analytics.analytics.optimization_type";
    pub const ANALYTICS_RECOMMENDATION: &str = "swarmsh.analytics.analytics.recommendation";
    pub const ANALYTICS_ROI_PERCENTAGE: &str = "swarmsh.analytics.analytics.roi_percentage";
    pub const ANALYTICS_SIGMA_LEVEL: &str = "swarmsh.analytics.analytics.sigma_level";
    pub const ANALYTICS_THROUGHPUT: &str = "swarmsh.analytics.analytics.throughput";
    pub const ANALYTICS_TIER: &str = "swarmsh.analytics.analytics.tier";
    pub const ANALYTICS_VALUE_RATIO: &str = "swarmsh.analytics.analytics.value_ratio";
    pub const ANALYTICS_WASTE_PERCENTAGE: &str = "swarmsh.analytics.analytics.waste_percentage";
    pub const ANALYTICS_WASTE_TYPE: &str = "swarmsh.analytics.analytics.waste_type";
    pub const SWARMSH_ANALYTICS_8020_ANALYSIS: &str = "swarmsh.analytics.swarmsh.analytics.8020_analysis";
    pub const SWARMSH_ANALYTICS_CLIAPI_METHODOLOGY_ANALYSIS: &str = "swarmsh.analytics.swarmsh.analytics.cliapi_methodology_analysis";
    pub const SWARMSH_ANALYTICS_FLOW_OPTIMIZATION: &str = "swarmsh.analytics.swarmsh.analytics.flow_optimization";
    pub const SWARMSH_ANALYTICS_QUALITY_ANALYSIS: &str = "swarmsh.analytics.swarmsh.analytics.quality_analysis";
    pub const SWARMSH_ANALYTICS_VALUE_STREAM_MAPPING: &str = "swarmsh.analytics.swarmsh.analytics.value_stream_mapping";
    pub const SWARMSH_ANALYTICS_WASTE_DETECTION: &str = "swarmsh.analytics.swarmsh.analytics.waste_detection";
}

// Infinite agentic loop coordination and execution
pub mod swarmsh_infinite_loop {
    pub const CONFLICT_RESOLUTION: &str = "swarmsh.infinite_loop.conflict_resolution";
    pub const COORDINATION_STRATEGY: &str = "swarmsh.infinite_loop.coordination_strategy";
    pub const ITERATION_COUNT: &str = "swarmsh.infinite_loop.iteration_count";
    pub const LOOP_ID: &str = "swarmsh.infinite_loop.loop_id";
    pub const LOOP_MODE: &str = "swarmsh.infinite_loop.loop_mode";
    pub const OUTPUT_DIRECTORY: &str = "swarmsh.infinite_loop.output_directory";
    pub const QUALITY_SCORE: &str = "swarmsh.infinite_loop.quality_score";
    pub const SPECIFICATION_PATH: &str = "swarmsh.infinite_loop.specification_path";
    pub const SWARMSH_INFINITE_LOOP_COORDINATION: &str = "swarmsh.infinite_loop.swarmsh.infinite_loop.coordination";
    pub const SWARMSH_INFINITE_LOOP_QUALITY: &str = "swarmsh.infinite_loop.swarmsh.infinite_loop.quality";
    pub const VALIDATION_METHOD: &str = "swarmsh.infinite_loop.validation_method";
}

// SwarmSH v2 Worktree Management Operations
pub mod swarmsh_worktree {
    pub const SWARMSH_WORKTREE_AGENT_COUNT: &str = "swarmsh.worktree.swarmsh.worktree.agent_count";
    pub const SWARMSH_WORKTREE_AGENT_UTILIZATION: &str = "swarmsh.worktree.swarmsh.worktree.agent.utilization";
    pub const SWARMSH_WORKTREE_AI_INTEGRATION: &str = "swarmsh.worktree.swarmsh.worktree.ai_integration";
    pub const SWARMSH_WORKTREE_AI_CONFIDENCE: &str = "swarmsh.worktree.swarmsh.worktree.ai.confidence";
    pub const SWARMSH_WORKTREE_AI_DECISION_TYPE: &str = "swarmsh.worktree.swarmsh.worktree.ai.decision_type";
    pub const SWARMSH_WORKTREE_AI_DECISIONS: &str = "swarmsh.worktree.swarmsh.worktree.ai.decisions";
    pub const SWARMSH_WORKTREE_AI_OPTIMIZATION_TARGET: &str = "swarmsh.worktree.swarmsh.worktree.ai.optimization_target";
    pub const SWARMSH_WORKTREE_AI_RECOMMENDATION_COUNT: &str = "swarmsh.worktree.swarmsh.worktree.ai.recommendation_count";
    pub const SWARMSH_WORKTREE_AUTO_SYNC: &str = "swarmsh.worktree.swarmsh.worktree.auto_sync";
    pub const SWARMSH_WORKTREE_BACKUP_ENABLED: &str = "swarmsh.worktree.swarmsh.worktree.backup_enabled";
    pub const SWARMSH_WORKTREE_BRANCH: &str = "swarmsh.worktree.swarmsh.worktree.branch";
    pub const SWARMSH_WORKTREE_COMMITS_COUNT: &str = "swarmsh.worktree.swarmsh.worktree.commits_count";
    pub const SWARMSH_WORKTREE_CONFLICTS_RESOLVED: &str = "swarmsh.worktree.swarmsh.worktree.conflicts.resolved";
    pub const SWARMSH_WORKTREE_COORDINATION: &str = "swarmsh.worktree.swarmsh.worktree.coordination";
    pub const SWARMSH_WORKTREE_COORDINATION_EPOCH: &str = "swarmsh.worktree.swarmsh.worktree.coordination_epoch";
    pub const SWARMSH_WORKTREE_COORDINATION_PATTERN: &str = "swarmsh.worktree.swarmsh.worktree.coordination_pattern";
    pub const SWARMSH_WORKTREE_COORDINATION_CONSENSUS_REQUIRED: &str = "swarmsh.worktree.swarmsh.worktree.coordination.consensus_required";
    pub const SWARMSH_WORKTREE_COORDINATION_LATENCY: &str = "swarmsh.worktree.swarmsh.worktree.coordination.latency";
    pub const SWARMSH_WORKTREE_COORDINATION_LATENCY_MS: &str = "swarmsh.worktree.swarmsh.worktree.coordination.latency_ms";
    pub const SWARMSH_WORKTREE_COORDINATION_PARTICIPANT_COUNT: &str = "swarmsh.worktree.swarmsh.worktree.coordination.participant_count";
    pub const SWARMSH_WORKTREE_COORDINATION_VOTING_THRESHOLD: &str = "swarmsh.worktree.swarmsh.worktree.coordination.voting_threshold";
    pub const SWARMSH_WORKTREE_DISK_USAGE: &str = "swarmsh.worktree.swarmsh.worktree.disk_usage";
    pub const SWARMSH_WORKTREE_DISK_USAGE_MB: &str = "swarmsh.worktree.swarmsh.worktree.disk_usage_mb";
    pub const SWARMSH_WORKTREE_HEALTH: &str = "swarmsh.worktree.swarmsh.worktree.health";
    pub const SWARMSH_WORKTREE_HEALTH_BOTTLENECK_DETECTED: &str = "swarmsh.worktree.swarmsh.worktree.health.bottleneck_detected";
    pub const SWARMSH_WORKTREE_HEALTH_BOTTLENECK_TYPE: &str = "swarmsh.worktree.swarmsh.worktree.health.bottleneck_type";
    pub const SWARMSH_WORKTREE_HEALTH_LAST_CHECK_TIMESTAMP: &str = "swarmsh.worktree.swarmsh.worktree.health.last_check_timestamp";
    pub const SWARMSH_WORKTREE_HEALTH_STATUS: &str = "swarmsh.worktree.swarmsh.worktree.health.status";
    pub const SWARMSH_WORKTREE_HEALTH_UPTIME_PERCENTAGE: &str = "swarmsh.worktree.swarmsh.worktree.health.uptime_percentage";
    pub const SWARMSH_WORKTREE_LIFECYCLE: &str = "swarmsh.worktree.swarmsh.worktree.lifecycle";
    pub const SWARMSH_WORKTREE_LIFECYCLE_DURATION_MS: &str = "swarmsh.worktree.swarmsh.worktree.lifecycle.duration_ms";
    pub const SWARMSH_WORKTREE_LIFECYCLE_PHASE: &str = "swarmsh.worktree.swarmsh.worktree.lifecycle.phase";
    pub const SWARMSH_WORKTREE_LIFECYCLE_SUCCESS: &str = "swarmsh.worktree.swarmsh.worktree.lifecycle.success";
    pub const SWARMSH_WORKTREE_METRICS: &str = "swarmsh.worktree.swarmsh.worktree.metrics";
    pub const SWARMSH_WORKTREE_NAME: &str = "swarmsh.worktree.swarmsh.worktree.name";
    pub const SWARMSH_WORKTREE_OPERATION: &str = "swarmsh.worktree.swarmsh.worktree.operation";
    pub const SWARMSH_WORKTREE_OPERATIONS_COUNT: &str = "swarmsh.worktree.swarmsh.worktree.operations.count";
    pub const SWARMSH_WORKTREE_OPERATIONS_DURATION: &str = "swarmsh.worktree.swarmsh.worktree.operations.duration";
    pub const SWARMSH_WORKTREE_PATH: &str = "swarmsh.worktree.swarmsh.worktree.path";
    pub const SWARMSH_WORKTREE_STATUS: &str = "swarmsh.worktree.swarmsh.worktree.status";
    pub const SWARMSH_WORKTREE_SYNC_FREQUENCY_HOURS: &str = "swarmsh.worktree.swarmsh.worktree.sync_frequency_hours";
    pub const SWARMSH_WORKTREE_SYNC_FREQUENCY: &str = "swarmsh.worktree.swarmsh.worktree.sync.frequency";
}

// Automated 80/20 feature detection and implementation
pub mod swarmsh_auto {
    pub const ANALYSIS_ID: &str = "swarmsh.auto.analysis_id";
    pub const BOTTLENECK_IDENTIFIED: &str = "swarmsh.auto.bottleneck_identified";
    pub const COMPLEXITY_SCORE: &str = "swarmsh.auto.complexity_score";
    pub const COORDINATION_METHOD: &str = "swarmsh.auto.coordination_method";
    pub const DEFECT_DENSITY: &str = "swarmsh.auto.defect_density";
    pub const FEATURE_ID: &str = "swarmsh.auto.feature_id";
    pub const FEATURE_NAME: &str = "swarmsh.auto.feature_name";
    pub const FEATURES_DETECTED: &str = "swarmsh.auto.features_detected";
    pub const FEATURES_SELECTED: &str = "swarmsh.auto.features_selected";
    pub const FLOW_EFFICIENCY: &str = "swarmsh.auto.flow_efficiency";
    pub const IMPACT_SCORE: &str = "swarmsh.auto.impact_score";
    pub const IMPLEMENTATION_STATUS: &str = "swarmsh.auto.implementation_status";
    pub const PARALLELISM_FACTOR: &str = "swarmsh.auto.parallelism_factor";
    pub const PERFORMANCE_IMPACT: &str = "swarmsh.auto.performance_impact";
    pub const PROJECT_PATH: &str = "swarmsh.auto.project_path";
    pub const QUALITY_GATE_PASSED: &str = "swarmsh.auto.quality_gate_passed";
    pub const ROLLBACK_TRIGGERED: &str = "swarmsh.auto.rollback_triggered";
    pub const SIGMA_LEVEL: &str = "swarmsh.auto.sigma_level";
    pub const SWARMSH_AUTO_DLSS: &str = "swarmsh.auto.swarmsh.auto.dlss";
    pub const SWARMSH_AUTO_FEATURE: &str = "swarmsh.auto.swarmsh.auto.feature";
    pub const SWARMSH_AUTO_VALIDATION: &str = "swarmsh.auto.swarmsh.auto.validation";
    pub const SWARMSH_AUTO_WAVE: &str = "swarmsh.auto.swarmsh.auto.wave";
    pub const VALIDATION_TYPE: &str = "swarmsh.auto.validation_type";
    pub const VALUE_RATIO: &str = "swarmsh.auto.value_ratio";
    pub const VALUE_SCORE_THRESHOLD: &str = "swarmsh.auto.value_score_threshold";
    pub const WASTE_TYPE: &str = "swarmsh.auto.waste_type";
    pub const WAVE_COMPLETION_RATE: &str = "swarmsh.auto.wave_completion_rate";
    pub const WAVE_ID: &str = "swarmsh.auto.wave_id";
}

