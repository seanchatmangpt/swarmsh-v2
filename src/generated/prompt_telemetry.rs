//! Generated prompt telemetry for SwarmSH v2
//! 
//! Auto-generated from semantic conventions for AI prompt tracking in coordination patterns.
//! DO NOT EDIT MANUALLY - regenerate with `weaver generate --template rust`

use opentelemetry::{trace::Span, KeyValue};
use tracing::{info_span, Span as TracingSpan, info};
use std::time::{Duration, Instant};

/// Scrum at Scale prompt span builder
pub struct ScrumAtScaleSpanBuilder {
    operation_name: String,
    pattern: Option<String>,
    operation: Option<String>,
    template_id: Option<String>,
    context_size: Option<i64>,
    agent_count: Option<i64>,
    sprint_number: Option<i64>,
    team_count: Option<i64>,
    velocity_planned: Option<f64>,
    velocity_actual: Option<f64>,
    impediment_count: Option<i64>,
    backlog_size: Option<i64>,
}

impl ScrumAtScaleSpanBuilder {
    pub fn new(operation: &str) -> Self {
        Self {
            operation_name: operation.to_string(),
            pattern: None,
            operation: None,
            template_id: None,
            context_size: None,
            agent_count: None,
            sprint_number: None,
            team_count: None,
            velocity_planned: None,
            velocity_actual: None,
            impediment_count: None,
            backlog_size: None,
        }
    }

    /// Set coordination pattern type
    pub fn with_pattern(mut self, value: String) -> Self {
        self.pattern = Some(value);
        self
    }

    /// Set specific operation within the pattern
    pub fn with_operation(mut self, value: String) -> Self {
        self.operation = Some(value);
        self
    }

    /// Set unique identifier for the prompt template
    pub fn with_template_id(mut self, value: String) -> Self {
        self.template_id = Some(value);
        self
    }

    /// Set size of context data in bytes
    pub fn with_context_size(mut self, value: i64) -> Self {
        self.context_size = Some(value);
        self
    }

    /// Set number of agents involved in coordination
    pub fn with_agent_count(mut self, value: i64) -> Self {
        self.agent_count = Some(value);
        self
    }

    /// Set current sprint number
    pub fn with_sprint_number(mut self, value: i64) -> Self {
        self.sprint_number = Some(value);
        self
    }

    /// Set number of teams in scaled coordination
    pub fn with_team_count(mut self, value: i64) -> Self {
        self.team_count = Some(value);
        self
    }

    /// Set planned sprint velocity
    pub fn with_velocity_planned(mut self, value: f64) -> Self {
        self.velocity_planned = Some(value);
        self
    }

    /// Set actual achieved velocity
    pub fn with_velocity_actual(mut self, value: f64) -> Self {
        self.velocity_actual = Some(value);
        self
    }

    /// Set number of active impediments
    pub fn with_impediment_count(mut self, value: i64) -> Self {
        self.impediment_count = Some(value);
        self
    }

    /// Set current backlog item count
    pub fn with_backlog_size(mut self, value: i64) -> Self {
        self.backlog_size = Some(value);
        self
    }
    
    /// Start the span with all configured attributes
    pub fn start(self) -> TracingSpan {
        let span = info_span!(
            "swarmsh.prompts.scrum_at_scale",
            operation = %self.operation_name,
            pattern = tracing::field::Empty,
            operation_field = tracing::field::Empty,
            template_id = tracing::field::Empty,
            context_size = tracing::field::Empty,
            agent_count = tracing::field::Empty,
            sprint_number = tracing::field::Empty,
            team_count = tracing::field::Empty,
            velocity_planned = tracing::field::Empty,
            velocity_actual = tracing::field::Empty,
            impediment_count = tracing::field::Empty,
            backlog_size = tracing::field::Empty,
        );
        
        if let Some(value) = self.pattern {
            span.record("pattern", &tracing::field::display(&value));
        }
        if let Some(value) = self.operation {
            span.record("operation_field", &tracing::field::display(&value));
        }
        if let Some(value) = self.template_id {
            span.record("template_id", &tracing::field::display(&value));
        }
        if let Some(value) = self.context_size {
            span.record("context_size", &tracing::field::display(&value));
        }
        if let Some(value) = self.agent_count {
            span.record("agent_count", &tracing::field::display(&value));
        }
        if let Some(value) = self.sprint_number {
            span.record("sprint_number", &tracing::field::display(&value));
        }
        if let Some(value) = self.team_count {
            span.record("team_count", &tracing::field::display(&value));
        }
        if let Some(value) = self.velocity_planned {
            span.record("velocity_planned", &tracing::field::display(&value));
        }
        if let Some(value) = self.velocity_actual {
            span.record("velocity_actual", &tracing::field::display(&value));
        }
        if let Some(value) = self.impediment_count {
            span.record("impediment_count", &tracing::field::display(&value));
        }
        if let Some(value) = self.backlog_size {
            span.record("backlog_size", &tracing::field::display(&value));
        }
        
        span
    }
}

/// Roberts Rules prompt span builder
pub struct RobertsRulesSpanBuilder {
    operation_name: String,
    pattern: Option<String>,
    operation: Option<String>,
    template_id: Option<String>,
    context_size: Option<i64>,
    agent_count: Option<i64>,
    motion_id: Option<String>,
    motion_type: Option<String>,
    quorum_required: Option<i64>,
    quorum_present: Option<i64>,
    voting_method: Option<String>,
    debate_time_limit: Option<i64>,
    amendment_count: Option<i64>,
    speakers_queue_length: Option<i64>,
}

impl RobertsRulesSpanBuilder {
    pub fn new(operation: &str) -> Self {
        Self {
            operation_name: operation.to_string(),
            pattern: None,
            operation: None,
            template_id: None,
            context_size: None,
            agent_count: None,
            motion_id: None,
            motion_type: None,
            quorum_required: None,
            quorum_present: None,
            voting_method: None,
            debate_time_limit: None,
            amendment_count: None,
            speakers_queue_length: None,
        }
    }

    /// Set coordination pattern type
    pub fn with_pattern(mut self, value: String) -> Self {
        self.pattern = Some(value);
        self
    }

    /// Set specific operation within the pattern
    pub fn with_operation(mut self, value: String) -> Self {
        self.operation = Some(value);
        self
    }

    /// Set unique identifier for the prompt template
    pub fn with_template_id(mut self, value: String) -> Self {
        self.template_id = Some(value);
        self
    }

    /// Set size of context data in bytes
    pub fn with_context_size(mut self, value: i64) -> Self {
        self.context_size = Some(value);
        self
    }

    /// Set number of agents involved in coordination
    pub fn with_agent_count(mut self, value: i64) -> Self {
        self.agent_count = Some(value);
        self
    }

    /// Set unique identifier for the motion being processed
    pub fn with_motion_id(mut self, value: String) -> Self {
        self.motion_id = Some(value);
        self
    }

    /// Set type of motion being processed
    pub fn with_motion_type(mut self, value: String) -> Self {
        self.motion_type = Some(value);
        self
    }

    /// Set minimum quorum required for valid proceedings
    pub fn with_quorum_required(mut self, value: i64) -> Self {
        self.quorum_required = Some(value);
        self
    }

    /// Set number of agents currently present
    pub fn with_quorum_present(mut self, value: i64) -> Self {
        self.quorum_present = Some(value);
        self
    }

    /// Set method used for voting
    pub fn with_voting_method(mut self, value: String) -> Self {
        self.voting_method = Some(value);
        self
    }

    /// Set time limit for debate in seconds
    pub fn with_debate_time_limit(mut self, value: i64) -> Self {
        self.debate_time_limit = Some(value);
        self
    }

    /// Set number of amendments proposed
    pub fn with_amendment_count(mut self, value: i64) -> Self {
        self.amendment_count = Some(value);
        self
    }

    /// Set number of agents in speaking queue
    pub fn with_speakers_queue_length(mut self, value: i64) -> Self {
        self.speakers_queue_length = Some(value);
        self
    }
    
    /// Start the span with all configured attributes
    pub fn start(self) -> TracingSpan {
        let span = info_span!(
            "swarmsh.prompts.roberts_rules",
            operation = %self.operation_name,
            pattern = tracing::field::Empty,
            operation_field = tracing::field::Empty,
            template_id = tracing::field::Empty,
            context_size = tracing::field::Empty,
            agent_count = tracing::field::Empty,
            motion_id = tracing::field::Empty,
            motion_type = tracing::field::Empty,
            quorum_required = tracing::field::Empty,
            quorum_present = tracing::field::Empty,
            voting_method = tracing::field::Empty,
            debate_time_limit = tracing::field::Empty,
            amendment_count = tracing::field::Empty,
            speakers_queue_length = tracing::field::Empty,
        );
        
        if let Some(value) = self.pattern {
            span.record("pattern", &tracing::field::display(&value));
        }
        if let Some(value) = self.operation {
            span.record("operation_field", &tracing::field::display(&value));
        }
        if let Some(value) = self.template_id {
            span.record("template_id", &tracing::field::display(&value));
        }
        if let Some(value) = self.context_size {
            span.record("context_size", &tracing::field::display(&value));
        }
        if let Some(value) = self.agent_count {
            span.record("agent_count", &tracing::field::display(&value));
        }
        if let Some(value) = self.motion_id {
            span.record("motion_id", &tracing::field::display(&value));
        }
        if let Some(value) = self.motion_type {
            span.record("motion_type", &tracing::field::display(&value));
        }
        if let Some(value) = self.quorum_required {
            span.record("quorum_required", &tracing::field::display(&value));
        }
        if let Some(value) = self.quorum_present {
            span.record("quorum_present", &tracing::field::display(&value));
        }
        if let Some(value) = self.voting_method {
            span.record("voting_method", &tracing::field::display(&value));
        }
        if let Some(value) = self.debate_time_limit {
            span.record("debate_time_limit", &tracing::field::display(&value));
        }
        if let Some(value) = self.amendment_count {
            span.record("amendment_count", &tracing::field::display(&value));
        }
        if let Some(value) = self.speakers_queue_length {
            span.record("speakers_queue_length", &tracing::field::display(&value));
        }
        
        span
    }
}

/// General coordination prompt span builder
pub struct CoordinationSpanBuilder {
    operation_name: String,
    pattern: Option<String>,
    operation: Option<String>,
    template_id: Option<String>,
    ai_provider: Option<String>,
    model_name: Option<String>,
    response_time_ms: Option<i64>,
    response_confidence: Option<f64>,
    response_length: Option<i64>,
    coordination_decision: Option<String>,
    alternatives_count: Option<i64>,
    implementation_priority: Option<String>,
    coordination_epoch: Option<i64>,
    conflict_resolution: Option<bool>,
    cache_hit: Option<bool>,
}

impl CoordinationSpanBuilder {
    pub fn new(operation: &str) -> Self {
        Self {
            operation_name: operation.to_string(),
            pattern: None,
            operation: None,
            template_id: None,
            ai_provider: None,
            model_name: None,
            response_time_ms: None,
            response_confidence: None,
            response_length: None,
            coordination_decision: None,
            alternatives_count: None,
            implementation_priority: None,
            coordination_epoch: None,
            conflict_resolution: None,
            cache_hit: None,
        }
    }

    /// Set coordination pattern type
    pub fn with_pattern(mut self, value: String) -> Self {
        self.pattern = Some(value);
        self
    }

    /// Set specific operation within the pattern
    pub fn with_operation(mut self, value: String) -> Self {
        self.operation = Some(value);
        self
    }

    /// Set unique identifier for the prompt template
    pub fn with_template_id(mut self, value: String) -> Self {
        self.template_id = Some(value);
        self
    }

    /// Set AI provider used for prompt processing
    pub fn with_ai_provider(mut self, value: String) -> Self {
        self.ai_provider = Some(value);
        self
    }

    /// Set specific AI model identifier
    pub fn with_model_name(mut self, value: String) -> Self {
        self.model_name = Some(value);
        self
    }

    /// Set AI response time in milliseconds
    pub fn with_response_time_ms(mut self, value: i64) -> Self {
        self.response_time_ms = Some(value);
        self
    }

    /// Set AI confidence score for the response
    pub fn with_response_confidence(mut self, value: f64) -> Self {
        self.response_confidence = Some(value);
        self
    }

    /// Set length of AI response in characters
    pub fn with_response_length(mut self, value: i64) -> Self {
        self.response_length = Some(value);
        self
    }

    /// Set primary coordination decision made
    pub fn with_coordination_decision(mut self, value: String) -> Self {
        self.coordination_decision = Some(value);
        self
    }

    /// Set number of alternative options provided
    pub fn with_alternatives_count(mut self, value: i64) -> Self {
        self.alternatives_count = Some(value);
        self
    }

    /// Set priority level for implementing the decision
    pub fn with_implementation_priority(mut self, value: String) -> Self {
        self.implementation_priority = Some(value);
        self
    }

    /// Set nanosecond-precision coordination timestamp
    pub fn with_coordination_epoch(mut self, value: i64) -> Self {
        self.coordination_epoch = Some(value);
        self
    }

    /// Set whether prompt was used for conflict resolution
    pub fn with_conflict_resolution(mut self, value: bool) -> Self {
        self.conflict_resolution = Some(value);
        self
    }

    /// Set whether prompt response was served from cache
    pub fn with_cache_hit(mut self, value: bool) -> Self {
        self.cache_hit = Some(value);
        self
    }
    
    /// Start the span with all configured attributes
    pub fn start(self) -> TracingSpan {
        let span = info_span!(
            "swarmsh.prompts.coordination",
            operation = %self.operation_name,
            pattern = tracing::field::Empty,
            operation_field = tracing::field::Empty,
            template_id = tracing::field::Empty,
            ai_provider = tracing::field::Empty,
            model_name = tracing::field::Empty,
            response_time_ms = tracing::field::Empty,
            response_confidence = tracing::field::Empty,
            response_length = tracing::field::Empty,
            coordination_decision = tracing::field::Empty,
            alternatives_count = tracing::field::Empty,
            implementation_priority = tracing::field::Empty,
            coordination_epoch = tracing::field::Empty,
            conflict_resolution = tracing::field::Empty,
            cache_hit = tracing::field::Empty,
        );
        
        if let Some(value) = self.pattern {
            span.record("pattern", &tracing::field::display(&value));
        }
        if let Some(value) = self.operation {
            span.record("operation_field", &tracing::field::display(&value));
        }
        if let Some(value) = self.template_id {
            span.record("template_id", &tracing::field::display(&value));
        }
        if let Some(value) = self.ai_provider {
            span.record("ai_provider", &tracing::field::display(&value));
        }
        if let Some(value) = self.model_name {
            span.record("model_name", &tracing::field::display(&value));
        }
        if let Some(value) = self.response_time_ms {
            span.record("response_time_ms", &tracing::field::display(&value));
        }
        if let Some(value) = self.response_confidence {
            span.record("response_confidence", &tracing::field::display(&value));
        }
        if let Some(value) = self.response_length {
            span.record("response_length", &tracing::field::display(&value));
        }
        if let Some(value) = self.coordination_decision {
            span.record("coordination_decision", &tracing::field::display(&value));
        }
        if let Some(value) = self.alternatives_count {
            span.record("alternatives_count", &tracing::field::display(&value));
        }
        if let Some(value) = self.implementation_priority {
            span.record("implementation_priority", &tracing::field::display(&value));
        }
        if let Some(value) = self.coordination_epoch {
            span.record("coordination_epoch", &tracing::field::display(&value));
        }
        if let Some(value) = self.conflict_resolution {
            span.record("conflict_resolution", &tracing::field::display(&value));
        }
        if let Some(value) = self.cache_hit {
            span.record("cache_hit", &tracing::field::display(&value));
        }
        
        span
    }
}

/// Prompt telemetry manager for coordination patterns
pub struct PromptTelemetry {
    start_time: Option<Instant>,
}

impl PromptTelemetry {
    pub fn new() -> Self {
        Self {
            start_time: None,
        }
    }

    /// Create Scrum at Scale prompt span
    pub fn scrum_at_scale_span(&self, operation: &str) -> ScrumAtScaleSpanBuilder {
        ScrumAtScaleSpanBuilder::new(operation)
    }

    /// Create Roberts Rules prompt span  
    pub fn roberts_rules_span(&self, operation: &str) -> RobertsRulesSpanBuilder {
        RobertsRulesSpanBuilder::new(operation)
    }

    /// Create general coordination prompt span
    pub fn coordination_span(&self, operation: &str) -> CoordinationSpanBuilder {
        CoordinationSpanBuilder::new(operation)
    }

    /// Record prompt execution timing
    pub fn record_prompt_timing(&mut self, pattern: &str, operation: &str, duration: Duration) {
        metrics::histogram!("swarmsh_prompts_response_time", duration.as_millis() as f64);
    }

    /// Record prompt decision outcome
    pub fn record_prompt_decision(&self, pattern: &str, decision: &str, confidence: f64) {
        metrics::counter!("swarmsh_prompts_decisions_total", 1);
        metrics::histogram!("swarmsh_prompts_confidence_score", confidence);
    }

    /// Record prompt cache hit
    pub fn record_cache_hit(&self, pattern: &str, ai_provider: &str) {
        metrics::counter!("swarmsh_prompts_cache_hits_total", 1);
    }

    /// Record prompt error
    pub fn record_prompt_error(&self, pattern: &str, ai_provider: &str, error_type: &str) {
        metrics::counter!("swarmsh_prompts_errors_total", 1);
    }
}

impl Default for PromptTelemetry {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience functions for common prompt spans
pub fn scrum_sprint_planning_span() -> ScrumAtScaleSpanBuilder {
    ScrumAtScaleSpanBuilder::new("sprint_planning")
        .with_operation("sprint_planning".to_string())
        .with_pattern("scrum_at_scale".to_string())
}

pub fn scrum_daily_standup_span() -> ScrumAtScaleSpanBuilder {
    ScrumAtScaleSpanBuilder::new("daily_standup")
        .with_operation("daily_standup".to_string())
        .with_pattern("scrum_at_scale".to_string())
}

pub fn scrum_retrospective_span() -> ScrumAtScaleSpanBuilder {
    ScrumAtScaleSpanBuilder::new("retrospective")
        .with_operation("retrospective".to_string())
        .with_pattern("scrum_at_scale".to_string())
}

pub fn roberts_motion_processing_span() -> RobertsRulesSpanBuilder {
    RobertsRulesSpanBuilder::new("motion_processing")
        .with_operation("motion_processing".to_string())
        .with_pattern("roberts_rules".to_string())
}

pub fn roberts_voting_procedure_span() -> RobertsRulesSpanBuilder {
    RobertsRulesSpanBuilder::new("voting_procedure")
        .with_operation("voting_procedure".to_string())
        .with_pattern("roberts_rules".to_string())
}

pub fn roberts_debate_management_span() -> RobertsRulesSpanBuilder {
    RobertsRulesSpanBuilder::new("debate_management")
        .with_operation("debate_management".to_string())
        .with_pattern("roberts_rules".to_string())
}

/// Prompt execution context for comprehensive tracking
#[derive(Debug, Clone)]
pub struct PromptExecutionContext {
    pub pattern: String,
    pub operation: String,
    pub ai_provider: String,
    pub model_name: Option<String>,
    pub context_size: usize,
    pub start_time: Instant,
}

impl PromptExecutionContext {
    pub fn new(pattern: &str, operation: &str, ai_provider: &str) -> Self {
        Self {
            pattern: pattern.to_string(),
            operation: operation.to_string(),
            ai_provider: ai_provider.to_string(),
            model_name: None,
            context_size: 0,
            start_time: Instant::now(),
        }
    }

    pub fn with_model(mut self, model: &str) -> Self {
        self.model_name = Some(model.to_string());
        self
    }

    pub fn with_context_size(mut self, size: usize) -> Self {
        self.context_size = size;
        self
    }

    pub fn record_completion(&self, confidence: f64, decision: &str) {
        let duration = self.start_time.elapsed();
        
        // Record timing
        metrics::histogram!("swarmsh_prompts_response_time", duration.as_millis() as f64);

        // Record decision
        metrics::counter!("swarmsh_prompts_decisions_total", 1);

        // Record confidence
        metrics::histogram!("swarmsh_prompts_confidence_score", confidence);

        // Record context size
        metrics::histogram!("swarmsh_prompts_context_size", self.context_size as f64);

        info!(
            pattern = %self.pattern,
            operation = %self.operation,
            ai_provider = %self.ai_provider,
            model_name = ?self.model_name,
            duration_ms = duration.as_millis(),
            confidence = confidence,
            decision = %decision,
            context_size = self.context_size,
            "Prompt execution completed"
        );
    }

    pub fn record_error(&self, error_type: &str, error_message: &str) {
        let duration = self.start_time.elapsed();
        
        // Record error
        metrics::counter!("swarmsh_prompts_errors_total", 1);

        tracing::error!(
            pattern = %self.pattern,
            operation = %self.operation,
            ai_provider = %self.ai_provider,
            error_type = %error_type,
            error_message = %error_message,
            duration_ms = duration.as_millis(),
            "Prompt execution failed"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scrum_span_builder() {
        let span = scrum_sprint_planning_span()
            .with_sprint_number(5)
            .with_team_count(3)
            .with_velocity_planned(25.5)
            .start();
        
        // Verify span is created
        assert_eq!(span.metadata().unwrap().name(), "swarmsh.prompts.scrum_at_scale");
    }

    #[test]
    fn test_roberts_span_builder() {
        let span = roberts_motion_processing_span()
            .with_motion_id("motion_001".to_string())
            .with_quorum_required(10)
            .with_quorum_present(12)
            .start();
        
        // Verify span is created
        assert_eq!(span.metadata().unwrap().name(), "swarmsh.prompts.roberts_rules");
    }

    #[test]
    fn test_prompt_execution_context() {
        let context = PromptExecutionContext::new("scrum_at_scale", "sprint_planning", "ollama")
            .with_model("llama2:latest")
            .with_context_size(1024);
        
        assert_eq!(context.pattern, "scrum_at_scale");
        assert_eq!(context.operation, "sprint_planning");
        assert_eq!(context.ai_provider, "ollama");
        assert_eq!(context.model_name, Some("llama2:latest".to_string()));
        assert_eq!(context.context_size, 1024);
    }
}