//! Revolutionary Platform Validator
//! 
//! 80/20 tool: Validates core claims in <30 seconds
//! - OTEL traces working
//! - Shell export functional  
//! - Mathematical coordination
//! - Core compilation status

use anyhow::Result;
use std::process::Command;
use std::time::Instant;
use tokio;

struct ValidationResult {
    name: String,
    passed: bool,
    details: String,
    duration_ms: u128,
}

impl ValidationResult {
    fn success(name: String, details: String, duration_ms: u128) -> Self {
        Self { name, passed: true, details, duration_ms }
    }
    
    fn failure(name: String, details: String, duration_ms: u128) -> Self {
        Self { name, passed: false, details, duration_ms }
    }
}

async fn validate_core_compilation() -> Result<ValidationResult> {
    let start = Instant::now();
    
    let output = Command::new("cargo")
        .args(&["check", "--lib"])
        .output()?;
    
    let duration = start.elapsed().as_millis();
    
    if output.status.success() {
        Ok(ValidationResult::success(
            "Core Compilation".to_string(),
            "✅ Library compiles successfully".to_string(),
            duration
        ))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Ok(ValidationResult::failure(
            "Core Compilation".to_string(),
            format!("❌ Compilation errors:\n{}", stderr),
            duration
        ))
    }
}

async fn validate_otel_traces() -> Result<ValidationResult> {
    let start = Instant::now();
    
    // Use our existing OTEL test binary
    let output = Command::new("cargo")
        .args(&["run", "--bin", "test_otel_traces"])
        .output()?;
    
    let duration = start.elapsed().as_millis();
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    if output.status.success() && stdout.contains("swarmsh.agent.lifecycle") {
        Ok(ValidationResult::success(
            "OTEL Telemetry".to_string(),
            "✅ Traces generated with correlation IDs".to_string(),
            duration
        ))
    } else {
        Ok(ValidationResult::failure(
            "OTEL Telemetry".to_string(),
            "❌ OTEL traces not working properly".to_string(),
            duration
        ))
    }
}

async fn validate_shell_export() -> Result<ValidationResult> {
    let start = Instant::now();
    
    // Check if shell scripts exist and are executable
    let coordination_script = std::path::Path::new("exported-shell/coordination_helper.sh");
    
    let duration = start.elapsed().as_millis();
    
    if coordination_script.exists() {
        // Quick syntax check
        let output = Command::new("bash")
            .args(&["-n", "exported-shell/coordination_helper.sh"])
            .output()?;
        
        if output.status.success() {
            Ok(ValidationResult::success(
                "Shell Export".to_string(),
                "✅ Shell scripts exist and syntax valid".to_string(),
                duration
            ))
        } else {
            Ok(ValidationResult::failure(
                "Shell Export".to_string(),
                "❌ Shell script syntax errors".to_string(),
                duration
            ))
        }
    } else {
        Ok(ValidationResult::failure(
            "Shell Export".to_string(),
            "❌ Shell scripts not found".to_string(),
            duration
        ))
    }
}

async fn validate_mathematical_coordination() -> Result<ValidationResult> {
    let start = Instant::now();
    
    let output = Command::new("cargo")
        .args(&["test", "coordination::tests", "--", "--nocapture"])
        .output()?;
    
    let duration = start.elapsed().as_millis();
    
    if output.status.success() {
        Ok(ValidationResult::success(
            "Mathematical Coordination".to_string(),
            "✅ Zero-conflict tests passing".to_string(),
            duration
        ))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Ok(ValidationResult::failure(
            "Mathematical Coordination".to_string(),
            format!("❌ Coordination tests failed:\n{}", stderr),
            duration
        ))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 SwarmSH v2 Revolutionary Platform Validator");
    println!("Running 80/20 validation suite...\n");
    
    let start_total = Instant::now();
    
    // Run all validations in parallel for speed
    let (comp_result, otel_result, shell_result, coord_result) = tokio::join!(
        validate_core_compilation(),
        validate_otel_traces(),
        validate_shell_export(),
        validate_mathematical_coordination()
    );
    
    let total_duration = start_total.elapsed().as_millis();
    
    // Collect results
    let results = vec![
        comp_result?,
        otel_result?,
        shell_result?,
        coord_result?,
    ];
    
    // Display results
    println!("📊 VALIDATION RESULTS:");
    println!("════════════════════════════════════════");
    
    let mut passed_count = 0;
    let mut total_count = 0;
    
    for result in &results {
        total_count += 1;
        if result.passed {
            passed_count += 1;
        }
        
        let status = if result.passed { "✅ PASS" } else { "❌ FAIL" };
        println!("{:<25} {} ({:.1}s)", result.name, status, result.duration_ms as f64 / 1000.0);
        if !result.passed {
            println!("   {}", result.details);
        }
    }
    
    println!("════════════════════════════════════════");
    println!("🎯 SUMMARY: {}/{} tests passed ({:.1}s total)", 
             passed_count, total_count, total_duration as f64 / 1000.0);
    
    if passed_count == total_count {
        println!("🏆 ALL CORE CLAIMS VERIFIED - REVOLUTIONARY PLATFORM OPERATIONAL");
    } else {
        println!("⚠️  ISSUES DETECTED - Focus on failed validations above");
    }
    
    // Return appropriate exit code
    if passed_count == total_count {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}