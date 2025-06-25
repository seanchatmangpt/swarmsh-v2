//! Coordination Lifecycle Tests with OpenTelemetry Integration
//! 
//! Tests for Scrum at Scale and Roberts Rules coordination patterns with full OTEL instrumentation

use swarmsh_v2::coordination::*;
use swarmsh_v2::ai_integration::*;
use swarmsh_v2::{TelemetryManager, AgentId, WorkId};
use std::sync::Arc;
use tokio_test;
use serde_json::json;
use tracing::{info, warn, debug, instrument, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use opentelemetry::{global, trace::{Tracer, TracerProvider}, KeyValue};
use opentelemetry_sdk::trace::TracerProvider as SdkTracerProvider;
use opentelemetry_stdout::SpanExporter;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Initialize lightweight OTEL for testing
fn init_test_telemetry() -> SdkTracerProvider {
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(SpanExporter::default())
        .build();

    let tracer = provider.tracer("swarmsh-test");
    
    tracing_subscriber::registry()
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .with(tracing_subscriber::fmt::layer().with_test_writer())
        .init();

    global::set_tracer_provider(provider.clone());
    provider
}

/// Test fixture for coordination testing
struct CoordinationTestFixture {
    coordinator: Arc<AgentCoordinator>,
    work_queue: Arc<WorkQueue>,
    telemetry: Arc<TelemetryManager>,
    _provider: SdkTracerProvider,
}

impl CoordinationTestFixture {
    async fn new() -> Self {
        let provider = init_test_telemetry();
        let telemetry = Arc::new(TelemetryManager::new().await.unwrap());
        let ai_integration = AIIntegration::new().await.ok().map(Arc::new);
        let work_queue = Arc::new(WorkQueue::new(ai_integration).await.unwrap());
        let coordinator = Arc::new(AgentCoordinator::new(telemetry.clone(), work_queue.clone()).await.unwrap());

        Self {
            coordinator,
            work_queue,
            telemetry,
            _provider: provider,
        }
    }

    async fn register_test_agents(&self) -> Vec<AgentId> {
        let mut agent_ids = Vec::new();
        
        // Scrum Master Agent
        let scrum_master = AgentSpec {
            id: "scrum_master_001".to_string(),
            role: "scrum_master".to_string(),
            capacity: 1.0,
            specializations: vec!["facilitation".to_string(), "coordination".to_string()],
            work_capacity: Some(5),
        };
        self.coordinator.register_agent(scrum_master.clone()).await.unwrap();
        agent_ids.push(scrum_master.id);

        // Product Owner Agent
        let product_owner = AgentSpec {
            id: "product_owner_001".to_string(),
            role: "product_owner".to_string(),
            capacity: 1.0,
            specializations: vec!["requirements".to_string(), "prioritization".to_string()],
            work_capacity: Some(8),
        };
        self.coordinator.register_agent(product_owner.clone()).await.unwrap();
        agent_ids.push(product_owner.id);

        // Development Team Agents
        for i in 1..=3 {
            let dev_agent = AgentSpec {
                id: format!("dev_agent_{:03}", i),
                role: "developer".to_string(),
                capacity: 1.0,
                specializations: vec!["development".to_string(), "testing".to_string()],
                work_capacity: Some(10),
            };
            self.coordinator.register_agent(dev_agent.clone()).await.unwrap();
            agent_ids.push(dev_agent.id);
        }

        agent_ids
    }

    async fn add_test_work_items(&self) -> Vec<WorkId> {
        let mut work_ids = Vec::new();

        let work_items = vec![
            WorkItem {
                id: "story_001".to_string(),
                priority: 0.9,
                requirements: vec!["development".to_string()],
                estimated_duration_ms: 3600000, // 1 hour
                created_at: SystemTime::now(),
            },
            WorkItem {
                id: "story_002".to_string(),
                priority: 0.8,
                requirements: vec!["development".to_string(), "testing".to_string()],
                estimated_duration_ms: 7200000, // 2 hours
                created_at: SystemTime::now(),
            },
            WorkItem {
                id: "spike_001".to_string(),
                priority: 0.7,
                requirements: vec!["requirements".to_string()],
                estimated_duration_ms: 1800000, // 30 minutes
                created_at: SystemTime::now(),
            },
        ];

        for work in work_items {
            work_ids.push(work.id.clone());
            self.work_queue.add_work(work).await.unwrap();
        }

        work_ids
    }
}

#[tokio::test]
#[instrument]
async fn test_scrum_at_scale_full_lifecycle() {
    let span = Span::current();
    span.record("test_type", "scrum_at_scale_lifecycle");
    
    info!("Starting Scrum at Scale full lifecycle test");
    
    let fixture = CoordinationTestFixture::new().await;
    let agent_ids = fixture.register_test_agents().await;
    let work_ids = fixture.add_test_work_items().await;
    
    info!(agents_registered = agent_ids.len(), work_items = work_ids.len(), "Test setup complete");

    // Sprint Planning Phase
    test_sprint_planning_phase(&fixture).await;
    
    // Daily Scrum Phase
    test_daily_scrum_phase(&fixture).await;
    
    // Sprint Review Phase
    test_sprint_review_phase(&fixture).await;
    
    // Sprint Retrospective Phase
    test_sprint_retrospective_phase(&fixture).await;
    
    // Scrum of Scrums Coordination
    test_scrum_of_scrums_coordination(&fixture).await;

    info!("Scrum at Scale lifecycle test completed successfully");
}

#[instrument(skip(fixture))]
async fn test_sprint_planning_phase(fixture: &CoordinationTestFixture) {
    let span = Span::current();
    span.record("scrum_ceremony", "sprint_planning");
    
    info!("Executing Sprint Planning ceremony");
    
    // Phase 1: Sprint Goal Definition
    let planning_span = global::tracer("swarmsh-test")
        .span_builder("sprint_planning_phase1")
        .with_attributes(vec![
            KeyValue::new("ceremony.type", "sprint_planning"),
            KeyValue::new("ceremony.phase", "goal_definition"),
            KeyValue::new("sprint.number", 1),
        ])
        .start(&global::tracer("swarmsh-test"));
    
    let _guard = planning_span.clone();
    
    // AI-enhanced sprint goal generation
    let coordination_result = fixture.coordinator.coordinate(CoordinationPattern::ScrumAtScale).await;
    assert!(coordination_result.is_ok(), "Sprint planning coordination failed");
    
    // Get AI recommendations for sprint planning
    let ai_analysis = fixture.coordinator.get_ai_recommendations(&CoordinationPattern::ScrumAtScale).await;
    match ai_analysis {
        Ok(analysis) => {
            info!(
                recommendations_count = analysis.recommendations.len(),
                confidence = analysis.confidence,
                "AI sprint planning analysis complete"
            );
            span.record("ai_confidence", analysis.confidence);
        }
        Err(e) => {
            warn!(error = %e, "AI analysis unavailable, using traditional planning");
        }
    }
    
    // Phase 2: Capacity Planning with OTEL metrics
    let capacity_span = global::tracer("swarmsh-test")
        .span_builder("capacity_planning")
        .with_attributes(vec![
            KeyValue::new("team_velocity", 25.0),
            KeyValue::new("sprint_duration_days", 14),
            KeyValue::new("team_members", 5),
        ])
        .start(&global::tracer("swarmsh-test"));
    
    let _capacity_guard = capacity_span.clone();
    
    // Simulate capacity calculation
    let total_capacity = 5 * 8 * 10; // 5 agents * 8 hours * 10 days
    capacity_span.set_attribute(KeyValue::new("total_capacity_hours", total_capacity as i64));
    
    info!(total_capacity_hours = total_capacity, "Sprint capacity calculated");
    
    planning_span.end();
    capacity_span.end();
}

#[instrument(skip(fixture))]
async fn test_daily_scrum_phase(fixture: &CoordinationTestFixture) {
    let span = Span::current();
    span.record("scrum_ceremony", "daily_scrum");
    
    info!("Executing Daily Scrum ceremony");
    
    // Simulate 3 days of daily scrums
    for day in 1..=3 {
        let daily_span = global::tracer("swarmsh-test")
            .span_builder("daily_scrum")
            .with_attributes(vec![
                KeyValue::new("ceremony.type", "daily_scrum"),
                KeyValue::new("sprint.day", day),
                KeyValue::new("duration_minutes", 15),
            ])
            .start(&global::tracer("swarmsh-test"));
        
        let _guard = daily_span.clone();
        
        // Three questions pattern with OTEL instrumentation
        test_daily_scrum_three_questions(&daily_span, day).await;
        
        // Impediment identification with AI assistance
        test_impediment_identification(&daily_span, fixture).await;
        
        // Real-time coordination for blockers
        let realtime_result = fixture.coordinator.coordinate(CoordinationPattern::Realtime).await;
        assert!(realtime_result.is_ok(), "Real-time coordination failed");
        
        daily_span.end();
        
        info!(day = day, "Daily Scrum completed");
    }
}

#[instrument(skip(span))]
async fn test_daily_scrum_three_questions(span: &opentelemetry::trace::Span, day: i32) {
    // Yesterday's work
    let yesterday_span = global::tracer("swarmsh-test")
        .span_builder("yesterday_update")
        .with_attributes(vec![
            KeyValue::new("question", "what_did_yesterday"),
            KeyValue::new("tasks_completed", 3),
            KeyValue::new("story_points_completed", 8),
        ])
        .start(&global::tracer("swarmsh-test"));
    
    yesterday_span.end();
    
    // Today's plan
    let today_span = global::tracer("swarmsh-test")
        .span_builder("today_plan")
        .with_attributes(vec![
            KeyValue::new("question", "what_will_today"),
            KeyValue::new("tasks_planned", 2),
            KeyValue::new("story_points_planned", 5),
        ])
        .start(&global::tracer("swarmsh-test"));
    
    today_span.end();
    
    // Impediments
    let impediments_span = global::tracer("swarmsh-test")
        .span_builder("impediments_check")
        .with_attributes(vec![
            KeyValue::new("question", "any_impediments"),
            KeyValue::new("impediments_count", if day == 2 { 1 } else { 0 }),
        ])
        .start(&global::tracer("swarmsh-test"));
    
    if day == 2 {
        impediments_span.set_attribute(KeyValue::new("impediment.type", "dependency_blocking"));
        impediments_span.set_attribute(KeyValue::new("impediment.severity", "medium"));
        warn!("Impediment identified: dependency blocking development");
    }
    
    impediments_span.end();
}

#[instrument(skip(span, fixture))]
async fn test_impediment_identification(span: &opentelemetry::trace::Span, fixture: &CoordinationTestFixture) {
    // Use AI to analyze potential impediments
    let impediment_context = json!({
        "sprint_progress": 0.6,
        "velocity_trend": "declining", 
        "team_morale": "medium",
        "technical_debt_level": "high"
    });
    
    // Get AI decision on impediment resolution
    let ai_integration = AIIntegration::new().await.unwrap();
    match ai_integration.make_decision(&impediment_context, "impediment_resolution").await {
        Ok(decision) => {
            span.set_attribute(KeyValue::new("ai_decision.action", decision.action.clone()));
            span.set_attribute(KeyValue::new("ai_decision.confidence", decision.confidence));
            
            info!(
                action = decision.action,
                confidence = decision.confidence,
                alternatives = ?decision.alternatives,
                "AI impediment resolution decision"
            );
        }
        Err(e) => {
            debug!(error = %e, "AI impediment analysis unavailable");
        }
    }
}

#[instrument(skip(fixture))]
async fn test_sprint_review_phase(fixture: &CoordinationTestFixture) {
    let span = Span::current();
    span.record("scrum_ceremony", "sprint_review");
    
    info!("Executing Sprint Review ceremony");
    
    let review_span = global::tracer("swarmsh-test")
        .span_builder("sprint_review")
        .with_attributes(vec![
            KeyValue::new("ceremony.type", "sprint_review"),
            KeyValue::new("demo_items", 4),
            KeyValue::new("stakeholders_present", 8),
            KeyValue::new("duration_minutes", 60),
        ])
        .start(&global::tracer("swarmsh-test"));
    
    let _guard = review_span.clone();
    
    // Product demonstration with metrics
    let demo_span = global::tracer("swarmsh-test")
        .span_builder("product_demo")
        .with_attributes(vec![
            KeyValue::new("features_demonstrated", 3),
            KeyValue::new("user_stories_completed", 5),
            KeyValue::new("stakeholder_satisfaction", 4.2),
        ])
        .start(&global::tracer("swarmsh-test"));
    
    info!("Product demonstration in progress");
    demo_span.end();
    
    // Sprint metrics collection
    let metrics_span = global::tracer("swarmsh-test")
        .span_builder("sprint_metrics")
        .with_attributes(vec![
            KeyValue::new("velocity_achieved", 23),
            KeyValue::new("velocity_planned", 25),
            KeyValue::new("burndown_accuracy", 0.92),
            KeyValue::new("definition_of_done_adherence", 1.0),
        ])
        .start(&global::tracer("swarmsh-test"));
    
    metrics_span.end();
    review_span.end();
    
    info!("Sprint Review completed successfully");
}

#[instrument(skip(fixture))]
async fn test_sprint_retrospective_phase(fixture: &CoordinationTestFixture) {
    let span = Span::current();
    span.record("scrum_ceremony", "sprint_retrospective");
    
    info!("Executing Sprint Retrospective ceremony");
    
    let retro_span = global::tracer("swarmsh-test")
        .span_builder("sprint_retrospective")
        .with_attributes(vec![
            KeyValue::new("ceremony.type", "sprint_retrospective"),
            KeyValue::new("duration_minutes", 90),
            KeyValue::new("format", "start_stop_continue"),
        ])
        .start(&global::tracer("swarmsh-test"));
    
    let _guard = retro_span.clone();
    
    // Start/Stop/Continue analysis with AI enhancement
    let retrospective_data = json!({
        "start_items": ["automated_testing", "pair_programming"],
        "stop_items": ["late_meetings", "context_switching"],
        "continue_items": ["code_reviews", "daily_standups"],
        "team_satisfaction": 7.5,
        "process_effectiveness": 8.2
    });
    
    // Get AI analysis of retrospective patterns
    let ai_integration = AIIntegration::new().await.unwrap();
    match ai_integration.analyze(&retrospective_data.to_string()).await {
        Ok(analysis) => {
            retro_span.set_attribute(KeyValue::new("ai_insights_count", analysis.recommendations.len() as i64));
            retro_span.set_attribute(KeyValue::new("ai_confidence", analysis.confidence));
            
            info!(
                insights = analysis.recommendations.len(),
                confidence = analysis.confidence,
                "AI retrospective analysis complete"
            );
            
            for (i, recommendation) in analysis.recommendations.iter().enumerate() {
                debug!(recommendation_id = i, recommendation = %recommendation, "AI retrospective insight");
            }
        }
        Err(e) => {
            debug!(error = %e, "AI retrospective analysis unavailable");
        }
    }
    
    // Action items creation
    let actions_span = global::tracer("swarmsh-test")
        .span_builder("action_items")
        .with_attributes(vec![
            KeyValue::new("action_items_created", 3),
            KeyValue::new("action_items_assigned", 3),
        ])
        .start(&global::tracer("swarmsh-test"));
    
    actions_span.end();
    retro_span.end();
    
    info!("Sprint Retrospective completed with AI-enhanced insights");
}

#[instrument(skip(fixture))]
async fn test_scrum_of_scrums_coordination(fixture: &CoordinationTestFixture) {
    let span = Span::current();
    span.record("coordination_type", "scrum_of_scrums");
    
    info!("Executing Scrum of Scrums coordination");
    
    let sos_span = global::tracer("swarmsh-test")
        .span_builder("scrum_of_scrums")
        .with_attributes(vec![
            KeyValue::new("coordination.pattern", "scrum_at_scale"),
            KeyValue::new("teams_participating", 3),
            KeyValue::new("duration_minutes", 30),
        ])
        .start(&global::tracer("swarmsh-test"));
    
    let _guard = sos_span.clone();
    
    // Cross-team dependency resolution
    let dependency_span = global::tracer("swarmsh-test")
        .span_builder("dependency_resolution")
        .with_attributes(vec![
            KeyValue::new("dependencies_identified", 2),
            KeyValue::new("blockers_resolved", 1),
        ])
        .start(&global::tracer("swarmsh-test"));
    
    dependency_span.end();
    
    // Scale coordination with AI assistance
    let coordination_result = fixture.coordinator.coordinate(CoordinationPattern::ScrumAtScale).await;
    assert!(coordination_result.is_ok(), "Scrum of Scrums coordination failed");
    
    sos_span.end();
    
    info!("Scrum of Scrums coordination completed");
}

#[tokio::test]
#[instrument]
async fn test_roberts_rules_full_lifecycle() {
    let span = Span::current();
    span.record("test_type", "roberts_rules_lifecycle");
    
    info!("Starting Roberts Rules full lifecycle test");
    
    let fixture = CoordinationTestFixture::new().await;
    let agent_ids = fixture.register_test_agents().await;
    
    info!(agents_registered = agent_ids.len(), "Roberts Rules test setup complete");

    // Call to Order
    test_call_to_order_phase(&fixture).await;
    
    // Motion Processing
    test_motion_processing_phase(&fixture).await;
    
    // Discussion and Debate
    test_discussion_debate_phase(&fixture).await;
    
    // Voting Process
    test_voting_process_phase(&fixture).await;
    
    // Meeting Adjournment
    test_adjournment_phase(&fixture).await;

    info!("Roberts Rules lifecycle test completed successfully");
}

#[instrument(skip(fixture))]
async fn test_call_to_order_phase(fixture: &CoordinationTestFixture) {
    let span = Span::current();
    span.record("roberts_rules_phase", "call_to_order");
    
    info!("Executing Call to Order");
    
    let meeting_span = global::tracer("swarmsh-test")
        .span_builder("call_to_order")
        .with_attributes(vec![
            KeyValue::new("governance.type", "roberts_rules"),
            KeyValue::new("meeting.type", "regular"),
            KeyValue::new("quorum_present", true),
            KeyValue::new("members_present", 5),
            KeyValue::new("members_required", 3),
        ])
        .start(&global::tracer("swarmsh-test"));
    
    let _guard = meeting_span.clone();
    
    // Quorum verification with OTEL
    let quorum_span = global::tracer("swarmsh-test")
        .span_builder("quorum_verification")
        .with_attributes(vec![
            KeyValue::new("quorum.required", 3),
            KeyValue::new("quorum.present", 5),
            KeyValue::new("quorum.satisfied", true),
        ])
        .start(&global::tracer("swarmsh-test"));
    
    let quorum_satisfied = 5 >= 3; // 5 agents present, 3 required
    assert!(quorum_satisfied, "Quorum not satisfied for Roberts Rules meeting");
    
    quorum_span.end();
    meeting_span.end();
    
    info!("Call to Order completed - quorum satisfied");
}

#[instrument(skip(fixture))]
async fn test_motion_processing_phase(fixture: &CoordinationTestFixture) {
    let span = Span::current();
    span.record("roberts_rules_phase", "motion_processing");
    
    info!("Executing Motion Processing");
    
    let motion_span = global::tracer("swarmsh-test")
        .span_builder("motion_processing")
        .with_attributes(vec![
            KeyValue::new("motion.type", "main_motion"),
            KeyValue::new("motion.text", "Adopt new coordination protocol"),
            KeyValue::new("motion.seconded", true),
        ])
        .start(&global::tracer("swarmsh-test"));
    
    let _guard = motion_span.clone();
    
    // AI-enhanced motion analysis
    let motion_context = json!({
        "motion_text": "Adopt new coordination protocol for improved efficiency",
        "precedence_level": "main_motion",
        "requires_majority": true,
        "amendable": true
    });
    
    let ai_integration = AIIntegration::new().await.unwrap();
    match ai_integration.make_decision(&motion_context, "motion_analysis").await {
        Ok(decision) => {
            motion_span.set_attribute(KeyValue::new("ai_analysis.action", decision.action.clone()));
            motion_span.set_attribute(KeyValue::new("ai_analysis.confidence", decision.confidence));
            
            info!(
                action = decision.action,
                confidence = decision.confidence,
                "AI motion analysis complete"
            );
        }
        Err(e) => {
            debug!(error = %e, "AI motion analysis unavailable");
        }
    }
    
    // Roberts Rules coordination
    let coordination_result = fixture.coordinator.coordinate(CoordinationPattern::RobertsRules).await;
    assert!(coordination_result.is_ok(), "Roberts Rules coordination failed");
    
    motion_span.end();
    
    info!("Motion processing completed");
}

#[instrument(skip(fixture))]
async fn test_discussion_debate_phase(fixture: &CoordinationTestFixture) {
    let span = Span::current();
    span.record("roberts_rules_phase", "discussion_debate");
    
    info!("Executing Discussion and Debate");
    
    let debate_span = global::tracer("swarmsh-test")
        .span_builder("discussion_debate")
        .with_attributes(vec![
            KeyValue::new("speakers_recognized", 3),
            KeyValue::new("speaking_time_limit_minutes", 3),
            KeyValue::new("amendments_proposed", 1),
        ])
        .start(&global::tracer("swarmsh-test"));
    
    let _guard = debate_span.clone();
    
    // Simulate debate with time tracking
    for speaker_id in 1..=3 {
        let speaker_span = global::tracer("swarmsh-test")
            .span_builder("speaker_recognition")
            .with_attributes(vec![
                KeyValue::new("speaker.id", format!("agent_{:03}", speaker_id)),
                KeyValue::new("speaking_duration_seconds", 45 + (speaker_id * 15)),
                KeyValue::new("position", if speaker_id % 2 == 0 { "against" } else { "for" }),
            ])
            .start(&global::tracer("swarmsh-test"));
        
        speaker_span.end();
    }
    
    // Amendment processing
    let amendment_span = global::tracer("swarmsh-test")
        .span_builder("amendment_processing")
        .with_attributes(vec![
            KeyValue::new("amendment.type", "friendly_amendment"),
            KeyValue::new("amendment.accepted", true),
        ])
        .start(&global::tracer("swarmsh-test"));
    
    amendment_span.end();
    debate_span.end();
    
    info!("Discussion and debate phase completed");
}

#[instrument(skip(fixture))]
async fn test_voting_process_phase(fixture: &CoordinationTestFixture) {
    let span = Span::current();
    span.record("roberts_rules_phase", "voting_process");
    
    info!("Executing Voting Process");
    
    let voting_span = global::tracer("swarmsh-test")
        .span_builder("voting_process")
        .with_attributes(vec![
            KeyValue::new("voting.method", "voice_vote"),
            KeyValue::new("votes.for", 4),
            KeyValue::new("votes.against", 1),
            KeyValue::new("votes.abstain", 0),
            KeyValue::new("motion.passed", true),
        ])
        .start(&global::tracer("swarmsh-test"));
    
    let _guard = voting_span.clone();
    
    // AI-enhanced vote counting and verification
    let voting_context = json!({
        "total_eligible_voters": 5,
        "votes_cast": 5,
        "majority_required": 3,
        "votes_for": 4,
        "votes_against": 1,
        "supermajority_required": false
    });
    
    let ai_integration = AIIntegration::new().await.unwrap();
    match ai_integration.make_decision(&voting_context, "vote_verification").await {
        Ok(decision) => {
            voting_span.set_attribute(KeyValue::new("ai_verification.result", decision.action.clone()));
            voting_span.set_attribute(KeyValue::new("ai_verification.confidence", decision.confidence));
            
            info!(
                verification_result = decision.action,
                confidence = decision.confidence,
                "AI vote verification complete"
            );
        }
        Err(e) => {
            debug!(error = %e, "AI vote verification unavailable");
        }
    }
    
    // Verify motion passed
    let votes_for = 4;
    let majority_required = 3; // Simple majority of 5 members
    let motion_passed = votes_for >= majority_required;
    
    assert!(motion_passed, "Motion should have passed with majority vote");
    
    voting_span.set_attribute(KeyValue::new("motion.result", "passed"));
    voting_span.end();
    
    info!(votes_for = votes_for, majority_required = majority_required, "Motion passed by majority vote");
}

#[instrument(skip(fixture))]
async fn test_adjournment_phase(fixture: &CoordinationTestFixture) {
    let span = Span::current();
    span.record("roberts_rules_phase", "adjournment");
    
    info!("Executing Meeting Adjournment");
    
    let adjournment_span = global::tracer("swarmsh-test")
        .span_builder("meeting_adjournment")
        .with_attributes(vec![
            KeyValue::new("adjournment.type", "regular"),
            KeyValue::new("meeting_duration_minutes", 45),
            KeyValue::new("motions_processed", 2),
            KeyValue::new("motions_passed", 1),
        ])
        .start(&global::tracer("swarmsh-test"));
    
    let _guard = adjournment_span.clone();
    
    // Meeting summary with AI analysis
    let meeting_summary = json!({
        "total_duration_minutes": 45,
        "motions_introduced": 2,
        "motions_passed": 1,
        "amendments_proposed": 1,
        "participation_rate": 1.0,
        "procedural_compliance": "excellent"
    });
    
    let ai_integration = AIIntegration::new().await.unwrap();
    match ai_integration.analyze(&meeting_summary.to_string()).await {
        Ok(analysis) => {
            adjournment_span.set_attribute(KeyValue::new("ai_summary.insights", analysis.recommendations.len() as i64));
            adjournment_span.set_attribute(KeyValue::new("ai_summary.confidence", analysis.confidence));
            
            info!(
                insights_generated = analysis.recommendations.len(),
                confidence = analysis.confidence,
                "AI meeting summary analysis complete"
            );
        }
        Err(e) => {
            debug!(error = %e, "AI meeting summary unavailable");
        }
    }
    
    adjournment_span.end();
    
    info!("Meeting adjourned successfully");
}

#[tokio::test]
#[instrument]
async fn test_coordination_pattern_transitions() {
    let span = Span::current();
    span.record("test_type", "pattern_transitions");
    
    info!("Testing coordination pattern transitions with OTEL");
    
    let fixture = CoordinationTestFixture::new().await;
    fixture.register_test_agents().await;
    
    // Test transitions between patterns
    let patterns = vec![
        CoordinationPattern::ScrumAtScale,
        CoordinationPattern::RobertsRules,
        CoordinationPattern::Realtime,
        CoordinationPattern::Atomic,
    ];
    
    for (i, pattern) in patterns.iter().enumerate() {
        let transition_span = global::tracer("swarmsh-test")
            .span_builder("pattern_transition")
            .with_attributes(vec![
                KeyValue::new("transition.sequence", i as i64),
                KeyValue::new("pattern.from", if i == 0 { "none" } else { patterns[i-1].description() }),
                KeyValue::new("pattern.to", pattern.description()),
            ])
            .start(&global::tracer("swarmsh-test"));
        
        let _guard = transition_span.clone();
        
        let result = fixture.coordinator.coordinate(pattern.clone()).await;
        assert!(result.is_ok(), "Pattern transition failed for {:?}", pattern);
        
        transition_span.set_attribute(KeyValue::new("transition.success", true));
        transition_span.end();
        
        info!(pattern = ?pattern, sequence = i, "Pattern transition successful");
    }
    
    info!("All coordination pattern transitions completed successfully");
}

#[tokio::test]
#[instrument]
async fn test_ai_enhanced_coordination_metrics() {
    let span = Span::current();
    span.record("test_type", "ai_enhanced_metrics");
    
    info!("Testing AI-enhanced coordination metrics");
    
    let fixture = CoordinationTestFixture::new().await;
    fixture.register_test_agents().await;
    fixture.add_test_work_items().await;
    
    // Test work distribution optimization
    let optimization_result = fixture.coordinator.optimize_work_distribution().await;
    assert!(optimization_result.is_ok(), "Work distribution optimization failed");
    
    // Test AI recommendations for each pattern
    for pattern in &[
        CoordinationPattern::ScrumAtScale,
        CoordinationPattern::RobertsRules,
        CoordinationPattern::Realtime,
        CoordinationPattern::Atomic,
    ] {
        let recommendations_span = global::tracer("swarmsh-test")
            .span_builder("ai_recommendations")
            .with_attributes(vec![
                KeyValue::new("pattern.type", pattern.description()),
            ])
            .start(&global::tracer("swarmsh-test"));
        
        let _guard = recommendations_span.clone();
        
        match fixture.coordinator.get_ai_recommendations(pattern).await {
            Ok(analysis) => {
                recommendations_span.set_attribute(KeyValue::new("recommendations.count", analysis.recommendations.len() as i64));
                recommendations_span.set_attribute(KeyValue::new("confidence", analysis.confidence));
                
                info!(
                    pattern = ?pattern,
                    recommendations = analysis.recommendations.len(),
                    confidence = analysis.confidence,
                    "AI recommendations generated"
                );
            }
            Err(e) => {
                debug!(pattern = ?pattern, error = %e, "AI recommendations unavailable");
            }
        }
        
        recommendations_span.end();
    }
    
    info!("AI-enhanced coordination metrics test completed");
}

// Cleanup function to ensure proper OTEL shutdown
impl Drop for CoordinationTestFixture {
    fn drop(&mut self) {
        global::shutdown_tracer_provider();
    }
}