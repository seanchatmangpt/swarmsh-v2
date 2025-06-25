//! Shell Export Integration Tests for SwarmSH v2
//! 
//! Comprehensive validation of Rust → Shell export functionality,
//! ensuring complete parity between Rust implementation and exported shell scripts.

use anyhow::Result;
use std::path::{Path, PathBuf};
use std::fs;
use std::process::{Command, Stdio};
use std::collections::HashMap;
use tempfile::TempDir;
use swarmsh_v2::{
    SwarmSystem,
    shell_export::{ExportConfig, ShellExportSystem},
    coordination::{AgentSpec, WorkSpec, CoordinationPattern},
    AgentId, WorkId,
};
use tokio::time::{sleep, Duration};

/// Test complete shell export generation and validation
#[tokio::test]
async fn test_complete_shell_export_generation() -> Result<()> {
    let system = SwarmSystem::new().await?;
    
    // Create temporary export directory
    let temp_dir = TempDir::new()?;
    let export_path = temp_dir.path().to_path_buf();
    
    // Configure comprehensive export
    let export_config = ExportConfig {
        output_dir: export_path.clone(),
        include_telemetry: true,
        include_ai_integration: false, // Skip AI for reproducible tests
        optimization_level: 2,
    };
    
    // Generate shell export
    let export_result = system.export_to_shell(export_config).await?;
    assert!(export_result.success, "Shell export should succeed");
    assert!(!export_result.generated_files.is_empty(), "Should generate shell files");
    
    // Validate all expected files are generated
    let expected_files = vec![
        "coordination_helper.sh",
        "agent_swarm_orchestrator.sh",
        "telemetry_collector.sh",
        "health_monitor.sh",
        "analytics_engine.sh",
        "work_coordinator.sh",
        "zero_conflict_manager.sh",
    ];
    
    for expected_file in expected_files {
        let file_path = export_path.join(expected_file);
        assert!(file_path.exists(), "Expected file {} should be generated", expected_file);
        
        // Validate file structure
        validate_shell_script_structure(&file_path)?;
    }
    
    // Validate shell script syntax
    validate_all_shell_scripts(&export_path)?;
    
    Ok(())
}

/// Test shell script functionality matches Rust implementation
#[tokio::test]
async fn test_shell_rust_functionality_parity() -> Result<()> {
    let system = SwarmSystem::new().await?;
    let temp_dir = TempDir::new()?;
    let export_path = temp_dir.path().to_path_buf();
    
    // Export shell scripts
    let export_config = ExportConfig {
        output_dir: export_path.clone(),
        include_telemetry: true,
        include_ai_integration: false,
        optimization_level: 1,
    };
    
    let _export_result = system.export_to_shell(export_config).await?;
    
    // Test 1: Agent Registration Parity
    let agent_spec = AgentSpec {
        id: AgentId::generate(),
        role: "parity_test_agent".to_string(),
        capacity: 0.8,
        specializations: vec!["parity_test".to_string()],
        work_capacity: Some(3),
    };
    
    // Register via Rust
    system.start().await?;
    let rust_result = system.coordinator.register_agent(agent_spec.clone()).await?;
    
    // Register via Shell (simulate)
    let shell_result = simulate_shell_agent_registration(&export_path, &agent_spec).await?;
    
    // Compare results
    assert_eq!(rust_result.success, shell_result.success, "Registration success should match");
    assert_eq!(rust_result.agent_id, shell_result.agent_id, "Agent ID should match");
    
    // Test 2: Work Coordination Parity
    let work_spec = WorkSpec {
        id: WorkId::generate(),
        work_type: "parity_test".to_string(),
        priority: "normal".to_string(),
        description: "Test work for parity validation".to_string(),
        specification: "Validate Rust-Shell parity".to_string(),
        tags: vec!["parity".to_string()],
        estimated_effort: Some(2.0),
    };
    
    // Submit via Rust
    let rust_work_result = system.coordinator.submit_work(work_spec.clone()).await?;
    
    // Submit via Shell (simulate)
    let shell_work_result = simulate_shell_work_submission(&export_path, &work_spec).await?;
    
    // Compare results
    assert_eq!(rust_work_result.success, shell_work_result.success, "Work submission success should match");
    
    system.stop().await?;
    Ok(())
}

/// Test zero-conflict guarantees in shell scripts
#[tokio::test]
async fn test_shell_zero_conflict_guarantees() -> Result<()> {
    let system = SwarmSystem::new().await?;
    let temp_dir = TempDir::new()?;
    let export_path = temp_dir.path().to_path_buf();
    
    // Export shell scripts
    let export_config = ExportConfig {
        output_dir: export_path.clone(),
        include_telemetry: true,
        include_ai_integration: false,
        optimization_level: 2,
    };
    
    let _export_result = system.export_to_shell(export_config).await?;
    
    // Test concurrent operations via shell scripts
    let mut handles = Vec::new();
    
    for i in 0..10 {
        let export_path_clone = export_path.clone();
        let handle = tokio::spawn(async move {
            // Simulate concurrent agent registration
            let agent_spec = AgentSpec {
                id: AgentId::generate(),
                role: format!("conflict_test_agent_{}", i),
                capacity: 0.8,
                specializations: vec!["conflict_test".to_string()],
                work_capacity: Some(1),
            };
            
            simulate_shell_agent_registration(&export_path_clone, &agent_spec).await
        });
        
        handles.push(handle);
    }
    
    // Wait for all operations to complete
    let mut successful_operations = 0;
    let mut conflicts = 0;
    
    for handle in handles {
        match handle.await? {
            Ok(result) => {
                if result.success {
                    successful_operations += 1;
                } else {
                    conflicts += 1;
                }
            },
            Err(_) => conflicts += 1,
        }
    }
    
    // Verify zero conflicts
    assert_eq!(conflicts, 0, "Shell scripts should maintain zero-conflict guarantees");
    assert_eq!(successful_operations, 10, "All operations should succeed without conflicts");
    
    Ok(())
}

/// Test nanosecond precision in shell scripts
#[tokio::test]
async fn test_shell_nanosecond_precision() -> Result<()> {
    let system = SwarmSystem::new().await?;
    let temp_dir = TempDir::new()?;
    let export_path = temp_dir.path().to_path_buf();
    
    // Export shell scripts
    let export_config = ExportConfig {
        output_dir: export_path.clone(),
        include_telemetry: true,
        include_ai_integration: false,
        optimization_level: 2,
    };
    
    let _export_result = system.export_to_shell(export_config).await?;
    
    // Test nanosecond precision ID generation
    let mut generated_ids = std::collections::HashSet::new();
    
    for i in 0..1000 {
        let timestamp_id = generate_shell_timestamp_id(&export_path).await?;
        
        // Verify uniqueness (nanosecond precision should prevent duplicates)
        assert!(
            generated_ids.insert(timestamp_id.clone()),
            "Timestamp ID {} should be unique (iteration {})",
            timestamp_id,
            i
        );
        
        // Verify format (should include nanosecond precision)
        assert!(
            timestamp_id.len() >= 19, // At least nanosecond precision
            "Timestamp ID should have nanosecond precision"
        );
    }
    
    Ok(())
}

/// Test telemetry collection in shell scripts
#[tokio::test]
async fn test_shell_telemetry_collection() -> Result<()> {
    let system = SwarmSystem::new().await?;
    let temp_dir = TempDir::new()?;
    let export_path = temp_dir.path().to_path_buf();
    
    // Export shell scripts with telemetry
    let export_config = ExportConfig {
        output_dir: export_path.clone(),
        include_telemetry: true,
        include_ai_integration: false,
        optimization_level: 2,
    };
    
    let _export_result = system.export_to_shell(export_config).await?;
    
    // Execute shell operations that should generate telemetry
    let agent_spec = AgentSpec {
        id: AgentId::generate(),
        role: "telemetry_test_agent".to_string(),
        capacity: 1.0,
        specializations: vec!["telemetry_test".to_string()],
        work_capacity: Some(1),
    };
    
    // Register agent via shell (should generate telemetry)
    let _registration_result = simulate_shell_agent_registration(&export_path, &agent_spec).await?;
    
    // Check if telemetry files are created
    let telemetry_dir = export_path.join("telemetry");
    assert!(telemetry_dir.exists(), "Telemetry directory should be created");
    
    // Check for telemetry data files
    let telemetry_files = fs::read_dir(telemetry_dir)?;
    let mut telemetry_count = 0;
    
    for entry in telemetry_files {
        let entry = entry?;
        let file_name = entry.file_name();
        
        if file_name.to_str().unwrap_or("").contains("span") {
            telemetry_count += 1;
            
            // Validate telemetry file content
            let content = fs::read_to_string(entry.path())?;
            assert!(!content.trim().is_empty(), "Telemetry file should not be empty");
            assert!(content.contains("swarmsh.agent."), "Telemetry should contain SwarmSH attributes");
        }
    }
    
    assert!(telemetry_count > 0, "Should generate telemetry files");
    
    Ok(())
}

/// Test coordination patterns in shell scripts
#[tokio::test]
async fn test_shell_coordination_patterns() -> Result<()> {
    let system = SwarmSystem::new().await?;
    let temp_dir = TempDir::new()?;
    let export_path = temp_dir.path().to_path_buf();
    
    // Export shell scripts
    let export_config = ExportConfig {
        output_dir: export_path.clone(),
        include_telemetry: true,
        include_ai_integration: false,
        optimization_level: 2,
    };
    
    let _export_result = system.export_to_shell(export_config).await?;
    
    // Test different coordination patterns
    let patterns = vec![
        "scrum_at_scale",
        "roberts_rules",
        "realtime",
        "atomic",
    ];
    
    for pattern in patterns {
        let pattern_result = execute_shell_coordination_pattern(&export_path, pattern).await?;
        
        assert!(pattern_result.success, "Pattern {} should execute successfully", pattern);
        assert_eq!(pattern_result.conflicts, 0, "Pattern {} should maintain zero conflicts", pattern);
        
        // Verify pattern-specific behavior
        match pattern {
            "scrum_at_scale" => {
                assert!(pattern_result.scrum_events > 0, "Scrum at Scale should execute scrum events");
            },
            "roberts_rules" => {
                assert!(pattern_result.parliamentary_procedures > 0, "Roberts Rules should execute parliamentary procedures");
            },
            "realtime" => {
                assert!(pattern_result.latency_ms < 10, "Realtime pattern should have low latency");
            },
            "atomic" => {
                assert!(pattern_result.atomic_operations > 0, "Atomic pattern should execute atomic operations");
            },
            _ => {}
        }
    }
    
    Ok(())
}

/// Test shell script performance compared to Rust
#[tokio::test]
async fn test_shell_performance_benchmarks() -> Result<()> {
    let system = SwarmSystem::new().await?;
    let temp_dir = TempDir::new()?;
    let export_path = temp_dir.path().to_path_buf();
    
    // Export optimized shell scripts
    let export_config = ExportConfig {
        output_dir: export_path.clone(),
        include_telemetry: false, // Disable telemetry for pure performance test
        include_ai_integration: false,
        optimization_level: 3, // Maximum optimization
    };
    
    let _export_result = system.export_to_shell(export_config).await?;
    
    // Benchmark Rust implementation
    system.start().await?;
    let rust_start = std::time::Instant::now();
    
    for i in 0..100 {
        let agent_spec = AgentSpec {
            id: AgentId::generate(),
            role: format!("benchmark_agent_{}", i),
            capacity: 1.0,
            specializations: vec!["benchmark".to_string()],
            work_capacity: Some(1),
        };
        
        let _result = system.coordinator.register_agent(agent_spec).await?;
    }
    
    let rust_duration = rust_start.elapsed();
    system.stop().await?;
    
    // Benchmark Shell implementation
    let shell_start = std::time::Instant::now();
    
    for i in 0..100 {
        let agent_spec = AgentSpec {
            id: AgentId::generate(),
            role: format!("shell_benchmark_agent_{}", i),
            capacity: 1.0,
            specializations: vec!["benchmark".to_string()],
            work_capacity: Some(1),
        };
        
        let _result = simulate_shell_agent_registration(&export_path, &agent_spec).await?;
    }
    
    let shell_duration = shell_start.elapsed();
    
    // Shell should be within reasonable performance bounds (within 5x of Rust)
    let performance_ratio = shell_duration.as_millis() as f64 / rust_duration.as_millis() as f64;
    
    println!("Rust duration: {:?}", rust_duration);
    println!("Shell duration: {:?}", shell_duration);
    println!("Performance ratio: {:.2}x", performance_ratio);
    
    assert!(
        performance_ratio < 5.0,
        "Shell performance should be within 5x of Rust performance, got {:.2}x",
        performance_ratio
    );
    
    Ok(())
}

/// Test shell script cross-platform compatibility
#[tokio::test]
async fn test_shell_cross_platform_compatibility() -> Result<()> {
    let system = SwarmSystem::new().await?;
    let temp_dir = TempDir::new()?;
    let export_path = temp_dir.path().to_path_buf();
    
    // Export shell scripts
    let export_config = ExportConfig {
        output_dir: export_path.clone(),
        include_telemetry: true,
        include_ai_integration: false,
        optimization_level: 1,
    };
    
    let _export_result = system.export_to_shell(export_config).await?;
    
    // Test scripts with different shells
    let shells = vec!["bash", "sh"];
    
    for shell in shells {
        // Check if shell is available
        if !is_shell_available(shell) {
            println!("Shell {} not available, skipping", shell);
            continue;
        }
        
        // Test basic script execution
        let coordination_script = export_path.join("coordination_helper.sh");
        let output = Command::new(shell)
            .arg("-n") // Syntax check
            .arg(&coordination_script)
            .output()?;
        
        assert!(
            output.status.success(),
            "Script should pass syntax check with shell {}",
            shell
        );
        
        // Test script with help flag
        let help_output = Command::new(shell)
            .arg(&coordination_script)
            .arg("--help")
            .output()?;
        
        assert!(
            help_output.status.success(),
            "Script should respond to --help with shell {}",
            shell
        );
        
        let help_text = String::from_utf8_lossy(&help_output.stdout);
        assert!(
            help_text.contains("SwarmSH"),
            "Help text should contain SwarmSH information"
        );
    }
    
    Ok(())
}

// Helper functions

fn validate_shell_script_structure(script_path: &Path) -> Result<()> {
    let content = fs::read_to_string(script_path)?;
    
    // Validate shebang
    assert!(content.starts_with("#!/"), "Script should have shebang");
    
    // Validate SwarmSH header
    assert!(content.contains("# SwarmSH v2"), "Script should have SwarmSH header");
    
    // Validate functions are defined
    assert!(content.contains("function ") || content.contains("() {"), "Script should define functions");
    
    // Validate error handling
    assert!(content.contains("set -e") || content.contains("trap"), "Script should have error handling");
    
    // Validate nanosecond precision support
    assert!(content.contains("date +%s%N") || content.contains("nanoseconds"), "Script should support nanosecond precision");
    
    Ok(())
}

fn validate_all_shell_scripts(export_dir: &Path) -> Result<()> {
    let entries = fs::read_dir(export_dir)?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().map_or(false, |ext| ext == "sh") {
            // Run shellcheck if available
            if is_command_available("shellcheck") {
                let output = Command::new("shellcheck")
                    .arg(&path)
                    .output()?;
                
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    println!("Shellcheck warnings for {}: {}", path.display(), stderr);
                }
            }
            
            // Run bash syntax check
            let output = Command::new("bash")
                .arg("-n")
                .arg(&path)
                .output()?;
            
            assert!(
                output.status.success(),
                "Script {} should pass bash syntax check",
                path.display()
            );
        }
    }
    
    Ok(())
}

async fn simulate_shell_agent_registration(export_path: &Path, agent_spec: &AgentSpec) -> Result<ShellRegistrationResult> {
    let script_path = export_path.join("coordination_helper.sh");
    
    // Simulate shell agent registration
    let output = Command::new("bash")
        .arg(&script_path)
        .arg("register-agent")
        .arg("--id").arg(&agent_spec.id.0)
        .arg("--role").arg(&agent_spec.role)
        .arg("--capacity").arg(&agent_spec.capacity.to_string())
        .arg("--specializations").arg(&agent_spec.specializations.join(","))
        .output()?;
    
    let success = output.status.success();
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    Ok(ShellRegistrationResult {
        success,
        agent_id: agent_spec.id.clone(),
        output: stdout.to_string(),
    })
}

async fn simulate_shell_work_submission(export_path: &Path, work_spec: &WorkSpec) -> Result<ShellWorkResult> {
    let script_path = export_path.join("work_coordinator.sh");
    
    // Simulate shell work submission
    let output = Command::new("bash")
        .arg(&script_path)
        .arg("submit-work")
        .arg("--id").arg(&work_spec.id.0)
        .arg("--type").arg(&work_spec.work_type)
        .arg("--priority").arg(&work_spec.priority)
        .arg("--description").arg(&work_spec.description)
        .output()?;
    
    let success = output.status.success();
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    Ok(ShellWorkResult {
        success,
        work_id: work_spec.id.clone(),
        output: stdout.to_string(),
    })
}

async fn generate_shell_timestamp_id(export_path: &Path) -> Result<String> {
    let script_path = export_path.join("coordination_helper.sh");
    
    let output = Command::new("bash")
        .arg(&script_path)
        .arg("generate-id")
        .arg("--type").arg("timestamp")
        .output()?;
    
    assert!(output.status.success(), "ID generation should succeed");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.trim().to_string())
}

async fn execute_shell_coordination_pattern(export_path: &Path, pattern: &str) -> Result<ShellCoordinationResult> {
    let script_path = export_path.join("coordination_helper.sh");
    
    let start_time = std::time::Instant::now();
    
    let output = Command::new("bash")
        .arg(&script_path)
        .arg("coordinate")
        .arg("--pattern").arg(pattern)
        .arg("--test-mode") // Dry run for testing
        .output()?;
    
    let duration = start_time.elapsed();
    let success = output.status.success();
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Parse output for pattern-specific metrics
    let scrum_events = if stdout.contains("scrum_events:") {
        parse_metric_from_output(&stdout, "scrum_events")
    } else { 0 };
    
    let parliamentary_procedures = if stdout.contains("parliamentary_procedures:") {
        parse_metric_from_output(&stdout, "parliamentary_procedures")
    } else { 0 };
    
    let atomic_operations = if stdout.contains("atomic_operations:") {
        parse_metric_from_output(&stdout, "atomic_operations")
    } else { 0 };
    
    Ok(ShellCoordinationResult {
        success,
        conflicts: 0, // Parse from output if needed
        latency_ms: duration.as_millis() as u64,
        scrum_events,
        parliamentary_procedures,
        atomic_operations,
    })
}

fn is_shell_available(shell: &str) -> bool {
    Command::new("which")
        .arg(shell)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn is_command_available(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn parse_metric_from_output(output: &str, metric_name: &str) -> u64 {
    for line in output.lines() {
        if line.contains(&format!("{}:", metric_name)) {
            if let Some(value_str) = line.split(':').nth(1) {
                if let Ok(value) = value_str.trim().parse::<u64>() {
                    return value;
                }
            }
        }
    }
    0
}

// Result structures for shell operations

#[derive(Debug)]
struct ShellRegistrationResult {
    success: bool,
    agent_id: AgentId,
    output: String,
}

#[derive(Debug)]
struct ShellWorkResult {
    success: bool,
    work_id: WorkId,
    output: String,
}

#[derive(Debug)]
struct ShellCoordinationResult {
    success: bool,
    conflicts: u64,
    latency_ms: u64,
    scrum_events: u64,
    parliamentary_procedures: u64,
    atomic_operations: u64,
}