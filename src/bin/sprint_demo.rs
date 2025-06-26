//! SwarmSH v2 Sprint Demo - Complete System Demonstration
//! 
//! Showcases all SwarmSH v2 capabilities in a live Scrum at Scale sprint
//! with Robert's Rules of Order governance and real Ollama integration.

use anyhow::Result;
use clap::{Arg, Command};
use std::path::PathBuf;
use swarmsh_v2::demo_sprint::{SprintDemo, ScrumTeam};
use swarmsh_v2::telemetry::TelemetryManager;
use swarmsh_v2::shell_export::{ShellExporter, ExportConfig};
use swarmsh_v2::{SwarmSystem, coordination::CoordinationPattern, AgentId};
use tracing::{info, error, warn};
use tokio::time::{sleep, Duration};
use std::sync::Arc;

/// Generate agent ID with nanosecond precision
fn generate_agent_id() -> AgentId {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("agent_{}", timestamp)
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("SwarmSH v2 Sprint Demo")
        .version("2.0.0")
        .about("Complete demonstration of SwarmSH v2 capabilities with Ollama integration")
        .arg(
            Arg::new("ollama-url")
                .long("ollama-url")
                .value_name("URL")
                .help("Ollama API URL")
                .default_value("http://localhost:11434")
        )
        .arg(
            Arg::new("model")
                .long("model")
                .value_name("MODEL")
                .help("Ollama model to use")
                .default_value("llama3.2")
        )
        .arg(
            Arg::new("sprint-duration")
                .long("sprint-duration")
                .value_name("MINUTES")
                .help("Sprint duration in minutes")
                .default_value("5")
        )
        .arg(
            Arg::new("export-shell")
                .long("export-shell")
                .help("Export sprint artifacts to shell scripts")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("output-dir")
                .long("output-dir")
                .value_name("DIR")
                .help("Output directory for shell exports")
                .default_value("./sprint-artifacts")
        )
        .arg(
            Arg::new("teams")
                .long("teams")
                .value_name("COUNT")
                .help("Number of Scrum teams")
                .default_value("3")
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose logging")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    // Initialize telemetry
    let telemetry_manager = Arc::new(TelemetryManager::new().await?);
    telemetry_manager.start().await?;

    // Initialize logging
    let log_level = if matches.get_flag("verbose") {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };
    
    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .init();

    info!("🚀 Starting SwarmSH v2 Sprint Demo");
    info!("🤖 Ollama URL: {}", matches.get_one::<String>("ollama-url").unwrap());
    info!("🧠 Model: {}", matches.get_one::<String>("model").unwrap());

    // Parse configuration
    let ollama_url = matches.get_one::<String>("ollama-url").unwrap().clone();
    let model = matches.get_one::<String>("model").unwrap().clone();
    let sprint_duration_minutes: u64 = matches.get_one::<String>("sprint-duration")
        .unwrap()
        .parse()
        .unwrap_or(5);
    let team_count: usize = matches.get_one::<String>("teams")
        .unwrap()
        .parse()
        .unwrap_or(3);
    let export_shell = matches.get_flag("export-shell");
    let output_dir = PathBuf::from(matches.get_one::<String>("output-dir").unwrap());

    // Test Ollama connectivity
    info!("🔗 Testing Ollama connectivity...");
    if let Err(e) = test_ollama_connectivity(&ollama_url, &model).await {
        error!("❌ Ollama connectivity test failed: {}", e);
        warn!("💡 Make sure Ollama is running: ollama serve");
        warn!("💡 And the model is available: ollama pull {}", model);
        return Err(e);
    }
    info!("✅ Ollama connectivity verified");

    // Initialize sprint demo
    info!("🏗️  Initializing Sprint Demo...");
    let mut demo = SprintDemo::new().await?;
    
    info!("🎯 Starting {} minute sprint with {} teams", sprint_duration_minutes, team_count);
    info!("📋 Using Robert's Rules of Order for governance");
    
    // Run the complete sprint demonstration
    match run_complete_sprint_demo(&mut demo).await {
        Ok(_) => {
            info!("🎉 Sprint Demo completed successfully!");
            
            // Export shell artifacts if requested
            if export_shell {
                info!("📦 Exporting sprint artifacts to shell scripts...");
                match export_sprint_artifacts(&demo, &output_dir).await {
                    Ok(_) => info!("✅ Sprint artifacts exported to: {}", output_dir.display()),
                    Err(e) => error!("❌ Failed to export artifacts: {}", e),
                }
            }
        }
        Err(e) => {
            error!("❌ Sprint Demo failed: {}", e);
            return Err(e);
        }
    }

    // Cleanup
    telemetry_manager.stop().await?;
    info!("🏁 SwarmSH v2 Sprint Demo finished");

    Ok(())
}

/// Test Ollama connectivity and model availability
async fn test_ollama_connectivity(url: &str, model: &str) -> Result<()> {
    use reqwest::Client;
    use serde_json::json;
    
    let client = Client::new();
    
    // Test basic connectivity
    let health_url = format!("{}/api/tags", url);
    let response = client.get(&health_url)
        .timeout(Duration::from_secs(5))
        .send()
        .await?;
    
    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Ollama server not responding: {}", response.status()));
    }

    // Test model availability with a simple chat
    let chat_url = format!("{}/api/chat", url);
    let test_request = json!({
        "model": model,
        "messages": [
            {
                "role": "user",
                "content": "Test connectivity - respond with just 'OK'"
            }
        ],
        "stream": false
    });

    let chat_response = client.post(&chat_url)
        .json(&test_request)
        .timeout(Duration::from_secs(10))
        .send()
        .await?;

    if !chat_response.status().is_success() {
        return Err(anyhow::anyhow!("Model '{}' not available or failed: {}", model, chat_response.status()));
    }

    Ok(())
}

/// Create demo Scrum teams
fn create_demo_teams(count: usize) -> Vec<ScrumTeam> {
    let team_templates = vec![
        ("Alpha Team", "Backend Systems", vec!["rust", "database", "api"]),
        ("Beta Team", "Frontend Interface", vec!["ui", "ux", "javascript"]),
        ("Gamma Team", "AI Integration", vec!["ai", "ml", "ollama"]),
        ("Delta Team", "DevOps & Infrastructure", vec!["docker", "kubernetes", "monitoring"]),
        ("Epsilon Team", "Quality Assurance", vec!["testing", "automation", "quality"]),
    ];

    (0..count)
        .map(|i| {
            let template = &team_templates[i % team_templates.len()];
            ScrumTeam {
                team_id: template.0.to_string(),
                scrum_master: generate_agent_id(),
                product_owner: generate_agent_id(),
                development_agents: vec![
                    generate_agent_id(),
                    generate_agent_id(),
                    generate_agent_id(),
                ],
                current_sprint: Some("Sprint 1".to_string()),
                velocity: 25.0 + (i as f64 * 3.0), // Varying velocity
                coordination_pattern: CoordinationPattern::ScrumAtScale,
            }
        })
        .collect()
}

/// Run the complete sprint demonstration
async fn run_complete_sprint_demo(demo: &mut SprintDemo) -> Result<()> {
    info!("🚀 Phase 1: Sprint Initialization");
    // Initialize and execute complete sprint
    demo.execute_complete_sprint().await?;
    sleep(Duration::from_secs(2)).await;

    info!("📊 Generating sprint analytics report");
    let sprint_report = demo.generate_sprint_report().await?;
    info!("Sprint report generated with {} characters", sprint_report.len());

    Ok(())
}

/// Export sprint artifacts to shell scripts
async fn export_sprint_artifacts(demo: &SprintDemo, output_dir: &PathBuf) -> Result<()> {
    let shell_exporter = ShellExporter::new().await?;
    
    let export_config = ExportConfig {
        output_dir: output_dir.clone(),
        include_telemetry: true,
        include_ai_integration: true,
        optimization_level: 3, // Maximum optimization
    };

    // Generate a simple demo system for export
    let demo_system = SwarmSystem::new().await?;
    shell_exporter.export_system(&demo_system, export_config.clone()).await?;
    
    // Write sprint report as artifact
    let sprint_report = demo.generate_sprint_report().await?;
    let sprint_report_path = output_dir.join("sprint_report.md");
    std::fs::write(sprint_report_path, sprint_report)?;
    
    info!("📄 Generated shell scripts:");
    info!("  • coordination_helper.sh - Agent coordination");
    info!("  • agent_swarm_orchestrator.sh - Swarm orchestration");
    info!("  • real_agent_coordinator.sh - Real-time coordination");
    info!("  • sprint_artifacts.sh - Sprint-specific utilities");
    info!("  • roberts_rules_governance.sh - Governance procedures");
    info!("  • ollama_integration.sh - AI integration");
    info!("  • telemetry_spans.sh - Observability");
    info!("  • health_monitor.sh - System health");
    info!("  • 8020_automation.sh - Analytics automation");

    Ok(())
}