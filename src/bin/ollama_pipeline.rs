//! Ollama-WeaverForge Pipeline Runner
//! 
//! Binary to execute the intelligent code generation pipeline that uses
//! local LLMs to analyze semantic conventions, generate enhanced code,
//! and iterate for continuous improvement.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use swarmsh_v2::ollama_weaver_pipeline::{OllamaWeaverPipeline, PipelineConfig};
use tracing::{info, error, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(name = "ollama-pipeline")]
#[command(about = "Intelligent code generation pipeline using Ollama and WeaverForge")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the complete pipeline
    Run {
        /// Ollama model to use
        #[arg(short, long, default_value = "llama2:latest")]
        model: String,
        
        /// WeaverForge configuration file
        #[arg(short, long, default_value = "weaver.yaml")]
        config: PathBuf,
        
        /// Output directory for generated code
        #[arg(short, long, default_value = "generated/ollama-pipeline")]
        output: PathBuf,
        
        /// Maximum iterations
        #[arg(long, default_value_t = 5)]
        max_iterations: u32,
        
        /// Quality threshold (0.0-1.0)
        #[arg(long, default_value_t = 0.85)]
        quality_threshold: f64,
        
        /// Disable test execution
        #[arg(long)]
        no_tests: bool,
        
        /// Run only one iteration (no loop)
        #[arg(long)]
        no_loop: bool,
    },
    
    /// Test Ollama connection
    TestConnection {
        /// Ollama model to test
        #[arg(short, long, default_value = "llama2:latest")]
        model: String,
    },
    
    /// Show pipeline status and history
    Status {
        /// Results file to analyze
        #[arg(short, long, default_value = "pipeline-results.json")]
        results: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            model,
            config,
            output,
            max_iterations,
            quality_threshold,
            no_tests,
            no_loop,
        } => {
            run_pipeline(
                model,
                config,
                output,
                max_iterations,
                quality_threshold,
                !no_tests,
                !no_loop,
            ).await?;
        }
        Commands::TestConnection { model } => {
            test_ollama_connection(&model).await?;
        }
        Commands::Status { results } => {
            show_pipeline_status(&results).await?;
        }
    }

    Ok(())
}

async fn run_pipeline(
    model: String,
    config: PathBuf,
    output: PathBuf,
    max_iterations: u32,
    quality_threshold: f64,
    run_tests: bool,
    enable_loop: bool,
) -> Result<()> {
    info!("Starting Ollama-WeaverForge pipeline");
    info!("Model: {}", model);
    info!("Config: {}", config.display());
    info!("Output: {}", output.display());
    info!("Max iterations: {}", max_iterations);
    info!("Quality threshold: {}", quality_threshold);
    info!("Run tests: {}", run_tests);
    info!("Enable loop: {}", enable_loop);

    // Create pipeline configuration
    let pipeline_config = PipelineConfig {
        model,
        weaver_config: config,
        output_dir: output.clone(),
        max_iterations,
        quality_threshold,
        run_tests,
        enable_loop,
    };

    // Create output directory
    tokio::fs::create_dir_all(&output).await
        .context("Failed to create output directory")?;

    // Initialize pipeline
    let mut pipeline = OllamaWeaverPipeline::new(pipeline_config).await
        .context("Failed to initialize pipeline")?;

    info!("Pipeline initialized successfully");

    // Execute pipeline
    let results = pipeline.execute().await
        .context("Pipeline execution failed")?;

    // Get summary
    let summary = pipeline.get_summary();
    
    info!("Pipeline execution complete!");
    info!("Total iterations: {}", summary.total_iterations);
    info!("Final quality score: {:.2}%", summary.final_quality_score * 100.0);
    info!("Total duration: {:?}", summary.total_duration);
    info!("Test pass rate: {:.2}%", summary.test_pass_rate);
    info!("Improvements applied: {}", summary.improvements_applied.len());

    // Save results to file
    let results_file = output.join("pipeline-results.json");
    let results_json = serde_json::to_string_pretty(&results)?;
    tokio::fs::write(&results_file, results_json).await?;
    info!("Results saved to: {}", results_file.display());

    // Save summary
    let summary_file = output.join("pipeline-summary.json");
    let summary_json = serde_json::to_string_pretty(&summary)?;
    tokio::fs::write(&summary_file, summary_json).await?;
    info!("Summary saved to: {}", summary_file.display());

    // Display iteration details
    println!("\n📊 Iteration Details:");
    println!("┌─────────────┬───────────────┬─────────────┬──────────────┬─────────────┐");
    println!("│ Iteration   │ Quality Score │ Tests       │ Duration     │ Improvements│");
    println!("├─────────────┼───────────────┼─────────────┼──────────────┼─────────────┤");
    
    for result in &results {
        let test_summary = if result.test_results.is_empty() {
            "N/A".to_string()
        } else {
            let passed = result.test_results.iter().filter(|t| t.passed).count();
            let total = result.test_results.len();
            format!("{}/{}", passed, total)
        };
        
        println!("│ {:11} │ {:13.2}% │ {:11} │ {:12.2}s │ {:11} │",
            result.iteration,
            result.quality_score * 100.0,
            test_summary,
            result.duration.as_secs_f64(),
            result.improvements_applied.len()
        );
    }
    
    println!("└─────────────┴───────────────┴─────────────┴──────────────┴─────────────┘");

    // Show generated files
    println!("\n📁 Generated Files:");
    let mut entries = tokio::fs::read_dir(&output).await?;
    while let Some(entry) = entries.next_entry().await? {
        if entry.file_type().await?.is_file() {
            println!("  - {}", entry.file_name().to_string_lossy());
        }
    }

    Ok(())
}

async fn test_ollama_connection(model: &str) -> Result<()> {
    use ollama_rs::{Ollama, generation::GenerationRequest};
    
    info!("Testing Ollama connection with model: {}", model);
    
    let ollama = Ollama::default();
    
    // List available models
    info!("Checking available models...");
    match ollama.list_local_models().await {
        Ok(models) => {
            println!("\n📦 Available Models:");
            for model in models {
                println!("  - {}", model.name);
            }
        }
        Err(e) => {
            error!("Failed to list models: {}", e);
            return Err(e.into());
        }
    }
    
    // Test generation
    info!("Testing generation with model: {}", model);
    let prompt = "Say 'Hello, SwarmSH!' and nothing else.";
    let request = GenerationRequest::new(model.to_string(), prompt.to_string());
    
    match ollama.generate(request).await {
        Ok(response) => {
            println!("\n✅ Connection successful!");
            println!("Model response: {}", response.response.trim());
        }
        Err(e) => {
            error!("Generation failed: {}", e);
            println!("\n❌ Connection failed!");
            println!("Error: {}", e);
            println!("\nMake sure:");
            println!("1. Ollama is running (check with: ollama list)");
            println!("2. The model '{}' is installed (install with: ollama pull {})", model, model);
        }
    }
    
    Ok(())
}

async fn show_pipeline_status(results_file: &PathBuf) -> Result<()> {
    use swarmsh_v2::ollama_weaver_pipeline::PipelineResult;
    
    if !results_file.exists() {
        println!("No results file found at: {}", results_file.display());
        println!("Run the pipeline first with: ollama-pipeline run");
        return Ok(());
    }
    
    let content = tokio::fs::read_to_string(results_file).await?;
    let results: Vec<PipelineResult> = serde_json::from_str(&content)?;
    
    if results.is_empty() {
        println!("No pipeline results found.");
        return Ok(());
    }
    
    println!("\n📊 Pipeline Execution History");
    println!("════════════════════════════════════════════════════════════════");
    
    for result in &results {
        println!("\n🔄 Iteration {}", result.iteration);
        println!("├─ Quality Score: {:.2}%", result.quality_score * 100.0);
        println!("├─ Duration: {:.2}s", result.duration.as_secs_f64());
        println!("├─ Generated Files: {}", result.generated_files.len());
        
        if !result.test_results.is_empty() {
            let passed = result.test_results.iter().filter(|t| t.passed).count();
            println!("├─ Tests: {}/{} passed", passed, result.test_results.len());
            
            for test in &result.test_results {
                let status = if test.passed { "✅" } else { "❌" };
                println!("│  {} {} ({:.2}s)", status, test.scenario, test.duration.as_secs_f64());
                if let Some(error) = &test.error {
                    println!("│     Error: {}", error);
                }
            }
        }
        
        if !result.improvements_applied.is_empty() {
            println!("└─ Improvements Applied:");
            for improvement in &result.improvements_applied {
                println!("   • {}", improvement);
            }
        }
    }
    
    // Calculate overall statistics
    let total_duration: std::time::Duration = results.iter()
        .map(|r| r.duration)
        .sum();
    
    let avg_quality = results.iter()
        .map(|r| r.quality_score)
        .sum::<f64>() / results.len() as f64;
    
    let all_tests: Vec<_> = results.iter()
        .flat_map(|r| &r.test_results)
        .collect();
    
    let test_pass_rate = if all_tests.is_empty() {
        100.0
    } else {
        let passed = all_tests.iter().filter(|t| t.passed).count() as f64;
        let total = all_tests.len() as f64;
        (passed / total) * 100.0
    };
    
    println!("\n📈 Overall Statistics");
    println!("════════════════════════════════════════════════════════════════");
    println!("Total Iterations: {}", results.len());
    println!("Total Duration: {:.2}s", total_duration.as_secs_f64());
    println!("Average Quality Score: {:.2}%", avg_quality * 100.0);
    println!("Overall Test Pass Rate: {:.2}%", test_pass_rate);
    println!("Final Quality Score: {:.2}%", 
        results.last().map(|r| r.quality_score * 100.0).unwrap_or(0.0)
    );
    
    Ok(())
}