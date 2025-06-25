//! SwarmSH v2 Coordinator Binary
//! 
//! Main coordinator process for the SwarmSH distributed agent coordination system.

use anyhow::Result;
use clap::{Parser, Subcommand};
use swarmsh_v2::{
    SwarmSystem, 
    shell_export::ExportConfig,
    telemetry::{init_global_telemetry, TelemetryConfig, SwarmTelemetry, DefaultSwarmTelemetry},
    auto_command::{AutoEngine, AutoMode, ValueDetectionConfig}
};
use std::path::PathBuf;
use tracing::{info, error, instrument};
use std::time::Instant;

#[derive(Parser)]
#[command(name = "swarmsh-coordinator")]
#[command(about = "SwarmSH v2 - Observability-First Agent Coordination System")]
#[command(version = "2.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the coordination system
    Start {
        /// Configuration file path
        #[arg(short, long)]
        config: Option<PathBuf>,
        
        /// Enable debug mode
        #[arg(short, long)]
        debug: bool,
    },
    
    /// Export system to shell scripts
    Export {
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
    
    /// Check system health
    Health,
    
    /// Analyze system with 8020 principles
    Analyze,
    
    /// Generate telemetry code from semantic conventions
    Generate,
    
    /// Auto feature implementation using 80/20 principle
    Auto {
        /// Project path to analyze
        #[arg(default_value = ".")]
        path: PathBuf,
        
        /// Auto mode: full, analyze, implement, wave, report
        #[arg(short, long, default_value = "full")]
        mode: String,
        
        /// Number of agents for wave mode
        #[arg(long, default_value_t = 8)]
        agents: usize,
    },
}

#[tokio::main]
#[instrument]
async fn main() -> Result<()> {
    // Initialize telemetry first
    let _telemetry = init_global_telemetry().await?;
    let telemetry = DefaultSwarmTelemetry;
    
    info!("SwarmSH v2 Coordinator starting");
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Start { config, debug } => {
            let start_time = Instant::now();
            let _span = telemetry.coordination_span("coordinator", "start").entered();
            
            info!(debug = debug, "Starting coordinator");
            
            if let Some(config_path) = config {
                info!(config_path = ?config_path, "Using configuration file");
            }
            
            let system = SwarmSystem::new().await?;
            system.start().await?;
            
            let startup_duration = start_time.elapsed();
            telemetry.record_coordination_duration("coordinator_startup", startup_duration);
            info!(startup_duration_ms = startup_duration.as_millis(), "Coordinator started successfully");
            
            // Run until interrupted
            tokio::signal::ctrl_c().await?;
            info!("Shutdown signal received");
            
            let stop_time = Instant::now();
            system.stop().await?;
            let shutdown_duration = stop_time.elapsed();
            telemetry.record_coordination_duration("coordinator_shutdown", shutdown_duration);
            info!(shutdown_duration_ms = shutdown_duration.as_millis(), "SwarmSH v2 Coordinator stopped");
        }
        
        Commands::Export { output, telemetry: include_telemetry, ai, optimization } => {
            let export_start = Instant::now();
            let _span = telemetry.coordination_span("coordinator", "export").entered();
            
            info!(output_dir = ?output, include_telemetry = include_telemetry, 
                  include_ai = ai, optimization_level = optimization, 
                  "Starting shell export");
            
            let system = SwarmSystem::new().await?;
            let config = ExportConfig {
                output_dir: output,
                include_telemetry: include_telemetry,
                include_ai_integration: ai,
                optimization_level: optimization,
            };
            
            system.export_to_shell(config).await?;
            
            let export_duration = export_start.elapsed();
            telemetry.record_coordination_duration("shell_export", export_duration);
            info!(export_duration_ms = export_duration.as_millis(), "Shell export completed successfully");
        }
        
        Commands::Health => {
            let health_start = Instant::now();
            let _span = telemetry.health_span("coordinator", "health_check").entered();
            
            info!("Starting system health check");
            
            let system = SwarmSystem::new().await?;
            let health_report = system.health_monitor.collect_health().await?;
            
            let health_duration = health_start.elapsed();
            telemetry.record_health_check("system", &format!("{:?}", health_report.status), health_duration);
            
            info!(
                score = health_report.score,
                component = %health_report.component,
                status = ?health_report.status,
                bottleneck = ?health_report.bottleneck,
                recommendation = ?health_report.recommendation,
                health_check_duration_ms = health_duration.as_millis(),
                "Health check completed"
            );
            
            println!("Health Report:");
            println!("  Score: {}/100", health_report.score);
            println!("  Component: {}", health_report.component);
            println!("  Status: {:?}", health_report.status);
            
            if let Some(bottleneck) = health_report.bottleneck {
                println!("  Bottleneck: {}", bottleneck);
            }
            
            if let Some(recommendation) = health_report.recommendation {
                println!("  Recommendation: {}", recommendation);
            }
        }
        
        Commands::Analyze => {
            let analyze_start = Instant::now();
            let _span = telemetry.analytics_span("8020", "analyze").entered();
            
            info!("Starting 8020 analysis");
            
            let system = SwarmSystem::new().await?;
            let optimization_report = system.analytics.analyze_8020().await?;
            
            let analyze_duration = analyze_start.elapsed();
            telemetry.record_coordination_duration("8020_analysis", analyze_duration);
            
            info!(
                tier = ?optimization_report.tier,
                value_ratio = optimization_report.value_ratio,
                roi_percentage = optimization_report.roi_percentage,
                flow_efficiency = optimization_report.value_stream.flow_efficiency,
                lead_time_ms = optimization_report.value_stream.lead_time_ms,
                waste_percentage = optimization_report.value_stream.waste_percentage,
                recommendations_count = optimization_report.recommendations.len(),
                analysis_duration_ms = analyze_duration.as_millis(),
                "8020 analysis completed"
            );
            
            println!("8020 Analysis Report:");
            println!("  Tier: {:?}", optimization_report.tier);
            println!("  Value Ratio: {:.2}", optimization_report.value_ratio);
            println!("  ROI: {:.1}%", optimization_report.roi_percentage);
            println!("  Flow Efficiency: {:.1}%", optimization_report.value_stream.flow_efficiency);
            println!("  Lead Time: {}ms", optimization_report.value_stream.lead_time_ms);
            println!("  Waste Reduction: {:.1}%", optimization_report.value_stream.waste_percentage);
            
            println!("Recommendations:");
            for (i, rec) in optimization_report.recommendations.iter().enumerate() {
                println!("  {}. {}", i + 1, rec);
            }
        }
        
        Commands::Generate => {
            let _span = telemetry.coordination_span("coordinator", "generate").entered();
            
            info!("Starting telemetry code generation");
            
            println!("Generating telemetry code from semantic conventions...");
            println!("Run: otel-weaver generate --config weaver.yaml");
            println!("This will generate type-safe telemetry code from the semantic convention specifications");
            
            info!("Telemetry code generation command displayed");
        }
        
        Commands::Auto { path, mode, agents } => {
            let auto_start = Instant::now();
            let _span = telemetry.analytics_span("auto_command", "execute").entered();
            
            info!(path = ?path, mode = %mode, agents = agents, "Starting /auto command");
            
            let system = SwarmSystem::new().await?;
            let auto_engine = AutoEngine::new(system.analytics.as_ref().clone());
            
            let auto_mode = match mode.as_str() {
                "full" => AutoMode::Full,
                "analyze" => AutoMode::Analyze,
                "implement" => AutoMode::Implement,
                "wave" => AutoMode::Wave(agents),
                "report" => AutoMode::Report,
                _ => {
                    error!("Unknown auto mode: {}", mode);
                    println!("Unknown auto mode: {}. Valid modes: full, analyze, implement, wave, report", mode);
                    return Ok(());
                }
            };
            
            println!("🚀 SwarmSH v2 Auto Feature Implementation");
            println!("📊 Using DLSS 80/20 principle to identify high-value features");
            println!("📁 Project path: {:?}", path);
            println!("⚙️  Mode: {:?}", auto_mode);
            
            let result = auto_engine.execute(&path, auto_mode).await?;
            
            let auto_duration = auto_start.elapsed();
            telemetry.record_coordination_duration("auto_command_total", auto_duration);
            
            println!("\n✅ Auto command completed!");
            println!("📊 Features analyzed: {}", result.features_analyzed);
            println!("🛠️  Features implemented: {}", result.features_implemented);
            println!("📈 Value delivered: {:.2}", result.value_delivered);
            println!("⚡ Flow efficiency: {:.1}%", result.flow_efficiency);
            println!("✓  Quality gates passed: {}", result.quality_gates_passed);
            println!("⏱️  Duration: {}ms", auto_duration.as_millis());
            
            if let Some(report) = result.report {
                println!("\n📋 Optimization Report:");
                println!("  Tier: {:?}", report.tier);
                println!("  Value Ratio: {:.2}", report.value_ratio);
                println!("  ROI: {:.1}%", report.roi_percentage);
            }
            
            info!(
                features_analyzed = result.features_analyzed,
                features_implemented = result.features_implemented,
                value_delivered = result.value_delivered,
                duration_ms = auto_duration.as_millis(),
                "Auto command completed"
            );
        }
    }
    
    Ok(())
}
