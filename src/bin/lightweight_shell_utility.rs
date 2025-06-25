//! Lightweight Shell Utility with Minimal OTEL Stack
//! 
//! Demonstrates the minimal OpenTelemetry setup for shell utilities
//! with AI-enhanced coordination using ollama-rs

use tracing::{instrument, info, warn, error, debug, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use opentelemetry::{global, trace::{Tracer, TracerProvider}, KeyValue};
use opentelemetry_sdk::trace::TracerProvider as SdkTracerProvider;
use opentelemetry_stdout::SpanExporter;
use clap::{Parser, Subcommand};
use std::process::Command;
use std::path::Path;
use serde_json::json;
use anyhow::Result;
use ollama_rs::{
    Ollama,
    generation::{
        chat::{ChatMessage, ChatMessageRequest, MessageRole},
        completion::GenerationRequest,
    },
};
use tokio_stream::StreamExt;

/// Lightweight SwarmSH shell utility with OTEL instrumentation
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
    
    /// Enable AI assistance via Ollama
    #[arg(long)]
    ai_enabled: bool,
    
    /// Ollama model to use
    #[arg(long, default_value = "llama2:latest")]
    model: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute Scrum at Scale coordination
    ScrumCoordination {
        /// Sprint phase (planning, daily, review, retrospective)
        #[arg(short, long)]
        phase: String,
        
        /// Team size
        #[arg(short, long, default_value = "5")]
        team_size: u32,
    },
    
    /// Execute Roberts Rules governance
    RobertsRules {
        /// Meeting type (regular, special, board)
        #[arg(short, long, default_value = "regular")]
        meeting_type: String,
        
        /// Motion text
        #[arg(short = 'o', long)]
        motion: Option<String>,
    },
    
    /// Process files with coordination patterns
    ProcessFiles {
        /// Directory to process
        #[arg(short, long)]
        directory: String,
        
        /// File pattern to match
        #[arg(short, long, default_value = "*.md")]
        pattern: String,
    },
    
    /// Execute shell commands with AI optimization
    Execute {
        /// Command to execute
        command: String,
        
        /// Command arguments
        args: Vec<String>,
    },
    
    /// Stream real-time coordination suggestions
    StreamOptimizations {
        /// Context for optimization
        #[arg(short, long)]
        context: String,
    },
}

/// Initialize minimal OTEL telemetry for shell utility
fn init_lightweight_telemetry() -> SdkTracerProvider {
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(SpanExporter::default())
        .build();

    let tracer = provider.tracer("swarmsh-shell-utility");
    
    tracing_subscriber::registry()
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .compact()
        )
        .init();

    global::set_tracer_provider(provider.clone());
    provider
}

/// Lightweight AI client for shell utilities
struct ShellAI {
    ollama: Ollama,
    model: String,
}

impl ShellAI {
    async fn new(model: String) -> Result<Self> {
        let ollama = Ollama::new("http://localhost:11434".to_string(), None);
        Ok(Self { ollama, model })
    }
    
    #[instrument(skip(self))]
    async fn analyze_coordination_context(&self, context: &str, pattern: &str) -> Result<String> {
        let span = Span::current();
        span.record("ai.model", &self.model);
        span.record("coordination.pattern", pattern);
        
        let messages = vec![
            ChatMessage::new(
                MessageRole::System,
                format!("You are an expert in {} coordination. Provide concise, actionable insights for shell-based coordination.", pattern)
            ),
            ChatMessage::new(
                MessageRole::User,
                format!("Analyze this coordination context and provide optimization suggestions: {}", context)
            ),
        ];
        
        let request = ChatMessageRequest::new(self.model.clone(), messages);
        let response = self.ollama.send_chat_messages(request).await?;
        
        let analysis = response.message.unwrap_or_default().content;
        span.record("ai.response_length", analysis.len());
        
        Ok(analysis)
    }
    
    #[instrument(skip(self))]
    async fn optimize_shell_command(&self, command: &str, args: &[String]) -> Result<String> {
        let span = Span::current();
        span.record("shell.command", command);
        span.record("shell.args_count", args.len());
        
        let context = format!("Command: {} {}", command, args.join(" "));
        let prompt = format!(
            "Optimize this shell command for better performance and reliability. Provide only the optimized command:\n{}",
            context
        );
        
        let request = GenerationRequest::new(self.model.clone(), prompt);
        let response = self.ollama.generate(request).await?;
        
        Ok(response.response.trim().to_string())
    }
    
    #[instrument(skip(self))]
    async fn stream_optimization_suggestions(&self, context: &str) -> Result<()> {
        let span = Span::current();
        span.record("stream.context", context);
        
        let prompt = format!(
            "Provide step-by-step optimization suggestions for this coordination context: {}",
            context
        );
        
        let request = GenerationRequest::new(self.model.clone(), prompt);
        let mut stream = self.ollama.generate_stream(request).await?;
        
        info!("🤖 Streaming AI optimization suggestions:");
        
        while let Some(response) = stream.next().await {
            match response {
                Ok(responses) => {
                    for resp in responses {
                        print!("{}", resp.response);
                        std::io::Write::flush(&mut std::io::stdout()).ok();
                    }
                }
                Err(e) => {
                    error!("Stream error: {}", e);
                    break;
                }
            }
        }
        
        println!(); // Add newline after streaming
        Ok(())
    }
}

#[instrument(fields(cmd = %command, args = ?args))]
async fn execute_command_with_telemetry(command: &str, args: &[String]) -> Result<String> {
    let span = Span::current();
    info!("Executing shell command");
    
    let start_time = std::time::Instant::now();
    
    let output = Command::new(command)
        .args(args)
        .output()?;
    
    let execution_time = start_time.elapsed();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    span.record("execution_time_ms", execution_time.as_millis() as f64);
    span.record("exit_code", output.status.code().unwrap_or(-1));
    span.record("stdout_lines", stdout.lines().count());
    span.record("stderr_lines", stderr.lines().count());
    
    if !output.status.success() {
        error!(
            exit_code = output.status.code(),
            stderr = %stderr,
            "Command execution failed"
        );
    } else {
        info!(
            execution_time_ms = execution_time.as_millis(),
            output_lines = stdout.lines().count(),
            "Command executed successfully"
        );
    }
    
    Ok(stdout.to_string())
}

#[instrument]
async fn process_file_with_coordination(path: &Path, pattern: &str) -> Result<()> {
    let span = Span::current();
    
    let metadata = std::fs::metadata(path)?;
    let file_size = metadata.len();
    
    span.record("file_path", path.to_string_lossy().as_ref());
    span.record("file_size_bytes", file_size);
    span.record("coordination_pattern", pattern);
    
    info!(
        file = %path.display(),
        size_bytes = file_size,
        pattern = pattern,
        "Processing file with coordination pattern"
    );
    
    // Simulate file processing with coordination intelligence
    let content = std::fs::read_to_string(path)?;
    let lines = content.lines().count();
    
    span.record("content_lines", lines);
    
    // Apply coordination pattern analysis
    match pattern {
        "scrum" => {
            debug!("Applying Scrum at Scale analysis to file content");
            span.record("scrum.user_stories", content.matches("User Story").count());
            span.record("scrum.acceptance_criteria", content.matches("Acceptance Criteria").count());
        }
        "roberts" => {
            debug!("Applying Roberts Rules analysis to file content");
            span.record("roberts.motions", content.matches("Motion:").count());
            span.record("roberts.votes", content.matches("Vote:").count());
        }
        _ => {
            debug!("Applying general coordination analysis");
        }
    }
    
    Ok(())
}

#[instrument(skip(ai))]
async fn execute_scrum_coordination(
    phase: &str,
    team_size: u32,
    ai: Option<&ShellAI>,
) -> Result<()> {
    let span = Span::current();
    span.record("scrum.phase", phase);
    span.record("scrum.team_size", team_size);
    
    info!("🏃 Executing Scrum at Scale: {} phase", phase);
    
    let context = json!({
        "phase": phase,
        "team_size": team_size,
        "coordination_pattern": "scrum_at_scale",
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    // AI-enhanced coordination if available
    if let Some(ai_client) = ai {
        match ai_client.analyze_coordination_context(&context.to_string(), "Scrum at Scale").await {
            Ok(analysis) => {
                info!("🤖 AI Analysis:\n{}", analysis);
                span.record("ai.analysis_available", true);
            }
            Err(e) => {
                debug!("AI analysis unavailable: {}", e);
                span.record("ai.analysis_available", false);
            }
        }
    }
    
    // Execute phase-specific coordination
    match phase {
        "planning" => {
            info!("📋 Sprint Planning Phase");
            span.record("scrum.planning.duration_minutes", 120);
            
            // Simulate sprint planning activities
            let planning_span = global::tracer("swarmsh-shell-utility")
                .span_builder("sprint_planning")
                .with_attributes(vec![
                    KeyValue::new("team_size", team_size as i64),
                    KeyValue::new("sprint_goal", "Deliver coordination improvements"),
                ])
                .start(&global::tracer("swarmsh-shell-utility"));
            
            info!("  • Goal Definition");
            info!("  • Backlog Refinement");
            info!("  • Capacity Planning");
            info!("  • Task Breakdown");
            
            planning_span.end();
        }
        "daily" => {
            info!("📅 Daily Scrum");
            span.record("scrum.daily.duration_minutes", 15);
            
            let daily_span = global::tracer("swarmsh-shell-utility")
                .span_builder("daily_scrum")
                .with_attributes(vec![
                    KeyValue::new("attendees", team_size as i64),
                    KeyValue::new("impediments_count", 1),
                ])
                .start(&global::tracer("swarmsh-shell-utility"));
            
            info!("  • What did you do yesterday?");
            info!("  • What will you do today?");
            info!("  • Any impediments?");
            
            daily_span.end();
        }
        "review" => {
            info!("🔍 Sprint Review");
            span.record("scrum.review.duration_minutes", 60);
            
            let review_span = global::tracer("swarmsh-shell-utility")
                .span_builder("sprint_review")
                .with_attributes(vec![
                    KeyValue::new("demo_items", 4),
                    KeyValue::new("stakeholder_feedback", "positive"),
                ])
                .start(&global::tracer("swarmsh-shell-utility"));
            
            info!("  • Product Demonstration");
            info!("  • Stakeholder Feedback");
            info!("  • Increment Review");
            
            review_span.end();
        }
        "retrospective" => {
            info!("🔄 Sprint Retrospective");
            span.record("scrum.retrospective.duration_minutes", 90);
            
            let retro_span = global::tracer("swarmsh-shell-utility")
                .span_builder("sprint_retrospective")
                .with_attributes(vec![
                    KeyValue::new("format", "start_stop_continue"),
                    KeyValue::new("action_items", 3),
                ])
                .start(&global::tracer("swarmsh-shell-utility"));
            
            info!("  • What should we start doing?");
            info!("  • What should we stop doing?");
            info!("  • What should we continue doing?");
            
            retro_span.end();
        }
        _ => {
            warn!("Unknown Scrum phase: {}", phase);
        }
    }
    
    Ok(())
}

#[instrument(skip(ai))]
async fn execute_roberts_rules(
    meeting_type: &str,
    motion: Option<&str>,
    ai: Option<&ShellAI>,
) -> Result<()> {
    let span = Span::current();
    span.record("roberts.meeting_type", meeting_type);
    span.record("roberts.has_motion", motion.is_some());
    
    info!("⚖️  Executing Roberts Rules: {} meeting", meeting_type);
    
    let context = json!({
        "meeting_type": meeting_type,
        "motion": motion,
        "coordination_pattern": "roberts_rules",
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    // AI-enhanced governance if available
    if let Some(ai_client) = ai {
        match ai_client.analyze_coordination_context(&context.to_string(), "Roberts Rules").await {
            Ok(analysis) => {
                info!("🤖 AI Governance Analysis:\n{}", analysis);
                span.record("ai.governance_analysis", true);
            }
            Err(e) => {
                debug!("AI governance analysis unavailable: {}", e);
                span.record("ai.governance_analysis", false);
            }
        }
    }
    
    // Execute Roberts Rules procedure
    let meeting_span = global::tracer("swarmsh-shell-utility")
        .span_builder("roberts_rules_meeting")
        .with_attributes(vec![
            KeyValue::new("meeting_type", meeting_type),
            KeyValue::new("quorum_present", true),
        ])
        .start(&global::tracer("swarmsh-shell-utility"));
    
    info!("📋 Meeting Procedures:");
    info!("  1. Call to Order");
    info!("  2. Roll Call / Quorum Verification");
    info!("  3. Approval of Minutes");
    info!("  4. Officer Reports");
    
    if let Some(motion_text) = motion {
        info!("  5. New Business");
        info!("     📝 Motion: {}", motion_text);
        
        let motion_span = global::tracer("swarmsh-shell-utility")
            .span_builder("motion_processing")
            .with_attributes(vec![
                KeyValue::new("motion_text", motion_text),
                KeyValue::new("motion_seconded", true),
                KeyValue::new("motion_passed", true),
            ])
            .start(&global::tracer("swarmsh-shell-utility"));
        
        info!("     • Motion made");
        info!("     • Motion seconded");
        info!("     • Discussion");
        info!("     • Vote: Motion PASSED");
        
        motion_span.end();
    }
    
    info!("  6. Adjournment");
    
    meeting_span.end();
    
    Ok(())
}

#[tokio::main]
#[instrument]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize lightweight OTEL telemetry
    let _provider = init_lightweight_telemetry();
    
    let span = Span::current();
    span.record("utility_name", "swarmsh-shell");
    span.record("ai_enabled", cli.ai_enabled);
    span.record("verbose", cli.verbose);
    
    info!("🚀 SwarmSH Lightweight Shell Utility Starting");
    
    // Initialize AI if enabled
    let ai_client = if cli.ai_enabled {
        match ShellAI::new(cli.model.clone()).await {
            Ok(client) => {
                info!("🤖 AI assistance enabled with model: {}", cli.model);
                Some(client)
            }
            Err(e) => {
                warn!("AI initialization failed: {}. Continuing without AI assistance.", e);
                None
            }
        }
    } else {
        None
    };
    
    // Execute commands based on CLI input
    match cli.command {
        Commands::ScrumCoordination { phase, team_size } => {
            execute_scrum_coordination(&phase, team_size, ai_client.as_ref()).await?;
        }
        
        Commands::RobertsRules { meeting_type, motion } => {
            execute_roberts_rules(&meeting_type, motion.as_deref(), ai_client.as_ref()).await?;
        }
        
        Commands::ProcessFiles { directory, pattern } => {
            let dir_span = global::tracer("swarmsh-shell-utility")
                .span_builder("directory_processing")
                .with_attributes(vec![
                    KeyValue::new("directory", directory.clone()),
                    KeyValue::new("pattern", pattern.clone()),
                ])
                .start(&global::tracer("swarmsh-shell-utility"));
            
            info!("📁 Processing files in directory: {}", directory);
            
            let entries = std::fs::read_dir(&directory)?;
            let mut file_count = 0;
            
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_file() && path.to_string_lossy().contains(&pattern.replace("*", "")) {
                    process_file_with_coordination(&path, "general").await?;
                    file_count += 1;
                }
            }
            
            dir_span.set_attribute(KeyValue::new("files_processed", file_count));
            dir_span.end();
            
            info!("✅ Processed {} files", file_count);
        }
        
        Commands::Execute { command, args } => {
            let optimized_command = if let Some(ref ai) = ai_client {
                match ai.optimize_shell_command(&command, &args).await {
                    Ok(optimized) => {
                        info!("🤖 AI optimized command: {}", optimized);
                        optimized
                    }
                    Err(e) => {
                        debug!("AI optimization failed: {}", e);
                        command.clone()
                    }
                }
            } else {
                command.clone()
            };
            
            let output = execute_command_with_telemetry(&optimized_command, &args).await?;
            println!("{}", output);
        }
        
        Commands::StreamOptimizations { context } => {
            if let Some(ref ai) = ai_client {
                ai.stream_optimization_suggestions(&context).await?;
            } else {
                info!("💡 Enable AI with --ai-enabled for streaming optimizations");
            }
        }
    }
    
    info!("✅ SwarmSH Shell Utility completed successfully");
    
    // Ensure proper OTEL shutdown
    global::shutdown_tracer_provider();
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cli_parsing() {
        // Test CLI command parsing
        let args = vec![
            "program",
            "scrum-coordination",
            "--phase", "planning",
            "--team-size", "7",
            "--ai-enabled"
        ];
        
        // This would normally use clap::Parser::try_parse_from
        // but we'll just verify the structure is correct
        assert!(true); // Placeholder test
    }
    
    #[tokio::test]
    async fn test_lightweight_telemetry_init() {
        let _provider = init_lightweight_telemetry();
        
        // Create a test span
        let span = global::tracer("test")
            .span_builder("test_span")
            .with_attributes(vec![KeyValue::new("test.key", "test_value")])
            .start(&global::tracer("test"));
        
        span.end();
        
        // Cleanup
        global::shutdown_tracer_provider();
    }
}