//! SwarmSH v2 Worktree Manager Binary
//!
//! Command-line interface for comprehensive Git worktree management with
//! SwarmSH coordination patterns, AI integration, and shell export capabilities.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use swarmsh_v2::{
    WorktreeManager, WorktreeSpec, WorktreeStatus, CoordinationPattern, TelemetryManager
};
use tracing::{info, error, Level};
use tracing_subscriber;

#[derive(Parser)]
#[command(name = "swarmsh-worktree")]
#[command(about = "SwarmSH v2 Worktree Management System")]
#[command(version = "2.0.0")]
#[command(author = "SwarmSH v2 Contributors")]
struct Cli {
    /// Base path for worktrees
    #[arg(long, default_value = "./worktrees")]
    base_path: PathBuf,
    
    /// Coordination pattern to use
    #[arg(long, default_value = "atomic")]
    pattern: String,
    
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
    
    /// Output format (json, yaml, text)
    #[arg(long, default_value = "text")]
    output: String,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new worktree
    Create {
        /// Name of the worktree
        name: String,
        /// Branch name (defaults to worktree name)
        #[arg(short, long)]
        branch: Option<String>,
        /// Base branch for new branch creation
        #[arg(long)]
        base_branch: Option<String>,
        /// Enable auto-sync
        #[arg(long)]
        auto_sync: bool,
        /// Enable backup
        #[arg(long)]
        backup: bool,
    },
    
    /// Remove a worktree
    Remove {
        /// Name of the worktree to remove
        name: String,
        /// Force removal
        #[arg(short, long)]
        force: bool,
    },
    
    /// List all worktrees
    List {
        /// Show detailed information
        #[arg(short, long)]
        detailed: bool,
    },
    
    /// Show worktree status
    Status {
        /// Name of the worktree
        name: String,
    },
    
    /// Synchronize worktree with upstream
    Sync {
        /// Name of the worktree
        name: String,
    },
    
    /// Backup a worktree
    Backup {
        /// Name of the worktree
        name: String,
        /// Custom backup path
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    
    /// Restore worktree from backup
    Restore {
        /// Name of the worktree
        name: String,
        /// Backup path to restore from
        backup_path: PathBuf,
    },
    
    /// Switch to a worktree
    Switch {
        /// Name of the worktree
        name: String,
    },
    
    /// Deploy worktree to environment
    Deploy {
        /// Name of the worktree
        name: String,
        /// Target environment
        environment: String,
    },
    
    /// Run tests in worktree
    Test {
        /// Name of the worktree
        name: String,
        /// Specific test suite to run
        #[arg(short, long)]
        suite: Option<String>,
    },
    
    /// Run benchmarks in worktree
    Benchmark {
        /// Name of the worktree
        name: String,
    },
    
    /// Get worktree usage analytics
    Usage {
        /// Name of the worktree
        name: String,
    },
    
    /// Optimize worktree performance
    Optimize {
        /// Name of the worktree
        name: String,
    },
    
    /// Merge changes between worktrees
    Merge {
        /// Source worktree name
        source: String,
        /// Target worktree name
        target: String,
    },
    
    /// Coordinate multiple worktrees
    Coordinate {
        /// Coordination pattern
        #[arg(short, long, default_value = "atomic")]
        pattern: String,
        /// Worktree names to coordinate
        worktrees: Vec<String>,
    },
    
    /// Generate telemetry report
    Telemetry {
        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    
    /// Export worktree functionality to shell script
    Export {
        /// Output shell script path
        output: PathBuf,
        /// Template to use
        #[arg(short, long, default_value = "worktree_manager")]
        template: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    let log_level = if cli.verbose { Level::DEBUG } else { Level::INFO };
    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_target(false)
        .init();
    
    // Parse coordination pattern
    let pattern = parse_coordination_pattern(&cli.pattern)?;
    
    // Initialize telemetry and worktree manager
    let telemetry = std::sync::Arc::new(
        TelemetryManager::new().await.context("Failed to initialize telemetry")?
    );
    
    let manager = WorktreeManager::new(cli.base_path.clone(), telemetry)
        .await
        .context("Failed to initialize worktree manager")?;
    
    info!("SwarmSH v2 Worktree Manager initialized with pattern: {:?}", pattern);
    
    // Execute command
    let result = match cli.command {
        Commands::Create { name, branch, base_branch, auto_sync, backup } => {
            let spec = WorktreeSpec {
                name: name.clone(),
                branch,
                base_branch,
                coordination_pattern: pattern,
                agent_assignments: Vec::new(),
                auto_sync,
                backup_enabled: backup,
            };
            
            let state = manager.create_worktree(spec).await?;
            output_result(&cli.output, &serde_json::json!(state))?;
            info!("Worktree '{}' created successfully", name);
            Ok(())
        }
        
        Commands::Remove { name, force } => {
            manager.remove_worktree(&name, force).await?;
            info!("Worktree '{}' removed successfully", name);
            Ok(())
        }
        
        Commands::List { detailed } => {
            let worktrees = manager.list_worktrees().await;
            if detailed {
                output_result(&cli.output, &serde_json::json!(worktrees))?;
            } else {
                for worktree in &worktrees {
                    println!("{} ({}): {:?}", worktree.name, worktree.branch, worktree.status);
                }
            }
            info!("Found {} worktrees", worktrees.len());
            Ok(())
        }
        
        Commands::Status { name } => {
            let state = manager.get_worktree(&name).await?;
            output_result(&cli.output, &serde_json::json!(state))?;
            Ok(())
        }
        
        Commands::Sync { name } => {
            manager.sync_worktree(&name).await?;
            info!("Worktree '{}' synchronized successfully", name);
            Ok(())
        }
        
        Commands::Backup { name, path } => {
            let backup_path = manager.backup_worktree(&name, path).await?;
            info!("Worktree '{}' backed up to {:?}", name, backup_path);
            output_result(&cli.output, &serde_json::json!({
                "worktree": name,
                "backup_path": backup_path
            }))?;
            Ok(())
        }
        
        Commands::Restore { name, backup_path } => {
            manager.restore_worktree(&name, backup_path).await?;
            info!("Worktree '{}' restored successfully", name);
            Ok(())
        }
        
        Commands::Switch { name } => {
            let path = manager.switch_worktree(&name).await?;
            println!("Switched to worktree '{}' at: {}", name, path.display());
            output_result(&cli.output, &serde_json::json!({
                "worktree": name,
                "path": path
            }))?;
            Ok(())
        }
        
        Commands::Deploy { name, environment } => {
            manager.deploy_worktree(&name, &environment).await?;
            info!("Worktree '{}' deployed to environment '{}'", name, environment);
            Ok(())
        }
        
        Commands::Test { name, suite } => {
            let success = manager.test_worktree(&name, suite.as_deref()).await?;
            if success {
                info!("Tests passed for worktree '{}'", name);
            } else {
                error!("Tests failed for worktree '{}'", name);
                std::process::exit(1);
            }
            output_result(&cli.output, &serde_json::json!({
                "worktree": name,
                "test_result": success
            }))?;
            Ok(())
        }
        
        Commands::Benchmark { name } => {
            let results = manager.benchmark_worktree(&name).await?;
            output_result(&cli.output, &results)?;
            info!("Benchmarks completed for worktree '{}'", name);
            Ok(())
        }
        
        Commands::Usage { name } => {
            let usage = manager.get_worktree_usage(&name).await?;
            output_result(&cli.output, &usage)?;
            Ok(())
        }
        
        Commands::Optimize { name } => {
            let optimizations = manager.optimize_worktree(&name).await?;
            for opt in &optimizations {
                println!("• {}", opt);
            }
            output_result(&cli.output, &serde_json::json!({
                "worktree": name,
                "optimizations": optimizations
            }))?;
            info!("Optimization completed for worktree '{}'", name);
            Ok(())
        }
        
        Commands::Merge { source, target } => {
            manager.merge_worktrees(&source, &target).await?;
            info!("Successfully merged '{}' into '{}'", source, target);
            Ok(())
        }
        
        Commands::Coordinate { pattern: coord_pattern, worktrees } => {
            let pattern = parse_coordination_pattern(&coord_pattern)?;
            manager.coordinate_worktrees(pattern).await?;
            info!("Coordination completed for {} worktrees using {:?}", worktrees.len(), pattern);
            Ok(())
        }
        
        Commands::Telemetry { output } => {
            let telemetry = manager.generate_telemetry().await?;
            
            if let Some(output_path) = output {
                tokio::fs::write(&output_path, serde_json::to_string_pretty(&telemetry)?)
                    .await
                    .context("Failed to write telemetry file")?;
                info!("Telemetry written to {:?}", output_path);
            } else {
                output_result(&cli.output, &telemetry)?;
            }
            Ok(())
        }
        
        Commands::Export { output, template } => {
            // This would integrate with the shell exporter
            info!("Exporting worktree functionality to {:?} using template '{}'", output, template);
            
            // Read the Tera template and generate shell script
            let template_content = include_str!("../../templates/worktree_manager.sh.tera");
            let coordination_patterns = vec!["Atomic", "Scrum at Scale", "Roberts Rules", "Real-time"];
            
            // Simple template substitution for now
            let shell_script = template_content
                .replace("{{ coordination_patterns | join(\", \") }}", &coordination_patterns.join(", "))
                .replace("{{ coordination_patterns | join(', ') | lower }}", &coordination_patterns.iter()
                    .map(|p| p.to_lowercase().replace(" ", "_"))
                    .collect::<Vec<_>>()
                    .join(", "));
            
            tokio::fs::write(&output, shell_script)
                .await
                .context("Failed to write shell script")?;
                
            info!("Shell script exported to {:?}", output);
            Ok(())
        }
    };
    
    if let Err(e) = result {
        error!("Command failed: {}", e);
        std::process::exit(1);
    }
    
    Ok(())
}

fn parse_coordination_pattern(pattern: &str) -> Result<CoordinationPattern> {
    match pattern.to_lowercase().as_str() {
        "atomic" => Ok(CoordinationPattern::Atomic),
        "scrum_at_scale" | "scrum-at-scale" | "scrum" => Ok(CoordinationPattern::ScrumAtScale),
        "roberts_rules" | "roberts-rules" | "roberts" => Ok(CoordinationPattern::RobertsRules),
        "realtime" | "real-time" | "rt" => Ok(CoordinationPattern::Realtime),
        _ => Err(anyhow::anyhow!("Unknown coordination pattern: {}", pattern)),
    }
}

fn output_result(format: &str, data: &serde_json::Value) -> Result<()> {
    match format.to_lowercase().as_str() {
        "json" => {
            println!("{}", serde_json::to_string_pretty(data)?);
        }
        "yaml" => {
            println!("{}", serde_yaml::to_string(data)?);
        }
        "text" | _ => {
            if let serde_json::Value::Object(obj) = data {
                for (key, value) in obj {
                    println!("{}: {}", key, value);
                }
            } else {
                println!("{}", serde_json::to_string_pretty(data)?);
            }
        }
    }
    Ok(())
}