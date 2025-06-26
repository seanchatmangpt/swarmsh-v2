//! Shell export functionality test for SwarmSH v2
//! 
//! Tests the core claim that "Complete Rust functionality exports to shell scripts"

use anyhow::Result;
use swarmsh_v2::{SwarmSystem, shell_export::ExportConfig};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🐚 Shell Export Functionality Test");
    println!("Testing: 'Complete Rust functionality → portable shell scripts'");
    
    // Test 1: System initialization 
    println!("\n📊 Test 1: Initialize SwarmSH System");
    let system = SwarmSystem::new().await?;
    println!("✅ System initialized");
    
    // Test 2: Create export configuration
    println!("\n📊 Test 2: Configure Shell Export");
    let export_config = ExportConfig {
        output_dir: PathBuf::from("./test_shell_export"),
        include_telemetry: true,
        include_ai_integration: true,
        optimization_level: 2,
    };
    println!("✅ Export config created");
    
    // Test 3: Attempt shell export
    println!("\n📊 Test 3: Export to Shell Scripts");
    match system.export_to_shell(export_config).await {
        Ok(_) => {
            println!("✅ Shell export completed successfully");
            
            // Test 4: Check exported files
            println!("\n📊 Test 4: Verify Exported Files");
            let export_dir = PathBuf::from("./test_shell_export");
            
            if export_dir.exists() {
                println!("✅ Export directory created: {:?}", export_dir);
                
                // List exported files
                if let Ok(entries) = std::fs::read_dir(&export_dir) {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            println!("📄 Exported: {}", entry.file_name().to_string_lossy());
                        }
                    }
                } else {
                    println!("❌ Could not read export directory");
                }
            } else {
                println!("❌ Export directory not created");
            }
        }
        Err(e) => {
            println!("❌ Shell export failed: {}", e);
            println!("   This indicates shell export functionality needs implementation");
        }
    }
    
    // Test 5: Basic coordination without shell export
    println!("\n📊 Test 5: Core Coordination Functionality");
    system.start().await?;
    
    // Create some spans to test coordination
    let agent_span = system.create_agent_span("test_agent_shell", "coordination_test");
    let _enter = agent_span.enter();
    println!("✅ Agent span created for coordination test");
    drop(_enter);
    
    system.stop().await?;
    println!("✅ System shutdown complete");
    
    println!("\n🎯 Shell Export Test Results:");
    println!("- System initialization: ✅ WORKING");
    println!("- OTEL span creation: ✅ WORKING");
    println!("- Shell export config: ✅ WORKING");
    println!("- Shell export execution: Check output above");
    
    Ok(())
}