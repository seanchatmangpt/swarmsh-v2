//! AI prompts for SwarmSH coordination patterns
//! 
//! Implements Scrum at Scale and Roberts Rules coordination with AI-enhanced decision making.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Coordination pattern prompt templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationPrompts {
    pub scrum_at_scale: ScrumAtScalePrompts,
    pub roberts_rules: RobertsRulesPrompts,
}

/// Scrum at Scale coordination prompts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrumAtScalePrompts {
    pub sprint_planning: String,
    pub daily_standup: String,
    pub sprint_review: String,
    pub retrospective: String,
    pub scaled_coordination: String,
    pub team_synchronization: String,
    pub backlog_refinement: String,
    pub impediment_removal: String,
}

/// Roberts Rules coordination prompts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobertsRulesPrompts {
    pub motion_processing: String,
    pub voting_procedure: String,
    pub debate_management: String,
    pub quorum_verification: String,
    pub amendment_handling: String,
    pub point_of_order: String,
    pub committee_coordination: String,
    pub decision_ratification: String,
}

impl Default for CoordinationPrompts {
    fn default() -> Self {
        Self {
            scrum_at_scale: ScrumAtScalePrompts::default(),
            roberts_rules: RobertsRulesPrompts::default(),
        }
    }
}

impl Default for ScrumAtScalePrompts {
    fn default() -> Self {
        Self {
            sprint_planning: r#"
You are a Scrum at Scale coordination AI for SwarmSH v2. Analyze the current system state and provide sprint planning decisions.

CONTEXT: {{context}}
CURRENT SPRINT: {{sprint_number}}
TEAM METRICS: {{team_metrics}}
BACKLOG ITEMS: {{backlog_items}}
VELOCITY DATA: {{velocity_data}}

ANALYZE AND PROVIDE:
1. Sprint capacity allocation across teams
2. Priority work items for maximum value delivery
3. Risk assessment and mitigation strategies
4. Cross-team dependencies and coordination needs
5. Success metrics and acceptance criteria

FORMAT: JSON with fields: capacity_allocation, priority_items, risks, dependencies, success_metrics
PRINCIPLES: Zero-conflict coordination, nanosecond precision, mathematical guarantees
"#.trim().to_string(),

            daily_standup: r#"
You are facilitating a Scrum at Scale daily standup for SwarmSH v2 agent coordination.

AGENT STATES: {{agent_states}}
YESTERDAY'S WORK: {{completed_work}}
TODAY'S COMMITMENTS: {{planned_work}}
IMPEDIMENTS: {{impediments}}
COORDINATION LATENCY: {{coordination_latency_ms}}ms

PROVIDE STANDUP ANALYSIS:
1. Progress assessment against sprint goals
2. Impediment prioritization and resolution paths
3. Cross-team synchronization needs
4. Capacity rebalancing recommendations
5. Real-time coordination adjustments

FORMAT: JSON with fields: progress_status, impediment_actions, sync_needs, rebalancing, adjustments
FOCUS: Flow efficiency, waste elimination, zero conflicts
"#.trim().to_string(),

            sprint_review: r#"
You are conducting a Scrum at Scale sprint review for SwarmSH v2.

SPRINT RESULTS: {{sprint_results}}
COMPLETED_WORK: {{completed_work}}
VELOCITY ACHIEVED: {{actual_velocity}}
VELOCITY PLANNED: {{planned_velocity}}
STAKEHOLDER_FEEDBACK: {{feedback}}
SYSTEM_METRICS: {{system_metrics}}

GENERATE REVIEW ANALYSIS:
1. Sprint goal achievement assessment
2. Value delivered to stakeholders
3. Technical debt and quality metrics
4. Velocity trend analysis
5. Coordination pattern effectiveness
6. Recommendations for next sprint

FORMAT: JSON with fields: goal_achievement, value_delivered, quality_metrics, velocity_trends, pattern_effectiveness, recommendations
OPTIMIZE: 80/20 principle, continuous improvement, stakeholder value
"#.trim().to_string(),

            retrospective: r#"
You are leading a Scrum at Scale retrospective for SwarmSH v2 coordination.

SPRINT_DATA: {{sprint_data}}
COORDINATION_METRICS: {{coordination_metrics}}
TEAM_FEEDBACK: {{team_feedback}}
PROCESS_VIOLATIONS: {{process_violations}}
IMPROVEMENT_OPPORTUNITIES: {{improvement_opportunities}}

FACILITATE RETROSPECTIVE:
1. What went well (coordination successes)
2. What could be improved (bottlenecks, waste)
3. Action items for next sprint
4. Process optimization opportunities
5. Tool and automation enhancements
6. Team coordination improvements

FORMAT: JSON with fields: successes, improvements, action_items, optimizations, enhancements, coordination_improvements
PRINCIPLES: Continuous improvement, team empowerment, data-driven decisions
"#.trim().to_string(),

            scaled_coordination: r#"
You are managing Scrum at Scale coordination across multiple SwarmSH v2 agent teams.

TEAMS: {{teams}}
CROSS_TEAM_DEPENDENCIES: {{dependencies}}
SHARED_RESOURCES: {{shared_resources}}
INTEGRATION_POINTS: {{integration_points}}
SCALING_METRICS: {{scaling_metrics}}

COORDINATE AT SCALE:
1. Cross-team dependency management
2. Resource allocation optimization
3. Integration synchronization
4. Scaling bottleneck identification
5. Communication pattern optimization
6. Coordination overhead minimization

FORMAT: JSON with fields: dependency_management, resource_allocation, synchronization, bottlenecks, communication_optimization, overhead_reduction
TARGET: Mathematical zero-conflict guarantees, nanosecond precision
"#.trim().to_string(),

            team_synchronization: r#"
You are synchronizing SwarmSH v2 agent teams in a Scrum at Scale environment.

TEAM_STATES: {{team_states}}
SYNC_POINTS: {{sync_points}}
INTEGRATION_SCHEDULE: {{integration_schedule}}
COORDINATION_EVENTS: {{coordination_events}}
TIMING_CONSTRAINTS: {{timing_constraints}}

SYNCHRONIZE TEAMS:
1. Optimal sync point scheduling
2. Integration timing coordination
3. Cross-team communication protocols
4. Conflict resolution procedures
5. Escalation paths and triggers
6. Performance impact minimization

FORMAT: JSON with fields: sync_schedule, integration_timing, communication_protocols, conflict_resolution, escalation_paths, performance_impact
ENSURE: Zero-conflict guarantees, atomic operations, real-time coordination
"#.trim().to_string(),

            backlog_refinement: r#"
You are refining the product backlog for SwarmSH v2 using Scrum at Scale principles.

BACKLOG_ITEMS: {{backlog_items}}
CAPACITY_FORECAST: {{capacity_forecast}}
BUSINESS_VALUE: {{business_value}}
TECHNICAL_DEBT: {{technical_debt}}
DEPENDENCIES: {{dependencies}}
STAKEHOLDER_PRIORITIES: {{stakeholder_priorities}}

REFINE BACKLOG:
1. Story prioritization by value and effort
2. Dependency analysis and sequencing
3. Capacity-based planning recommendations
4. Technical debt integration strategy
5. Risk assessment and mitigation
6. Definition of ready validation

FORMAT: JSON with fields: prioritized_stories, sequencing, capacity_planning, debt_strategy, risk_assessment, ready_validation
OPTIMIZE: Value delivery, flow efficiency, sustainable pace
"#.trim().to_string(),

            impediment_removal: r#"
You are identifying and removing impediments in SwarmSH v2 Scrum at Scale coordination.

IMPEDIMENTS: {{impediments}}
TEAM_IMPACT: {{team_impact}}
ESCALATION_HISTORY: {{escalation_history}}
AVAILABLE_RESOURCES: {{available_resources}}
COORDINATION_CONSTRAINTS: {{coordination_constraints}}

REMOVE IMPEDIMENTS:
1. Impediment impact analysis and prioritization
2. Resolution strategy recommendations
3. Resource allocation for removal
4. Escalation path activation
5. Prevention strategies for future
6. Coordination process improvements

FORMAT: JSON with fields: impact_analysis, resolution_strategies, resource_allocation, escalation_activation, prevention_strategies, process_improvements
FOCUS: Flow restoration, waste elimination, continuous improvement
"#.trim().to_string(),
        }
    }
}

impl Default for RobertsRulesPrompts {
    fn default() -> Self {
        Self {
            motion_processing: r#"
You are processing motions in SwarmSH v2 using Roberts Rules of Order for agent coordination.

MOTION: {{motion}}
PROPOSER: {{proposer_agent}}
CURRENT_BUSINESS: {{current_business}}
MEETING_STATE: {{meeting_state}}
QUORUM_STATUS: {{quorum_status}}
PENDING_MOTIONS: {{pending_motions}}

PROCESS MOTION:
1. Motion validity and precedence check
2. Required seconds and support validation
3. Debate requirements and time allocation
4. Voting procedure specification
5. Amendment possibilities assessment
6. Implementation timeline and responsibilities

FORMAT: JSON with fields: validity_check, seconds_required, debate_requirements, voting_procedure, amendment_options, implementation_plan
ENSURE: Parliamentary procedure compliance, fair debate, democratic decision-making
"#.trim().to_string(),

            voting_procedure: r#"
You are managing voting procedures in SwarmSH v2 using Roberts Rules coordination.

MOTION_TO_VOTE: {{motion_to_vote}}
ELIGIBLE_VOTERS: {{eligible_voters}}
VOTING_METHOD: {{voting_method}}
QUORUM_PRESENT: {{quorum_present}}
DEBATE_COMPLETE: {{debate_complete}}
AMENDMENTS: {{amendments}}

CONDUCT VOTING:
1. Voting eligibility verification
2. Voting method optimization (voice, roll call, ballot)
3. Quorum maintenance throughout vote
4. Amendment integration procedures
5. Vote counting and verification protocols
6. Result announcement and recording

FORMAT: JSON with fields: eligibility_verification, voting_method_choice, quorum_maintenance, amendment_procedures, counting_protocols, result_recording
PRINCIPLES: Transparency, accuracy, democratic participation, conflict resolution
"#.trim().to_string(),

            debate_management: r#"
You are managing debate in SwarmSH v2 using Roberts Rules for structured agent discussion.

DEBATE_TOPIC: {{debate_topic}}
SPEAKERS_QUEUE: {{speakers_queue}}
TIME_LIMITS: {{time_limits}}
POINTS_OF_ORDER: {{points_of_order}}
DEBATE_HISTORY: {{debate_history}}
AMENDMENT_PROPOSALS: {{amendment_proposals}}

MANAGE DEBATE:
1. Speaking order and time allocation
2. Point of order resolution
3. Relevance and decorum enforcement
4. Amendment introduction timing
5. Debate closure criteria assessment
6. Information gathering facilitation

FORMAT: JSON with fields: speaking_order, time_allocation, order_resolutions, decorum_enforcement, amendment_timing, closure_assessment
ENSURE: Fair participation, productive discussion, orderly process, informed decisions
"#.trim().to_string(),

            quorum_verification: r#"
You are verifying and maintaining quorum for SwarmSH v2 Roberts Rules coordination.

REGISTERED_AGENTS: {{registered_agents}}
PRESENT_AGENTS: {{present_agents}}
QUORUM_REQUIREMENT: {{quorum_requirement}}
MEETING_TYPE: {{meeting_type}}
BUSINESS_URGENCY: {{business_urgency}}
ATTENDANCE_HISTORY: {{attendance_history}}

VERIFY QUORUM:
1. Current attendance count and validation
2. Quorum requirement calculation
3. Meeting validity assessment
4. Business continuity recommendations
5. Attendance improvement strategies
6. Emergency procedure activation if needed

FORMAT: JSON with fields: attendance_count, quorum_status, meeting_validity, continuity_recommendations, attendance_strategies, emergency_procedures
MAINTAIN: Democratic legitimacy, proper authorization, representative decisions
"#.trim().to_string(),

            amendment_handling: r#"
You are processing amendments in SwarmSH v2 using Roberts Rules coordination protocols.

ORIGINAL_MOTION: {{original_motion}}
PROPOSED_AMENDMENT: {{proposed_amendment}}
AMENDMENT_TYPE: {{amendment_type}}
AMENDMENT_SPONSOR: {{amendment_sponsor}}
GERMANE_ANALYSIS: {{germane_analysis}}
VOTING_SEQUENCE: {{voting_sequence}}

HANDLE AMENDMENT:
1. Germaneness verification
2. Amendment precedence determination
3. Debate allocation for amendment
4. Voting sequence planning
5. Integration impact assessment
6. Implementation coordination

FORMAT: JSON with fields: germaneness_check, precedence_order, debate_allocation, voting_sequence, impact_assessment, implementation_coordination
ENSURE: Proper procedure, logical consideration, democratic refinement
"#.trim().to_string(),

            point_of_order: r#"
You are addressing points of order in SwarmSH v2 Roberts Rules coordination.

POINT_RAISED: {{point_raised}}
RAISING_AGENT: {{raising_agent}}
CURRENT_PROCEEDING: {{current_proceeding}}
RULE_CITATION: {{rule_citation}}
PROCEDURAL_HISTORY: {{procedural_history}}
CHAIR_RESPONSE: {{chair_response}}

ADDRESS POINT OF ORDER:
1. Point validity assessment
2. Procedural rule interpretation
3. Immediate action requirements
4. Proceeding correction recommendations
5. Educational guidance provision
6. Process improvement suggestions

FORMAT: JSON with fields: validity_assessment, rule_interpretation, immediate_actions, correction_recommendations, educational_guidance, process_improvements
MAINTAIN: Procedural integrity, fair process, rule compliance, learning culture
"#.trim().to_string(),

            committee_coordination: r#"
You are coordinating committee work in SwarmSH v2 using Roberts Rules structure.

COMMITTEE_TYPE: {{committee_type}}
COMMITTEE_MEMBERS: {{committee_members}}
ASSIGNED_WORK: {{assigned_work}}
REPORTING_SCHEDULE: {{reporting_schedule}}
COORDINATION_NEEDS: {{coordination_needs}}
RESOURCE_REQUIREMENTS: {{resource_requirements}}

COORDINATE COMMITTEE:
1. Work assignment optimization
2. Member role clarification
3. Reporting timeline management
4. Inter-committee coordination
5. Resource allocation efficiency
6. Progress monitoring protocols

FORMAT: JSON with fields: work_optimization, role_clarification, timeline_management, inter_committee_coordination, resource_efficiency, monitoring_protocols
ACHIEVE: Effective delegation, specialized focus, coordinated effort, accountability
"#.trim().to_string(),

            decision_ratification: r#"
You are ratifying decisions in SwarmSH v2 using Roberts Rules validation protocols.

DECISION_MADE: {{decision_made}}
VOTING_RECORD: {{voting_record}}
PROCEDURAL_COMPLIANCE: {{procedural_compliance}}
IMPLEMENTATION_PLAN: {{implementation_plan}}
STAKEHOLDER_NOTIFICATION: {{stakeholder_notification}}
DOCUMENTATION_REQUIREMENTS: {{documentation_requirements}}

RATIFY DECISION:
1. Procedural compliance verification
2. Decision validity confirmation
3. Implementation authorization
4. Stakeholder communication planning
5. Documentation completion requirements
6. Monitoring and evaluation setup

FORMAT: JSON with fields: compliance_verification, validity_confirmation, implementation_authorization, communication_planning, documentation_requirements, evaluation_setup
ENSURE: Legitimate authority, proper process, clear implementation, accountability
"#.trim().to_string(),
        }
    }
}

/// Prompt context for coordination decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationContext {
    pub pattern: String,
    pub operation: String,
    pub agents: Vec<String>,
    pub metrics: HashMap<String, serde_json::Value>,
    pub constraints: Vec<String>,
    pub objectives: Vec<String>,
}

impl CoordinationPrompts {
    /// Get prompt for specific coordination pattern and operation
    pub fn get_prompt(&self, pattern: &str, operation: &str) -> Option<&String> {
        match pattern {
            "scrum_at_scale" => match operation {
                "sprint_planning" => Some(&self.scrum_at_scale.sprint_planning),
                "daily_standup" => Some(&self.scrum_at_scale.daily_standup),
                "sprint_review" => Some(&self.scrum_at_scale.sprint_review),
                "retrospective" => Some(&self.scrum_at_scale.retrospective),
                "scaled_coordination" => Some(&self.scrum_at_scale.scaled_coordination),
                "team_synchronization" => Some(&self.scrum_at_scale.team_synchronization),
                "backlog_refinement" => Some(&self.scrum_at_scale.backlog_refinement),
                "impediment_removal" => Some(&self.scrum_at_scale.impediment_removal),
                _ => None,
            },
            "roberts_rules" => match operation {
                "motion_processing" => Some(&self.roberts_rules.motion_processing),
                "voting_procedure" => Some(&self.roberts_rules.voting_procedure),
                "debate_management" => Some(&self.roberts_rules.debate_management),
                "quorum_verification" => Some(&self.roberts_rules.quorum_verification),
                "amendment_handling" => Some(&self.roberts_rules.amendment_handling),
                "point_of_order" => Some(&self.roberts_rules.point_of_order),
                "committee_coordination" => Some(&self.roberts_rules.committee_coordination),
                "decision_ratification" => Some(&self.roberts_rules.decision_ratification),
                _ => None,
            },
            _ => None,
        }
    }

    /// Render prompt with context using basic string replacement
    pub fn render_prompt(&self, pattern: &str, operation: &str, context: &CoordinationContext) -> Option<String> {
        if let Some(template) = self.get_prompt(pattern, operation) {
            let mut rendered = template.clone();
            
            // Replace context variables
            rendered = rendered.replace("{{context}}", &serde_json::to_string_pretty(&context.metrics).unwrap_or_default());
            rendered = rendered.replace("{{pattern}}", pattern);
            rendered = rendered.replace("{{operation}}", operation);
            rendered = rendered.replace("{{agents}}", &context.agents.join(", "));
            rendered = rendered.replace("{{constraints}}", &context.constraints.join(", "));
            rendered = rendered.replace("{{objectives}}", &context.objectives.join(", "));
            
            // Replace metric-specific variables
            for (key, value) in &context.metrics {
                let placeholder = format!("{{{{{}}}}}", key);
                let value_str = match value {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    _ => serde_json::to_string(value).unwrap_or_default(),
                };
                rendered = rendered.replace(&placeholder, &value_str);
            }
            
            Some(rendered)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_retrieval() {
        let prompts = CoordinationPrompts::default();
        
        // Test Scrum at Scale prompt retrieval
        assert!(prompts.get_prompt("scrum_at_scale", "sprint_planning").is_some());
        assert!(prompts.get_prompt("scrum_at_scale", "daily_standup").is_some());
        
        // Test Roberts Rules prompt retrieval
        assert!(prompts.get_prompt("roberts_rules", "motion_processing").is_some());
        assert!(prompts.get_prompt("roberts_rules", "voting_procedure").is_some());
        
        // Test invalid patterns
        assert!(prompts.get_prompt("invalid_pattern", "operation").is_none());
        assert!(prompts.get_prompt("scrum_at_scale", "invalid_operation").is_none());
    }

    #[test]
    fn test_prompt_rendering() {
        let prompts = CoordinationPrompts::default();
        let mut metrics = HashMap::new();
        metrics.insert("sprint_number".to_string(), serde_json::Value::Number(serde_json::Number::from(1)));
        metrics.insert("team_metrics".to_string(), serde_json::Value::String("velocity: 25".to_string()));
        
        let context = CoordinationContext {
            pattern: "scrum_at_scale".to_string(),
            operation: "sprint_planning".to_string(),
            agents: vec!["agent1".to_string(), "agent2".to_string()],
            metrics,
            constraints: vec!["zero_conflict".to_string()],
            objectives: vec!["maximize_flow".to_string()],
        };
        
        let rendered = prompts.render_prompt("scrum_at_scale", "sprint_planning", &context);
        assert!(rendered.is_some());
        
        let rendered_text = rendered.unwrap();
        assert!(rendered_text.contains("agent1, agent2"));
        assert!(rendered_text.contains("zero_conflict"));
        assert!(rendered_text.contains("maximize_flow"));
    }
}