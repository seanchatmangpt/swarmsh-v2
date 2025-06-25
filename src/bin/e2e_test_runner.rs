//! End-to-End Test Runner for SwarmSH v2 Shell Export
//! 
//! Standalone binary to run comprehensive shell export tests with detailed reporting

use anyhow::Result;
use clap::{Arg, Command};
use std::path::PathBuf;
use tokio::time::Instant;
use tracing::{info, warn, error, Level};
use std::io::Write;

// Import test modules (they're in the tests directory, so we need to include them)
#[path = "../../tests/e2e_shell_export.rs"]
mod e2e_shell_export;

#[path = "../../tests/shell_script_validators.rs"]
mod shell_script_validators;

use e2e_shell_export::{ShellExportTestHarness, E2ETestConfig};

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("SwarmSH v2 E2E Test Runner")
        .version("2.0.0")
        .about("Comprehensive end-to-end test suite for SwarmSH shell export functionality")
        .arg(
            Arg::new("test-dir")
                .long("test-dir")
                .value_name("DIR")
                .help("Directory for test artifacts")
                .default_value("./test-artifacts")
        )
        .arg(
            Arg::new("ollama-url")
                .long("ollama-url")
                .value_name("URL")
                .help("Ollama API URL for AI testing")
                .default_value("http://localhost:11434")
        )
        .arg(
            Arg::new("ollama-model")
                .long("ollama-model")
                .value_name("MODEL")
                .help("Ollama model for AI testing")
                .default_value("llama3.2")
        )
        .arg(
            Arg::new("enable-ai-tests")
                .long("enable-ai-tests")
                .help("Enable AI integration tests (requires Ollama)")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("enable-performance-tests")
                .long("enable-performance-tests")
                .help("Enable performance benchmarking")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("timeout")
                .long("timeout")
                .value_name("SECONDS")
                .help("Test timeout in seconds")
                .default_value("300")
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose logging")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("output-format")
                .long("output-format")
                .value_name("FORMAT")
                .help("Output format: console, json, junit")
                .default_value("console")
        )
        .arg(
            Arg::new("output-file")
                .long("output-file")
                .value_name("FILE")
                .help("Output file for test results")
        )
        .get_matches();

    // Initialize logging
    let log_level = if matches.get_flag("verbose") {
        Level::DEBUG
    } else {
        Level::INFO
    };
    
    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .init();

    // Parse configuration
    let config = E2ETestConfig {
        test_dir: PathBuf::from(matches.get_one::<String>("test-dir").unwrap()),
        shell_export_dir: PathBuf::from(matches.get_one::<String>("test-dir").unwrap()).join("shell-export"),
        ollama_url: matches.get_one::<String>("ollama-url").unwrap().clone(),
        ollama_model: matches.get_one::<String>("ollama-model").unwrap().clone(),
        timeout_seconds: matches.get_one::<String>("timeout").unwrap().parse().unwrap_or(300),
        enable_performance_tests: matches.get_flag("enable-performance-tests"),
        enable_ai_tests: matches.get_flag("enable-ai-tests"),
    };

    let output_format = matches.get_one::<String>("output-format").unwrap();
    let output_file = matches.get_one::<String>("output-file");

    info!("🚀 Starting SwarmSH v2 E2E Test Suite");
    info!("Configuration:");
    info!("  Test Directory: {}", config.test_dir.display());
    info!("  Ollama URL: {}", config.ollama_url);
    info!("  Ollama Model: {}", config.ollama_model);
    info!("  AI Tests: {}", config.enable_ai_tests);
    info!("  Performance Tests: {}", config.enable_performance_tests);
    info!("  Timeout: {}s", config.timeout_seconds);

    let test_start = Instant::now();
    
    // Run the test suite
    let mut harness = ShellExportTestHarness::new(config).await?;
    let results = harness.run_comprehensive_test_suite().await?;
    
    let test_duration = test_start.elapsed();

    // Output results in requested format
    match output_format.as_str() {
        "console" => print_console_results(&results),
        "json" => {
            let json_output = generate_json_results(&results)?;
            if let Some(file_path) = output_file {
                std::fs::write(file_path, json_output)?;
                info!("JSON results written to: {}", file_path);
            } else {
                println!("{}", json_output);
            }
        }
        "junit" => {
            let junit_output = generate_junit_results(&results)?;
            if let Some(file_path) = output_file {
                std::fs::write(file_path, junit_output)?;
                info!("JUnit results written to: {}", file_path);
            } else {
                println!("{}", junit_output);
            }
        }
        _ => {
            error!("Unknown output format: {}", output_format);
            return Err(anyhow::anyhow!("Invalid output format"));
        }
    }

    // Exit with appropriate code
    if results.overall_success {
        info!("🎉 All tests passed! Total duration: {}ms", test_duration.as_millis());
        std::process::exit(0);
    } else {
        error!("❌ Tests failed! {} of {} tests failed", results.failed_tests, results.total_tests);
        std::process::exit(1);
    }
}

fn print_console_results(results: &e2e_shell_export::TestSuiteResult) {
    println!("\n🧪 SwarmSH v2 Shell Export Test Results");
    println!("==========================================");
    println!("📊 Summary:");
    println!("  Total Tests: {}", results.total_tests);
    println!("  ✅ Passed: {}", results.passed_tests);
    println!("  ❌ Failed: {}", results.failed_tests);
    println!("  ⏱️  Duration: {}ms", results.suite_duration.as_millis());
    println!("  🎯 Overall: {}", if results.overall_success { "✅ PASS" } else { "❌ FAIL" });
    
    println!("\n📋 Detailed Results:");
    for test_result in &results.test_results {
        let status_icon = if test_result.overall_success { "✅" } else { "❌" };
        println!("\n{} Test: {}", status_icon, test_result.test_name);
        println!("   Duration: {}ms", test_result.duration.as_millis());
        
        for op in &test_result.operations {
            let op_status = if op.success { "✅" } else { "❌" };
            println!("     {} {} ({}ms)", op_status, op.operation, op.duration.as_millis());
            
            if !op.success {
                if let Some(ref error) = op.error {
                    println!("       Error: {}", error);
                }
            } else if !op.output.is_empty() {
                println!("       Output: {}", op.output);
            }
        }
        
        // Performance metrics
        if test_result.performance_metrics.execution_time.as_millis() > 0 {
            println!("     ⚡ Performance: {}ms execution", 
                test_result.performance_metrics.execution_time.as_millis());
        }
    }
    
    println!("\n🔍 Test Coverage:");
    let script_validation_tests = results.test_results.iter()
        .filter(|r| r.test_name == "script_validation")
        .count();
    let coordination_tests = results.test_results.iter()
        .filter(|r| r.test_name == "coordination_helper")
        .count();
    let orchestrator_tests = results.test_results.iter()
        .filter(|r| r.test_name == "agent_orchestrator")
        .count();
    let telemetry_tests = results.test_results.iter()
        .filter(|r| r.test_name == "telemetry_spans")
        .count();
    let ai_tests = results.test_results.iter()
        .filter(|r| r.test_name == "ollama_integration")
        .count();
    let e2e_tests = results.test_results.iter()
        .filter(|r| r.test_name == "complete_sprint_e2e")
        .count();
    
    println!("  📝 Script Validation: {}", script_validation_tests);
    println!("  🔄 Coordination: {}", coordination_tests);
    println!("  🤖 Orchestration: {}", orchestrator_tests);
    println!("  📊 Telemetry: {}", telemetry_tests);
    println!("  🧠 AI Integration: {}", ai_tests);
    println!("  🏃‍♂️ End-to-End: {}", e2e_tests);
}

fn generate_json_results(results: &e2e_shell_export::TestSuiteResult) -> Result<String> {
    use serde_json::json;
    
    let json_result = json!({
        "test_suite": "swarmsh_v2_shell_export",
        "summary": {
            "total_tests": results.total_tests,
            "passed_tests": results.passed_tests,
            "failed_tests": results.failed_tests,
            "duration_ms": results.suite_duration.as_millis(),
            "overall_success": results.overall_success
        },
        "test_results": results.test_results.iter().map(|test| {
            json!({
                "name": test.test_name,
                "success": test.overall_success,
                "duration_ms": test.duration.as_millis(),
                "operations": test.operations.iter().map(|op| {
                    json!({
                        "operation": op.operation,
                        "success": op.success,
                        "duration_ms": op.duration.as_millis(),
                        "output": op.output,
                        "error": op.error
                    })
                }).collect::<Vec<_>>(),
                "performance": {
                    "execution_time_ms": test.performance_metrics.execution_time.as_millis(),
                    "memory_usage_mb": test.performance_metrics.memory_usage_mb,
                    "cpu_usage_percent": test.performance_metrics.cpu_usage_percent
                }
            })
        }).collect::<Vec<_>>(),
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    Ok(serde_json::to_string_pretty(&json_result)?)
}

fn generate_junit_results(results: &e2e_shell_export::TestSuiteResult) -> Result<String> {
    use std::fmt::Write;
    
    let mut xml = String::new();
    
    writeln!(xml, r#"<?xml version="1.0" encoding="UTF-8"?>"#)?;
    writeln!(xml, r#"<testsuite name="swarmsh_v2_shell_export" tests="{}" failures="{}" time="{:.3}">"#,
        results.total_tests, results.failed_tests, results.suite_duration.as_secs_f64())?;
    
    for test_result in &results.test_results {
        writeln!(xml, r#"  <testcase name="{}" time="{:.3}">"#,
            test_result.test_name, test_result.duration.as_secs_f64())?;
        
        if !test_result.overall_success {
            let failed_operations: Vec<String> = test_result.operations.iter()
                .filter(|op| !op.success)
                .map(|op| format!("{}: {}", op.operation, op.error.as_deref().unwrap_or("Unknown error")))
                .collect();
            
            writeln!(xml, r#"    <failure message="Test failed">{}</failure>"#,
                failed_operations.join("; "))?;
        }
        
        writeln!(xml, r#"  </testcase>"#)?;
    }
    
    writeln!(xml, r#"</testsuite>"#)?;
    
    Ok(xml)
}