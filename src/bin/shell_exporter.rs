//! SwarmSH v2 Shell Exporter Binary
//! 
//! Standalone tool for exporting Rust implementation to shell scripts.

use anyhow::Result;
use clap::{Parser, Subcommand};
use swarmsh_v2::{
    SwarmSystem, 
    shell_export::{ShellExporter, ExportConfig},
    telemetry::{init_global_telemetry, SwarmTelemetry, DefaultSwarmTelemetry}
};
use std::path::PathBuf;
use tracing::{info, error, instrument};
use std::time::Instant;

#[derive(Parser)]
#[command(name = "swarmsh-exporter")]
#[command(about = "SwarmSH v2 Shell Exporter - Convert Rust to shell scripts")]
#[command(version = "2.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Export complete system to shell scripts
    Full {
        /// Output directory for shell scripts
        #[arg(short, long, default_value = "./shell-export")]
        output: PathBuf,
        
        /// Include telemetry in export
        #[arg(long, default_value_t = true)]
        telemetry: bool,
        
        /// Include AI integration in export
        #[arg(long, default_value_t = true)]
        ai: bool,
        
        /// Optimization level (1-3)
        #[arg(long, default_value_t = 2)]
        optimization: u8,
    },
    
    /// Export specific component
    Component {
        /// Component to export (coordination, telemetry, health, analytics, ai)
        #[arg(value_name = "COMPONENT")]
        component: String,
        
        /// Output directory for shell scripts
        #[arg(short, long, default_value = "./shell-export")]
        output: PathBuf,
        
        /// Optimization level (1-3)
        #[arg(long, default_value_t = 2)]
        optimization: u8,
    },
    
    /// List available components
    List,
    
    /// Generate shell templates
    Templates {
        /// Output directory for templates
        #[arg(short, long, default_value = "./shell-templates")]
        output: PathBuf,
    },
}

#[tokio::main]
#[instrument]
async fn main() -> Result<()> {
    // Initialize telemetry first
    let _telemetry_manager = init_global_telemetry().await?;
    let telemetry = DefaultSwarmTelemetry;
    
    info!("SwarmSH v2 Shell Exporter starting");
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Full { output, telemetry: include_telemetry, ai, optimization } => {
            let export_start = Instant::now();
            let _span = telemetry.coordination_span("shell_exporter", "full_export").entered();
            
            info!(
                output_dir = ?output,
                include_telemetry = include_telemetry,
                include_ai = ai,
                optimization_level = optimization,
                "Starting full system export"
            );
            
            let system = SwarmSystem::new().await?;
            let config = ExportConfig {
                output_dir: output.clone(),
                include_telemetry: include_telemetry,
                include_ai_integration: ai,
                optimization_level: optimization,
            };
            
            system.export_to_shell(config).await?;
            
            let export_duration = export_start.elapsed();
            telemetry.record_coordination_duration("full_shell_export", export_duration);
            
            println!("✅ Full system export completed!");
            println!("📁 Output directory: {:?}", output);
            println!("📊 Telemetry included: {}", telemetry);
            println!("🤖 AI integration included: {}", ai);
            println!("⚡ Optimization level: {}", optimization);
            
            println!("\n🚀 Generated shell scripts:");
            println!("  coordination_helper.sh     - Main coordination engine");
            println!("  agent_swarm_orchestrator.sh - Agent orchestration");
            println!("  real_agent_coordinator.sh  - Agent coordination protocols");
            println!("  telemetry_spans.sh         - OTEL telemetry generation");
            println!("  8020_automation.sh         - Automated optimization");
            println!("  health_monitor.sh          - System health monitoring");
            
            if ai {
                println!("  claude_integration.sh      - Claude AI interface");
                println!("  ollama_integration.sh      - Ollama local LLM interface");
            }
        }
        
        Commands::Component { component, output, optimization } => {
            let component_start = Instant::now();
            let _span = telemetry.coordination_span("shell_exporter", "component_export").entered();
            
            info!(
                component = %component,
                output_dir = ?output,
                optimization_level = optimization,
                "Starting component export"
            );
            
            let exporter = ShellExporter::new().await?;
            let config = ExportConfig {
                output_dir: output.clone(),
                include_telemetry: true,
                include_ai_integration: true,
                optimization_level: optimization,
            };
            
            exporter.export_component(&component, &config).await?;
            
            let component_duration = component_start.elapsed();
            telemetry.record_coordination_duration("component_shell_export", component_duration);
            
            println!("✅ Component '{}' export completed!", component);
            println!("📁 Output directory: {:?}", output);
        }
        
        Commands::List => {
            println!("Available SwarmSH v2 components for export:");
            println!("  📋 coordination  - Agent coordination engine");
            println!("  📊 telemetry     - OTEL observability pipeline");
            println!("  🏥 health        - Health monitoring system");
            println!("  📈 analytics     - 8020 analytics and DLSS optimization");
            println!("  🤖 ai            - AI integration (Claude + Ollama)");
            
            println!("\nCoordination patterns included:");
            println!("  🏃 scrum_at_scale - Primary coordination pattern");
            println!("  ⚖️  roberts_rules  - Governance and decision making");
            println!("  ⚡ realtime       - High-frequency coordination");
            println!("  🔒 atomic         - File-based atomic operations");
            
            println!("\nCLIAPI principles integration:");
            println!("  🔧 machine_first  - JSON by default, --human flag");
            println!("  📄 yaml_specs     - YAML work specifications");
            println!("  🔄 infinite_loops - Autonomous agent workflows");
            println!("  📊 8020_principle - Pareto optimization");
        }
        
        Commands::Templates { output } => {
            let template_start = Instant::now();
            let _span = telemetry.coordination_span("shell_exporter", "generate_templates").entered();
            
            info!(output_dir = ?output, "Starting template generation");
            
            // Create template directory
            std::fs::create_dir_all(&output)?;
            
            // Generate basic template files
            let templates = vec![
                ("coordination_template.sh", "# SwarmSH v2 Coordination Template\n# Generated by swarmsh-exporter\n"),
                ("agent_template.sh", "# SwarmSH v2 Agent Template\n# Generated by swarmsh-exporter\n"),
                ("telemetry_template.sh", "# SwarmSH v2 Telemetry Template\n# Generated by swarmsh-exporter\n"),
                ("health_template.sh", "# SwarmSH v2 Health Monitoring Template\n# Generated by swarmsh-exporter\n"),
                ("analytics_template.sh", "# SwarmSH v2 Analytics Template\n# Generated by swarmsh-exporter\n"),
            ];
            
            for (filename, content) in templates {
                let filepath = output.join(filename);
                std::fs::write(&filepath, content)?;
                info!(template_file = ?filepath, "Template generated");
                println!("📄 Generated: {:?}", filepath);
            }
            
            let template_duration = template_start.elapsed();
            telemetry.record_coordination_duration("template_generation", template_duration);
            
            info!(
                output_dir = ?output,
                template_duration_ms = template_duration.as_millis(),
                templates_count = 5,
                "Template generation completed"
            );
            
            println!("✅ Shell templates generated!");
            println!("📁 Template directory: {:?}", output);
        }
    }
    
    Ok(())
}
