//! CLI command generator using WeaverForge
//! 
//! Generates CLI commands from OTEL semantic conventions using WeaverForge templates.
//! Creates both Rust CLI modules and shell script interfaces.

use anyhow::{Context, Result};
use clap::{Arg, Command};
use std::path::PathBuf;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter, prelude::*};

use swarmsh_v2::weaver_forge::{WeaverForge, WeaverConfig};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(fmt::layer().compact())
        .with(EnvFilter::new("info"))
        .init();

    let matches = Command::new("generate-cli")
        .version("1.0.0")
        .about("Generate CLI commands from SwarmSH v2 semantic conventions")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Weaver configuration file")
                .default_value("weaver.yaml")
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("DIR")
                .help("Output directory for generated files")
                .default_value("generated/cli")
        )
        .get_matches();

    let config_path = PathBuf::from(matches.get_one::<String>("config").unwrap());
    let output_dir = PathBuf::from(matches.get_one::<String>("output").unwrap());

    info!("Generating CLI commands with WeaverForge");
    info!("Config: {}", config_path.display());
    info!("Output: {}", output_dir.display());

    // Create output directory
    std::fs::create_dir_all(&output_dir)
        .context("Failed to create output directory")?;

    // Initialize WeaverForge directly from config path
    let mut weaver_forge = WeaverForge::new(&config_path)
        .context("Failed to initialize WeaverForge")?;

    // Generate CLI commands
    weaver_forge.generate_cli_commands(&output_dir)
        .context("Failed to generate CLI commands")?;

    info!("CLI command generation completed successfully!");
    info!("Generated files:");
    info!("  - {}/generated_cli.rs (Rust CLI module)", output_dir.display());
    info!("  - {}/swarmsh_cli.sh (Shell CLI interface)", output_dir.display());

    println!("✅ CLI commands generated successfully!");
    println!("📁 Output directory: {}", output_dir.display());
    println!("🦀 Rust CLI: {}/generated_cli.rs", output_dir.display());
    println!("🐚 Shell CLI: {}/swarmsh_cli.sh", output_dir.display());

    Ok(())
}