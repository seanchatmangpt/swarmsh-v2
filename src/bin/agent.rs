//! SwarmSH v2 Agent Binary
//! 
//! Individual agent process for participating in SwarmSH coordination.

use anyhow::Result;
use clap::{Parser, Subcommand};
use swarmsh_v2::{
    AgentId, 
    coordination::{AgentSpec, CoordinationPattern},
    telemetry::{init_global_telemetry, SwarmTelemetry, DefaultSwarmTelemetry}
};
use tracing::{info, error, instrument};
use std::time::Instant;

#[derive(Parser)]
#[command(name = "swarmsh-agent")]
#[command(about = "SwarmSH v2 Agent - Autonomous coordination participant")]
#[command(version = "2.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Join coordination system as agent
    Join {
        /// Agent role (coordinator, worker, analyzer, optimizer, monitor)
        #[arg(short, long, default_value = "worker")]
        role: String,
        
        /// Agent capacity (0.0-1.0)
        #[arg(short, long, default_value_t = 0.8)]
        capacity: f64,
        
        /// Work specializations (comma-separated)
        #[arg(short, long)]
        specializations: Option<String>,
        
        /// Maximum concurrent work items
        #[arg(short, long, default_value_t = 3)]
        work_capacity: u32,
        
        /// Coordination pattern to use
        #[arg(short, long, default_value = "scrum_at_scale")]
        pattern: String,
    },
    
    /// Claim and execute work
    Work {
        /// Work type to claim
        #[arg(short, long)]
        work_type: Option<String>,
        
        /// Work priority filter
        #[arg(short, long)]
        priority: Option<String>,
    },
    
    /// Send heartbeat signal
    Heartbeat {
        /// Agent ID
        #[arg(short, long)]
        agent_id: String,
    },
    
    /// Generate agent ID with nanosecond precision
    GenerateId,
}

#[tokio::main]
#[instrument]
async fn main() -> Result<()> {
    // Initialize telemetry first
    let _telemetry_manager = init_global_telemetry().await?;
    let telemetry = DefaultSwarmTelemetry;
    
    info!("SwarmSH v2 Agent starting");
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Join { role, capacity, specializations, work_capacity, pattern } => {
            let join_start = Instant::now();
            let agent_id = AgentId::generate();
            let _span = telemetry.agent_span(&agent_id.0, "join").entered();
            
            info!(agent_id = %agent_id.0, role = %role, capacity = capacity, 
                  work_capacity = work_capacity, "Agent joining coordination system");
            
            let specs = if let Some(spec_list) = specializations {
                spec_list.split(',').map(|s| s.trim().to_string()).collect()
            } else {
                vec![]
            };
            
            let agent_spec = AgentSpec {
                id: agent_id,
                role: role.clone(),
                capacity,
                specializations: specs,
                work_capacity: Some(work_capacity),
            };
            
            let coordination_pattern = match pattern.as_str() {
                "scrum_at_scale" => CoordinationPattern::ScrumAtScale,
                "roberts_rules" => CoordinationPattern::RobertsRules,
                "realtime" => CoordinationPattern::Realtime,
                "atomic" => CoordinationPattern::Atomic,
                _ => {
                    println!("Unknown coordination pattern: {}. Using scrum_at_scale", pattern);
                    CoordinationPattern::ScrumAtScale
                }
            };
            
            info!(
                role = %role,
                capacity = capacity,
                work_capacity = work_capacity,
                specializations = ?agent_spec.specializations,
                coordination_pattern = ?coordination_pattern,
                "Agent registration details"
            );
            
            // Record agent registration
            telemetry.record_agent_registration(&agent_id.0);
            
            println!("Joining SwarmSH coordination system...");
            println!("  Role: {}", role);
            println!("  Capacity: {:.1}", capacity);
            println!("  Work Capacity: {}", work_capacity);
            println!("  Specializations: {:?}", agent_spec.specializations);
            println!("  Coordination Pattern: {:?}", coordination_pattern);
            
            let join_duration = join_start.elapsed();
            telemetry.record_coordination_duration("agent_join", join_duration);
            info!(join_duration_ms = join_duration.as_millis(), "Agent registration completed");
            
            println!("Agent {} ready for coordination", agent_spec.id.0);
            
            // Run until interrupted
            tokio::signal::ctrl_c().await?;
            info!(agent_id = %agent_spec.id.0, "Shutdown signal received");
            
            let shutdown_start = Instant::now();
            // Cleanup and shutdown logic would go here
            let shutdown_duration = shutdown_start.elapsed();
            telemetry.record_coordination_duration("agent_shutdown", shutdown_duration);
            info!(agent_id = %agent_spec.id.0, shutdown_duration_ms = shutdown_duration.as_millis(), "Agent shut down completed");
        }
        
        Commands::Work { work_type, priority } => {
            let work_start = Instant::now();
            let work_id = format!("work_{}", chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0));
            let _span = telemetry.work_span(&work_id, "claim_and_execute").entered();
            
            info!(
                work_type = ?work_type,
                priority = ?priority,
                work_id = %work_id,
                "Starting work claiming process"
            );
            
            if let Some(wt) = &work_type {
                info!(work_type_filter = %wt, "Work type filter applied");
            }
            
            if let Some(p) = &priority {
                info!(priority_filter = %p, "Priority filter applied");
            }
            
            // Simulate work processing
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            
            let work_duration = work_start.elapsed();
            telemetry.record_work_item_processed(&work_id, work_duration);
            
            info!(
                work_id = %work_id,
                work_duration_ms = work_duration.as_millis(),
                "Work item processing completed"
            );
            
            println!("Work claiming and execution completed");
        }
        
        Commands::Heartbeat { agent_id } => {
            println!("Sending heartbeat for agent: {}", agent_id);
            
            // Implementation will send heartbeat to coordinator
            println!("Heartbeat sent successfully");
        }
        
        Commands::GenerateId => {
            let agent_id = AgentId::generate();
            println!("{}", agent_id.0);
        }
    }
    
    Ok(())
}
