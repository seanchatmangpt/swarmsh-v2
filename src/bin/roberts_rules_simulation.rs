//! Roberts Rules of Order 5-Agent AI Simulation
//! 
//! Comprehensive parliamentary procedure simulation with 5 intelligent agents:
//! - 1 Chair (presiding officer with ollama-rs decision making)
//! - 1 Secretary (recording and procedural assistant)
//! - 3 Members (active participants with varied perspectives)
//! 
//! Features full Roberts Rules implementation with AI-powered:
//! - Motion analysis and recommendations
//! - Voting strategy optimization
//! - Parliamentary procedure guidance
//! - Real-time conflict resolution

use anyhow::Result;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, SystemTime, UNIX_EPOCH, Instant};
use tokio::time::sleep;
use tracing::{info, warn, debug, instrument};
use uuid::Uuid;

use swarmsh_v2::{
    ai_integration::{AIIntegration, AgentDecision},
    telemetry::{SwarmTelemetry, DefaultSwarmTelemetry, PerfTimer, CorrelationId},
};

#[derive(Parser)]
#[command(name = "roberts-rules-sim")]
#[command(about = "5-Agent Roberts Rules of Order AI Simulation with ollama-rs")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the full 5-agent Roberts Rules simulation
    Run {
        /// Ollama model for AI decision making
        #[arg(short, long, default_value = "llama2:latest")]
        model: String,
        
        /// Duration of simulation in minutes
        #[arg(short, long, default_value_t = 10)]
        duration: u64,
        
        /// Number of motions to process
        #[arg(long, default_value_t = 5)]
        motions: u32,
        
        /// Enable AI analysis for all decisions
        #[arg(long)]
        ai_enhanced: bool,
        
        /// Output file for meeting minutes
        #[arg(short, long, default_value = "meeting_minutes.json")]
        output: String,
    },
    
    /// Test individual agent capabilities
    TestAgent {
        /// Agent role to test
        #[arg(short, long)]
        role: String,
        
        /// Test scenario
        #[arg(short, long)]
        scenario: String,
    },
    
    /// Generate sample motions for testing
    GenerateMotions {
        /// Number of motions to generate
        #[arg(short, long, default_value_t = 10)]
        count: u32,
    },
}

/// Agent role in Roberts Rules proceedings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AgentRole {
    Chair,
    Secretary,
    Member(u32), // Member number for identification
}

impl AgentRole {
    pub fn name(&self) -> String {
        match self {
            Self::Chair => "Chair".to_string(),
            Self::Secretary => "Secretary".to_string(),
            Self::Member(n) => format!("Member_{}", n),
        }
    }
    
    pub fn responsibilities(&self) -> Vec<&'static str> {
        match self {
            Self::Chair => vec![
                "Presiding over meeting",
                "Maintaining order",
                "Recognizing speakers",
                "Ruling on points of order",
                "Managing debate time",
            ],
            Self::Secretary => vec![
                "Recording meeting minutes",
                "Tracking motion status",
                "Reading previous minutes",
                "Maintaining member roll",
            ],
            Self::Member(_) => vec![
                "Making motions",
                "Seconding motions",
                "Participating in debate",
                "Voting on issues",
            ],
        }
    }
}

/// Motion in Roberts Rules proceedings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Motion {
    pub id: String,
    pub motion_type: MotionType,
    pub description: String,
    pub proposer: String,
    pub seconder: Option<String>,
    pub status: MotionStatus,
    pub submitted_at: SystemTime,
    pub debate_duration: Duration,
    pub votes: HashMap<String, Vote>,
    pub amendments: Vec<Amendment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MotionType {
    Main,           // Primary business motion
    Subsidiary,     // Modifies main motion (amend, refer to committee)
    Privileged,     // Urgent matters (adjourn, recess)
    Incidental,     // Questions of procedure (point of order)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MotionStatus {
    Submitted,
    Seconded,
    UnderDebate,
    ReadyForVote,
    Voted,
    Adopted,
    Rejected,
    Withdrawn,
    Tabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Vote {
    Aye,
    Nay,
    Abstain,
    Present,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amendment {
    pub id: String,
    pub amendment_type: AmendmentType,
    pub text: String,
    pub proposer: String,
    pub seconder: Option<String>,
    pub status: MotionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AmendmentType {
    Strike,
    Insert,
    Substitute,
}

/// Parliamentary agent with AI-powered decision making
#[derive(Debug, Clone)]
pub struct ParliamentaryAgent {
    pub id: String,
    pub role: AgentRole,
    pub ai_integration: Option<AIIntegration>,
    pub telemetry: DefaultSwarmTelemetry,
    pub personality_traits: PersonalityTraits,
    pub voting_history: Vec<VotingRecord>,
    pub speaking_queue_position: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityTraits {
    pub decisiveness: f64,        // 0.0-1.0: Quick vs deliberate decision making
    pub collaboration: f64,       // 0.0-1.0: Individual vs group focused
    pub formality: f64,          // 0.0-1.0: Casual vs strict procedure adherence
    pub innovation: f64,         // 0.0-1.0: Traditional vs progressive thinking
    pub debate_style: DebateStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DebateStyle {
    Analytical,    // Data-driven arguments
    Emotional,     // Appeals to values and feelings  
    Practical,     // Focus on implementation
    Collaborative, // Seeks consensus
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingRecord {
    pub motion_id: String,
    pub vote: Vote,
    pub reasoning: Option<String>,
    pub timestamp: SystemTime,
}

impl ParliamentaryAgent {
    pub async fn new(role: AgentRole, ai_integration: Option<AIIntegration>) -> Result<Self> {
        let agent_id = format!("{}_{}", 
            role.name().to_lowercase(), 
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos()
        );
        
        // Generate personality based on role
        let personality_traits = Self::generate_personality_for_role(&role);
        
        Ok(Self {
            id: agent_id.clone(),
            role,
            ai_integration,
            telemetry: DefaultSwarmTelemetry::new(agent_id),
            personality_traits,
            voting_history: Vec::new(),
            speaking_queue_position: None,
        })
    }
    
    fn generate_personality_for_role(role: &AgentRole) -> PersonalityTraits {
        match role {
            AgentRole::Chair => PersonalityTraits {
                decisiveness: 0.85,
                collaboration: 0.70,
                formality: 0.90,
                innovation: 0.60,
                debate_style: DebateStyle::Analytical,
            },
            AgentRole::Secretary => PersonalityTraits {
                decisiveness: 0.75,
                collaboration: 0.80,
                formality: 0.95,
                innovation: 0.40,
                debate_style: DebateStyle::Practical,
            },
            AgentRole::Member(n) => {
                // Vary member personalities
                match n % 3 {
                    0 => PersonalityTraits {
                        decisiveness: 0.90,
                        collaboration: 0.50,
                        formality: 0.60,
                        innovation: 0.85,
                        debate_style: DebateStyle::Analytical,
                    },
                    1 => PersonalityTraits {
                        decisiveness: 0.60,
                        collaboration: 0.90,
                        formality: 0.70,
                        innovation: 0.70,
                        debate_style: DebateStyle::Collaborative,
                    },
                    _ => PersonalityTraits {
                        decisiveness: 0.75,
                        collaboration: 0.65,
                        formality: 0.55,
                        innovation: 0.80,
                        debate_style: DebateStyle::Practical,
                    },
                }
            }
        }
    }
    
    /// Make AI-powered decision about motion
    #[instrument(skip(self, motion, context))]
    pub async fn analyze_motion(&self, motion: &Motion, context: &str) -> Result<MotionAnalysis> {
        let correlation_id = CorrelationId::new();
        let _perf_timer = PerfTimer::with_correlation("motion_analysis", correlation_id.clone());
        let _span = self.telemetry.span_with_correlation("analyze_motion", &correlation_id).entered();
        
        if let Some(ref ai) = self.ai_integration {
            let decision_context = serde_json::json!({
                "agent_role": self.role,
                "agent_id": self.id,
                "motion": motion,
                "context": context,
                "personality": self.personality_traits,
                "voting_history": self.voting_history.iter().take(5).collect::<Vec<_>>(),
            });
            
            match ai.make_decision(&decision_context, "motion_analysis").await {
                Ok(decision) => {
                    info!(
                        agent_id = %self.id,
                        role = ?self.role,
                        motion_id = %motion.id,
                        ai_confidence = %decision.confidence,
                        correlation_id = %correlation_id,
                        "AI motion analysis completed"
                    );
                    
                    return Ok(MotionAnalysis {
                        support_level: decision.confidence,
                        reasoning: decision.parameters.get("reasoning")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| "AI analysis completed".to_string()),
                        suggested_action: decision.action,
                        amendments_suggested: vec![], // Would parse from AI response
                        procedural_concerns: vec![],
                    });
                }
                Err(e) => {
                    warn!(
                        agent_id = %self.id,
                        error = %e,
                        correlation_id = %correlation_id,
                        "AI motion analysis failed, using rule-based fallback"
                    );
                }
            }
        }
        
        // Rule-based fallback analysis
        self.rule_based_motion_analysis(motion)
    }
    
    fn rule_based_motion_analysis(&self, motion: &Motion) -> Result<MotionAnalysis> {
        let support_level = match (&self.role, &motion.motion_type) {
            (AgentRole::Chair, MotionType::Privileged) => 0.8,
            (AgentRole::Chair, _) => 0.6,
            (AgentRole::Secretary, MotionType::Incidental) => 0.9,
            (AgentRole::Secretary, _) => 0.7,
            (AgentRole::Member(_), _) => 0.5 + (self.personality_traits.collaboration * 0.3),
        };
        
        Ok(MotionAnalysis {
            support_level,
            reasoning: format!("Rule-based analysis based on role: {:?}", self.role),
            suggested_action: "analyze_further".to_string(),
            amendments_suggested: vec![],
            procedural_concerns: vec![],
        })
    }
    
    /// Cast vote with AI reasoning
    #[instrument(skip(self, motion))]
    pub async fn cast_vote(&mut self, motion: &Motion) -> Result<Vote> {
        let correlation_id = CorrelationId::new();
        let _span = self.telemetry.span_with_correlation("cast_vote", &correlation_id).entered();
        
        if let Some(ref ai) = self.ai_integration {
            let voting_context = serde_json::json!({
                "agent_role": self.role,
                "motion": motion,
                "personality": self.personality_traits,
                "voting_history": self.voting_history.iter().take(3).collect::<Vec<_>>(),
            });
            
            match ai.make_decision(&voting_context, "voting_decision").await {
                Ok(decision) => {
                    let vote = self.parse_vote_from_decision(&decision);
                    let reasoning = decision.parameters.get("reasoning")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    
                    // Record vote in history
                    self.voting_history.push(VotingRecord {
                        motion_id: motion.id.clone(),
                        vote: vote.clone(),
                        reasoning,
                        timestamp: SystemTime::now(),
                    });
                    
                    info!(
                        agent_id = %self.id,
                        motion_id = %motion.id,
                        vote = ?vote,
                        ai_confidence = %decision.confidence,
                        correlation_id = %correlation_id,
                        "Vote cast with AI reasoning"
                    );
                    
                    return Ok(vote);
                }
                Err(e) => {
                    warn!(
                        agent_id = %self.id,
                        error = %e,
                        "AI voting failed, using personality-based vote"
                    );
                }
            }
        }
        
        // Personality-based voting fallback
        let vote = if self.personality_traits.decisiveness > 0.7 {
            if self.personality_traits.collaboration > 0.6 { Vote::Aye } else { Vote::Nay }
        } else {
            Vote::Abstain
        };
        
        self.voting_history.push(VotingRecord {
            motion_id: motion.id.clone(),
            vote: vote.clone(),
            reasoning: Some("Personality-based decision".to_string()),
            timestamp: SystemTime::now(),
        });
        
        Ok(vote)
    }
    
    fn parse_vote_from_decision(&self, decision: &AgentDecision) -> Vote {
        match decision.action.to_lowercase().as_str() {
            "support" | "aye" | "yes" => Vote::Aye,
            "oppose" | "nay" | "no" => Vote::Nay,
            "abstain" => Vote::Abstain,
            "present" => Vote::Present,
            _ => Vote::Abstain, // Default to abstain on unclear decision
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionAnalysis {
    pub support_level: f64,
    pub reasoning: String,
    pub suggested_action: String,
    pub amendments_suggested: Vec<String>,
    pub procedural_concerns: Vec<String>,
}

/// Roberts Rules meeting session with 5 agents
pub struct RobertsRulesMeeting {
    pub meeting_id: String,
    pub agents: HashMap<String, ParliamentaryAgent>,
    pub motion_queue: VecDeque<Motion>,
    pub active_motion: Option<Motion>,
    pub meeting_minutes: Vec<MinuteEntry>,
    pub ai_integration: Option<AIIntegration>,
    pub telemetry: DefaultSwarmTelemetry,
    pub session_start: SystemTime,
    pub quorum_met: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinuteEntry {
    pub timestamp: SystemTime,
    pub entry_type: MinuteType,
    pub description: String,
    pub speaker: Option<String>,
    pub motion_reference: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MinuteType {
    CallToOrder,
    MotionSubmitted,
    MotionSeconded,
    DebateOpened,
    DebateClosed,
    VoteCalled,
    VoteResult,
    Adjournment,
    PointOfOrder,
    Amendment,
}

impl RobertsRulesMeeting {
    pub async fn new(ai_integration: Option<AIIntegration>) -> Result<Self> {
        let meeting_id = format!("meeting_{}", 
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos()
        );
        
        info!("Initializing Roberts Rules meeting: {}", meeting_id);
        
        // Create 5 agents with specific roles
        let mut agents = HashMap::new();
        
        // Chair
        let chair = ParliamentaryAgent::new(
            AgentRole::Chair, 
            ai_integration.clone()
        ).await?;
        agents.insert(chair.id.clone(), chair);
        
        // Secretary
        let secretary = ParliamentaryAgent::new(
            AgentRole::Secretary,
            ai_integration.clone()
        ).await?;
        agents.insert(secretary.id.clone(), secretary);
        
        // Three members
        for i in 1..=3 {
            let member = ParliamentaryAgent::new(
                AgentRole::Member(i),
                ai_integration.clone()
            ).await?;
            agents.insert(member.id.clone(), member);
        }
        
        Ok(Self {
            meeting_id: meeting_id.clone(),
            agents,
            motion_queue: VecDeque::new(),
            active_motion: None,
            meeting_minutes: Vec::new(),
            ai_integration,
            telemetry: DefaultSwarmTelemetry::new(meeting_id),
            session_start: SystemTime::now(),
            quorum_met: true, // 5 agents present
        })
    }
    
    /// Run the complete meeting simulation
    #[instrument(skip(self))]
    pub async fn run_simulation(&mut self, duration_minutes: u64, motion_count: u32) -> Result<()> {
        let simulation_start = Instant::now();
        let correlation_id = CorrelationId::new();
        let _span = self.telemetry.span_with_correlation("run_simulation", &correlation_id).entered();
        
        info!(
            meeting_id = %self.meeting_id,
            duration_minutes,
            motion_count,
            correlation_id = %correlation_id,
            "Starting Roberts Rules simulation"
        );
        
        // Call meeting to order
        self.call_to_order().await?;
        
        // Generate motions for the meeting
        self.generate_sample_motions(motion_count).await?;
        
        let end_time = simulation_start + Duration::from_secs(duration_minutes * 60);
        
        // Main meeting loop
        while Instant::now() < end_time && (!self.motion_queue.is_empty() || self.active_motion.is_some()) {
            // Process current motion or move to next
            if self.active_motion.is_none() && !self.motion_queue.is_empty() {
                self.introduce_next_motion().await?;
            }
            
            // Process motion cycle
            let should_complete_motion = if let Some(ref motion) = self.active_motion {
                let motion_clone = motion.clone();
                self.process_motion_cycle_for_motion(motion_clone).await?
            } else {
                false
            };
            
            // Move to next motion after processing
            if should_complete_motion {
                self.active_motion = None;
            }
            
            // Small delay between processing cycles
            sleep(Duration::from_millis(500)).await;
        }
        
        // Adjourn meeting
        self.adjourn_meeting().await?;
        
        let simulation_duration = simulation_start.elapsed();
        info!(
            meeting_id = %self.meeting_id,
            simulation_duration_secs = simulation_duration.as_secs(),
            motions_processed = motion_count,
            correlation_id = %correlation_id,
            "Roberts Rules simulation completed"
        );
        
        Ok(())
    }
    
    async fn call_to_order(&mut self) -> Result<()> {
        self.add_minute_entry(MinuteType::CallToOrder, 
            "Meeting called to order by the Chair".to_string(), 
            self.get_chair_id(), None).await;
        
        info!("Meeting {} called to order with {} members present", 
            self.meeting_id, self.agents.len());
        
        Ok(())
    }
    
    async fn generate_sample_motions(&mut self, count: u32) -> Result<()> {
        let sample_motions = vec![
            ("Main", "Adopt new coordination protocol for enhanced efficiency"),
            ("Main", "Allocate budget for AI integration infrastructure"),
            ("Subsidiary", "Refer telemetry optimization to technical committee"),
            ("Main", "Establish regular performance review schedule"),
            ("Privileged", "Schedule special session for strategic planning"),
            ("Main", "Approve updated security protocols"),
            ("Incidental", "Clarify voting procedures for remote participation"),
            ("Main", "Implement enhanced observability framework"),
            ("Subsidiary", "Amend current quality assurance standards"),
            ("Main", "Authorize research into advanced coordination patterns"),
        ];
        
        for i in 0..count.min(sample_motions.len() as u32) {
            let (motion_type_str, description) = &sample_motions[i as usize];
            let motion_type = match *motion_type_str {
                "Main" => MotionType::Main,
                "Subsidiary" => MotionType::Subsidiary,
                "Privileged" => MotionType::Privileged,
                "Incidental" => MotionType::Incidental,
                _ => MotionType::Main,
            };
            
            // Randomly assign proposer from members
            let proposer = self.get_random_member_id();
            
            let motion = Motion {
                id: format!("motion_{}_{}", i + 1, Uuid::new_v4()),
                motion_type,
                description: description.to_string(),
                proposer,
                seconder: None,
                status: MotionStatus::Submitted,
                submitted_at: SystemTime::now(),
                debate_duration: Duration::from_secs(0),
                votes: HashMap::new(),
                amendments: vec![],
            };
            
            self.motion_queue.push_back(motion);
        }
        
        info!("Generated {} sample motions for the meeting", count);
        Ok(())
    }
    
    async fn introduce_next_motion(&mut self) -> Result<()> {
        if let Some(motion) = self.motion_queue.pop_front() {
            info!("Introducing motion: {} - {}", motion.id, motion.description);
            
            self.add_minute_entry(
                MinuteType::MotionSubmitted,
                format!("Motion submitted: {}", motion.description),
                Some(motion.proposer.clone()),
                Some(motion.id.clone())
            ).await;
            
            self.active_motion = Some(motion);
        }
        Ok(())
    }
    
    async fn process_motion_cycle_for_motion(&mut self, mut motion: Motion) -> Result<bool> {
        let motion_id = motion.id.clone();
        let should_complete = match motion.status {
            MotionStatus::Submitted => {
                // Seek seconder
                if let Some(seconder_id) = self.find_seconder(&motion.id).await? {
                    motion.seconder = Some(seconder_id.clone());
                    motion.status = MotionStatus::Seconded;
                    
                    self.add_minute_entry(
                        MinuteType::MotionSeconded,
                        "Motion seconded".to_string(),
                        Some(seconder_id),
                        Some(motion.id.clone())
                    ).await;
                    false
                } else {
                    // No seconder found, motion dies
                    motion.status = MotionStatus::Withdrawn;
                    self.add_minute_entry(
                        MinuteType::MotionSubmitted,
                        "Motion died for lack of second".to_string(),
                        None,
                        Some(motion.id.clone())
                    ).await;
                    true
                }
            }
            MotionStatus::Seconded => {
                // Open debate
                motion.status = MotionStatus::UnderDebate;
                self.add_minute_entry(
                    MinuteType::DebateOpened,
                    "Debate opened on motion".to_string(),
                    self.get_chair_id(),
                    Some(motion.id.clone())
                ).await;
                
                // Conduct AI-enhanced debate
                self.conduct_debate(&motion).await?;
                false
            }
            MotionStatus::UnderDebate => {
                // Close debate and move to voting
                motion.status = MotionStatus::ReadyForVote;
                self.add_minute_entry(
                    MinuteType::DebateClosed,
                    "Debate closed, motion ready for vote".to_string(),
                    self.get_chair_id(),
                    Some(motion.id.clone())
                ).await;
                false
            }
            MotionStatus::ReadyForVote => {
                // Conduct vote
                self.conduct_vote(&mut motion).await?;
                matches!(motion.status, MotionStatus::Adopted | MotionStatus::Rejected | MotionStatus::Withdrawn)
            }
            _ => {
                // Motion is complete
                true
            }
        };
        
        // Update the active motion
        self.active_motion = Some(motion);
        
        Ok(should_complete)
    }

    #[allow(dead_code)]
    async fn process_motion_cycle(&mut self, motion: &mut Motion) -> Result<()> {
        match motion.status {
            MotionStatus::Submitted => {
                // Seek seconder
                if let Some(seconder_id) = self.find_seconder(&motion.id).await? {
                    motion.seconder = Some(seconder_id.clone());
                    motion.status = MotionStatus::Seconded;
                    
                    self.add_minute_entry(
                        MinuteType::MotionSeconded,
                        "Motion seconded".to_string(),
                        Some(seconder_id),
                        Some(motion.id.clone())
                    ).await;
                } else {
                    // No seconder found, motion dies
                    motion.status = MotionStatus::Withdrawn;
                    self.add_minute_entry(
                        MinuteType::MotionSubmitted,
                        "Motion died for lack of second".to_string(),
                        None,
                        Some(motion.id.clone())
                    ).await;
                }
            }
            MotionStatus::Seconded => {
                // Open debate
                motion.status = MotionStatus::UnderDebate;
                self.add_minute_entry(
                    MinuteType::DebateOpened,
                    "Debate opened on motion".to_string(),
                    self.get_chair_id(),
                    Some(motion.id.clone())
                ).await;
                
                // Conduct AI-enhanced debate
                self.conduct_debate(motion).await?;
            }
            MotionStatus::UnderDebate => {
                // Close debate and move to voting
                motion.status = MotionStatus::ReadyForVote;
                self.add_minute_entry(
                    MinuteType::DebateClosed,
                    "Debate closed, motion ready for vote".to_string(),
                    self.get_chair_id(),
                    Some(motion.id.clone())
                ).await;
            }
            MotionStatus::ReadyForVote => {
                // Conduct vote
                self.conduct_vote(motion).await?;
            }
            _ => {
                // Motion is complete, no further processing needed
            }
        }
        
        Ok(())
    }
    
    async fn find_seconder(&self, motion_id: &str) -> Result<Option<String>> {
        // Find a member willing to second the motion
        for (agent_id, agent) in &self.agents {
            if matches!(agent.role, AgentRole::Member(_)) {
                // Use AI to determine if agent will second
                if let Some(ref ai) = self.ai_integration {
                    let context = serde_json::json!({
                        "motion_id": motion_id,
                        "agent_role": agent.role,
                        "agent_personality": agent.personality_traits,
                    });
                    
                    match ai.make_decision(&context, "second_motion").await {
                        Ok(decision) => {
                            if decision.action == "second" && decision.confidence > 0.6 {
                                return Ok(Some(agent_id.clone()));
                            }
                        }
                        Err(_) => {
                            // Fallback to personality-based decision
                            if agent.personality_traits.collaboration > 0.7 {
                                return Ok(Some(agent_id.clone()));
                            }
                        }
                    }
                }
            }
        }
        Ok(None)
    }
    
    async fn conduct_debate(&mut self, motion: &Motion) -> Result<()> {
        let debate_start = Instant::now();
        
        // Collect member agents first to avoid borrowing issues
        let member_agents: Vec<(String, ParliamentaryAgent)> = self.agents
            .iter()
            .filter(|(_, agent)| matches!(agent.role, AgentRole::Member(_)))
            .map(|(id, agent)| (id.clone(), agent.clone()))
            .collect();
        
        // Each member gets a chance to speak
        for (agent_id, agent) in member_agents {
            let analysis = agent.analyze_motion(motion, "debate_contribution").await?;
            
            info!(
                agent_id = %agent_id,
                motion_id = %motion.id,
                support_level = %analysis.support_level,
                "Agent contributed to debate: {}",
                analysis.reasoning
            );
            
            self.add_minute_entry(
                MinuteType::DebateOpened,
                format!("Debate contribution: {}", analysis.reasoning),
                Some(agent_id),
                Some(motion.id.clone())
            ).await;
        }
        
        // Record debate duration
        let debate_duration = debate_start.elapsed();
        info!("Debate completed in {:.2} seconds", debate_duration.as_secs_f64());
        
        Ok(())
    }
    
    async fn conduct_vote(&mut self, motion: &mut Motion) -> Result<()> {
        self.add_minute_entry(
            MinuteType::VoteCalled,
            "Vote called on motion".to_string(),
            self.get_chair_id(),
            Some(motion.id.clone())
        ).await;
        
        let mut aye_votes = 0;
        let mut nay_votes = 0;
        let mut abstentions = 0;
        let mut present_votes = 0;
        
        // Collect votes from all agents
        for (agent_id, agent) in &mut self.agents {
            let vote = agent.cast_vote(motion).await?;
            motion.votes.insert(agent_id.clone(), vote.clone());
            
            match vote {
                Vote::Aye => aye_votes += 1,
                Vote::Nay => nay_votes += 1,
                Vote::Abstain => abstentions += 1,
                Vote::Present => present_votes += 1,
            }
            
            info!(
                agent_id = %agent_id,
                motion_id = %motion.id,
                vote = ?vote,
                "Vote recorded"
            );
        }
        
        // Determine result
        let total_voting = aye_votes + nay_votes; // Abstentions don't count toward result
        let result = if aye_votes > nay_votes && total_voting >= 3 { // Simple majority with quorum
            motion.status = MotionStatus::Adopted;
            "ADOPTED"
        } else {
            motion.status = MotionStatus::Rejected;
            "REJECTED"
        };
        
        let result_description = format!(
            "Motion {}: Aye: {}, Nay: {}, Abstain: {}, Present: {}",
            result, aye_votes, nay_votes, abstentions, present_votes
        );
        
        self.add_minute_entry(
            MinuteType::VoteResult,
            result_description.clone(),
            self.get_chair_id(),
            Some(motion.id.clone())
        ).await;
        
        info!(
            motion_id = %motion.id,
            result = %result,
            aye_votes,
            nay_votes,
            abstentions,
            "Vote completed"
        );
        
        Ok(())
    }
    
    async fn adjourn_meeting(&mut self) -> Result<()> {
        self.add_minute_entry(
            MinuteType::Adjournment,
            "Meeting adjourned".to_string(),
            self.get_chair_id(),
            None
        ).await;
        
        let session_duration = self.session_start.elapsed()?;
        info!(
            meeting_id = %self.meeting_id,
            session_duration_secs = session_duration.as_secs(),
            total_minutes = self.meeting_minutes.len(),
            "Meeting adjourned"
        );
        
        Ok(())
    }
    
    async fn add_minute_entry(&mut self, entry_type: MinuteType, description: String, speaker: Option<String>, motion_reference: Option<String>) {
        let entry = MinuteEntry {
            timestamp: SystemTime::now(),
            entry_type,
            description: description.clone(),
            speaker,
            motion_reference,
        };
        
        debug!("Meeting minute: {}", description);
        self.meeting_minutes.push(entry);
    }
    
    fn get_chair_id(&self) -> Option<String> {
        self.agents.iter()
            .find(|(_, agent)| matches!(agent.role, AgentRole::Chair))
            .map(|(id, _)| id.clone())
    }
    
    fn get_random_member_id(&self) -> String {
        let members: Vec<_> = self.agents.iter()
            .filter(|(_, agent)| matches!(agent.role, AgentRole::Member(_)))
            .collect();
        
        if !members.is_empty() {
            let index = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() % members.len() as u128) as usize;
            members[index].0.clone()
        } else {
            "unknown_member".to_string()
        }
    }
    
    /// Save meeting minutes to file
    pub async fn save_minutes(&self, output_file: &str) -> Result<()> {
        let minutes_json = serde_json::to_string_pretty(&self.meeting_minutes)?;
        tokio::fs::write(output_file, minutes_json).await?;
        
        info!("Meeting minutes saved to: {}", output_file);
        Ok(())
    }
    
    /// Generate meeting summary
    pub fn generate_summary(&self) -> MeetingSummary {
        let motions_adopted = self.meeting_minutes.iter()
            .filter(|entry| matches!(entry.entry_type, MinuteType::VoteResult) && entry.description.contains("ADOPTED"))
            .count();
        
        let motions_rejected = self.meeting_minutes.iter()
            .filter(|entry| matches!(entry.entry_type, MinuteType::VoteResult) && entry.description.contains("REJECTED"))
            .count();
        
        let total_debate_time = self.meeting_minutes.iter()
            .filter(|entry| matches!(entry.entry_type, MinuteType::DebateOpened))
            .count() as u64 * 30; // Estimate 30 seconds per contribution
        
        MeetingSummary {
            meeting_id: self.meeting_id.clone(),
            session_duration: self.session_start.elapsed().unwrap_or_default(),
            total_motions: motions_adopted + motions_rejected,
            motions_adopted,
            motions_rejected,
            total_debate_time_seconds: total_debate_time,
            agent_count: self.agents.len(),
            total_minutes_entries: self.meeting_minutes.len(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingSummary {
    pub meeting_id: String,
    pub session_duration: Duration,
    pub total_motions: usize,
    pub motions_adopted: usize,
    pub motions_rejected: usize,
    pub total_debate_time_seconds: u64,
    pub agent_count: usize,
    pub total_minutes_entries: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Run { model, duration, motions, ai_enhanced, output } => {
            run_simulation(model, duration, motions, ai_enhanced, output).await?;
        }
        Commands::TestAgent { role, scenario } => {
            test_agent_capability(role, scenario).await?;
        }
        Commands::GenerateMotions { count } => {
            generate_sample_motions(count).await?;
        }
    }

    Ok(())
}

async fn run_simulation(
    model: String,
    duration: u64,
    motions: u32,
    ai_enhanced: bool,
    output: String,
) -> Result<()> {
    info!("Starting Roberts Rules 5-Agent AI Simulation");
    info!("Model: {}, Duration: {}min, Motions: {}, AI Enhanced: {}", 
        model, duration, motions, ai_enhanced);

    // Initialize AI integration if requested
    let ai_integration = if ai_enhanced {
        match AIIntegration::new().await {
            Ok(ai) => {
                info!("AI integration initialized successfully");
                Some(ai)
            }
            Err(e) => {
                warn!("Failed to initialize AI: {}, continuing without AI", e);
                None
            }
        }
    } else {
        None
    };

    // Create and run meeting
    let mut meeting = RobertsRulesMeeting::new(ai_integration).await?;
    meeting.run_simulation(duration, motions).await?;

    // Save results
    meeting.save_minutes(&output).await?;
    
    // Display summary
    let summary = meeting.generate_summary();
    
    println!("\n🏛️  Roberts Rules Simulation Complete!");
    println!("════════════════════════════════════════════════════════════════");
    println!("Meeting ID: {}", summary.meeting_id);
    println!("Duration: {:.2} minutes", summary.session_duration.as_secs_f64() / 60.0);
    println!("Agents: {} (1 Chair, 1 Secretary, 3 Members)", summary.agent_count);
    println!("Total Motions: {}", summary.total_motions);
    println!("├─ Adopted: {}", summary.motions_adopted);
    println!("└─ Rejected: {}", summary.motions_rejected);
    println!("Debate Time: {} seconds", summary.total_debate_time_seconds);
    println!("Meeting Minutes: {} entries", summary.total_minutes_entries);
    println!("Output File: {}", output);

    Ok(())
}

async fn test_agent_capability(role: String, scenario: String) -> Result<()> {
    info!("Testing agent capability: {} with scenario: {}", role, scenario);
    
    let agent_role = match role.as_str() {
        "chair" => AgentRole::Chair,
        "secretary" => AgentRole::Secretary,
        "member" => AgentRole::Member(1),
        _ => return Err(anyhow::anyhow!("Invalid role: {}", role)),
    };
    
    let ai_integration = AIIntegration::new().await.ok();
    let mut agent = ParliamentaryAgent::new(agent_role, ai_integration).await?;
    
    // Create test motion
    let test_motion = Motion {
        id: "test_motion".to_string(),
        motion_type: MotionType::Main,
        description: scenario,
        proposer: "test_proposer".to_string(),
        seconder: None,
        status: MotionStatus::Submitted,
        submitted_at: SystemTime::now(),
        debate_duration: Duration::from_secs(0),
        votes: HashMap::new(),
        amendments: vec![],
    };
    
    // Test analysis
    let analysis = agent.analyze_motion(&test_motion, "test_scenario").await?;
    println!("\n🤖 Agent Analysis Results:");
    println!("Agent: {} ({})", agent.id, role);
    println!("Support Level: {:.2}%", analysis.support_level * 100.0);
    println!("Reasoning: {}", analysis.reasoning);
    println!("Suggested Action: {}", analysis.suggested_action);
    
    // Test voting
    let vote = agent.cast_vote(&test_motion).await?;
    println!("Vote Decision: {:?}", vote);
    
    Ok(())
}

async fn generate_sample_motions(count: u32) -> Result<()> {
    info!("Generating {} sample motions", count);
    
    let sample_motions = vec![
        ("Main", "Implement new distributed coordination protocol"),
        ("Main", "Authorize AI integration for decision support"),
        ("Subsidiary", "Refer technical specifications to committee"),
        ("Main", "Establish quality assurance framework"),
        ("Privileged", "Call for special session on strategic direction"),
        ("Main", "Approve enhanced security measures"),
        ("Incidental", "Clarify remote participation procedures"),
        ("Main", "Adopt observability best practices"),
        ("Subsidiary", "Amend current performance standards"),
        ("Main", "Fund research into advanced coordination patterns"),
    ];
    
    println!("\n📋 Generated Sample Motions:");
    println!("═══════════════════════════════════════════════════════════════");
    
    for i in 0..count.min(sample_motions.len() as u32) {
        let (motion_type, description) = &sample_motions[i as usize];
        let motion_id = format!("motion_{}", i + 1);
        
        println!("{}. [{}] {}", i + 1, motion_type, description);
        println!("   ID: {}", motion_id);
        println!();
    }
    
    Ok(())
}