//! OTEL Weaver code generation demo using WeaverForge
//! 
//! Demonstrates semantic convention-driven code generation for SwarmSH v2

use anyhow::Result;
use swarmsh_v2::weaver_forge::WeaverForge;
use std::path::Path;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize basic logging
    tracing_subscriber::fmt::init();
    
    info!("🔧 SwarmSH v2 OTEL Weaver Code Generation");
    info!("📋 Generating telemetry code from semantic conventions...");
    
    // Initialize WeaverForge with our weaver.yaml configuration
    let mut forge = match WeaverForge::new("weaver.yaml") {
        Ok(forge) => {
            info!("✅ WeaverForge initialized successfully");
            forge
        }
        Err(e) => {
            error!("❌ Failed to initialize WeaverForge: {}", e);
            return Err(e);
        }
    };
    
    // Load and validate semantic conventions
    info!("📖 Loading semantic conventions...");
    let conventions = forge.load_semantic_conventions()?;
    info!("✅ Loaded semantic conventions with {} domains", 
          conventions.as_array().map(|a| a.len()).unwrap_or(0));
    
    // Generate core telemetry code
    info!("🏗️  Generating Rust telemetry code...");
    match forge.generate() {
        Ok(()) => info!("✅ Core telemetry generation complete"),
        Err(e) => {
            error!("❌ Core generation failed: {}", e);
            return Err(e);
        }
    }
    
    // Generate CLI commands from semantic conventions
    info!("🔧 Generating CLI commands...");
    let output_dir = Path::new("generated");
    std::fs::create_dir_all(output_dir)?;
    
    match forge.generate_cli_commands(output_dir) {
        Ok(()) => info!("✅ CLI generation complete"),
        Err(e) => {
            error!("❌ CLI generation failed: {}", e);
            return Err(e);
        }
    }
    
    info!("🎉 OTEL Weaver code generation completed successfully!");
    info!("📁 Generated files:");
    info!("   • generated/generated_cli.rs - CLI command implementations");
    info!("   • generated/swarmsh_cli.sh - Shell interface");
    info!("   • (Additional Rust modules per weaver.yaml configuration)");
    
    // Show semantic convention statistics
    if let Some(conv_array) = conventions.as_array() {
        info!("📊 Semantic Convention Statistics:");
        for (i, convention) in conv_array.iter().enumerate() {
            if let Some(groups) = convention.get("groups").and_then(|g| g.as_array()) {
                let total_attrs: usize = groups.iter()
                    .filter_map(|g| g.get("attributes"))
                    .filter_map(|a| a.as_array())
                    .map(|a| a.len())
                    .sum();
                    
                info!("   • Domain {}: {} groups, {} attributes", 
                      i + 1, groups.len(), total_attrs);
            }
        }
    }
    
    info!("🚀 SwarmSH v2 observability-first architecture ready!");
    Ok(())
}
