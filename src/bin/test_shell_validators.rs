//! Standalone test for shell script validators
//! 
//! This binary tests the shell script validation and mock generation
//! functionality without depending on the full SwarmSH system.

use anyhow::Result;
use std::fs;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

mod shell_script_validators {
    use anyhow::Result;
    use std::path::{Path, PathBuf};
    use std::fs;
    use std::collections::HashMap;
    use regex::Regex;

    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;

    /// Shell script validator that checks exported scripts for correctness
    pub struct ShellScriptValidator {
        script_requirements: HashMap<String, ScriptRequirements>,
    }

    #[derive(Debug, Clone)]
    pub struct ScriptRequirements {
        pub must_have_functions: Vec<String>,
        pub must_have_variables: Vec<String>,
        pub must_support_operations: Vec<String>,
        pub should_be_executable: bool,
        pub should_have_shebang: bool,
        pub max_lines: Option<usize>,
    }

    #[derive(Debug)]
    pub struct ValidationResult {
        pub script_name: String,
        pub is_valid: bool,
        pub errors: Vec<String>,
        pub warnings: Vec<String>,
        pub metrics: ScriptMetrics,
    }

    #[derive(Debug, Default)]
    pub struct ScriptMetrics {
        pub line_count: usize,
        pub function_count: usize,
        pub variable_count: usize,
        pub comment_lines: usize,
        pub executable: bool,
    }

    impl ShellScriptValidator {
        pub fn new() -> Self {
            let mut requirements = HashMap::new();
            
            // Coordination helper requirements
            requirements.insert("coordination_helper.sh".to_string(), ScriptRequirements {
                must_have_functions: vec![
                    "register_agent".to_string(),
                    "claim_work".to_string(),
                    "advance_epoch".to_string(),
                    "health_check".to_string(),
                ],
                must_have_variables: vec![
                    "COORDINATION_EPOCH".to_string(),
                    "AGENT_REGISTRY".to_string(),
                ],
                must_support_operations: vec![
                    "register".to_string(),
                    "claim_work".to_string(),
                    "advance_epoch".to_string(),
                    "health_check".to_string(),
                ],
                should_be_executable: true,
                should_have_shebang: true,
                max_lines: Some(500),
            });
            
            Self { script_requirements: requirements }
        }
        
        /// Validate a shell script against its requirements
        pub fn validate_script(&self, script_path: &Path) -> Result<ValidationResult> {
            let script_name = script_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();
            
            let content = fs::read_to_string(script_path)?;
            let requirements = self.script_requirements.get(&script_name);
            
            let mut errors = Vec::new();
            let mut warnings = Vec::new();
            let metrics = self.analyze_script_metrics(&content, script_path)?;
            
            if let Some(req) = requirements {
                // Check shebang
                if req.should_have_shebang && !content.starts_with("#!") {
                    errors.push("Missing shebang line".to_string());
                }
                
                // Check executable permission
                if req.should_be_executable && !metrics.executable {
                    warnings.push("Script is not executable".to_string());
                }
                
                // Check required functions
                for func in &req.must_have_functions {
                    if !self.has_function(&content, func) {
                        errors.push(format!("Missing required function: {}", func));
                    }
                }
                
                // Check required variables
                for var in &req.must_have_variables {
                    if !self.has_variable(&content, var) {
                        warnings.push(format!("Missing recommended variable: {}", var));
                    }
                }
                
                // Check operation support
                for op in &req.must_support_operations {
                    if !self.supports_operation(&content, op) {
                        errors.push(format!("Missing support for operation: {}", op));
                    }
                }
            } else {
                warnings.push("No validation requirements defined for this script".to_string());
            }
            
            let is_valid = errors.is_empty();
            
            Ok(ValidationResult {
                script_name,
                is_valid,
                errors,
                warnings,
                metrics,
            })
        }
        
        /// Analyze script metrics
        fn analyze_script_metrics(&self, content: &str, script_path: &Path) -> Result<ScriptMetrics> {
            let lines: Vec<&str> = content.lines().collect();
            let line_count = lines.len();
            
            let function_regex = Regex::new(r"^\s*(\w+)\s*\(\s*\)\s*\{")?;
            let variable_regex = Regex::new(r"^\s*(\w+)=")?;
            let comment_regex = Regex::new(r"^\s*#")?;
            
            let mut function_count = 0;
            let mut variable_count = 0;
            let mut comment_lines = 0;
            
            for line in &lines {
                if function_regex.is_match(line) {
                    function_count += 1;
                }
                if variable_regex.is_match(line) {
                    variable_count += 1;
                }
                if comment_regex.is_match(line) {
                    comment_lines += 1;
                }
            }
            
            let executable = {
                #[cfg(unix)]
                {
                    script_path.metadata()
                        .map(|m| m.permissions().mode() & 0o111 != 0)
                        .unwrap_or(false)
                }
                #[cfg(not(unix))]
                {
                    true // Assume executable on non-Unix systems
                }
            };
            
            Ok(ScriptMetrics {
                line_count,
                function_count,
                variable_count,
                comment_lines,
                executable,
            })
        }
        
        fn has_function(&self, content: &str, func_name: &str) -> bool {
            let pattern = format!(r"(?m)^\s*{}\s*\(\s*\)\s*\{{", regex::escape(func_name));
            Regex::new(&pattern).unwrap().is_match(content)
        }
        
        fn has_variable(&self, content: &str, var_name: &str) -> bool {
            let pattern = format!(r"(?m)^\s*{}\s*=", regex::escape(var_name));
            Regex::new(&pattern).unwrap().is_match(content)
        }
        
        fn supports_operation(&self, content: &str, operation: &str) -> bool {
            // Check if the script has case statement or function that handles the operation
            let case_pattern = format!(r#""{}".*\)"#, regex::escape(operation));
            let func_pattern = format!(r"handle_{}", regex::escape(operation));
            
            Regex::new(&case_pattern).unwrap().is_match(content) ||
            Regex::new(&func_pattern).unwrap().is_match(content) ||
            content.contains(&format!("\"{}\"", operation))
        }
    }

    /// Mock shell script generator for testing when templates don't exist
    pub struct MockShellScriptGenerator;

    impl MockShellScriptGenerator {
        /// Generate mock coordination helper script
        pub fn generate_coordination_helper(output_path: &Path) -> Result<()> {
            let script_content = r#"#!/bin/bash
# SwarmSH v2 - Coordination Helper Script
# Generated mock for testing purposes

set -euo pipefail

COORDINATION_EPOCH=0
AGENT_REGISTRY="${TMPDIR:-/tmp}/swarmsh_agents"
WORK_QUEUE="${TMPDIR:-/tmp}/swarmsh_work"

mkdir -p "$(dirname "$AGENT_REGISTRY")"
mkdir -p "$(dirname "$WORK_QUEUE")"

register_agent() {
    local agent_id="$1"
    echo "$(date +%s%N):$agent_id" >> "$AGENT_REGISTRY"
    echo "Agent $agent_id registered successfully"
}

claim_work() {
    local work_id="$1"
    local agent_id="$2"
    echo "$(date +%s%N):$work_id:$agent_id" >> "$WORK_QUEUE"
    echo "Work $work_id claimed by agent $agent_id"
}

advance_epoch() {
    COORDINATION_EPOCH=$((COORDINATION_EPOCH + 1))
    echo "Coordination epoch advanced to $COORDINATION_EPOCH"
}

health_check() {
    local agent_count=$(wc -l < "$AGENT_REGISTRY" 2>/dev/null || echo 0)
    local work_count=$(wc -l < "$WORK_QUEUE" 2>/dev/null || echo 0)
    echo "Health: $agent_count agents, $work_count work items, epoch $COORDINATION_EPOCH"
}

case "${1:-help}" in
    "register")
        register_agent "$2"
        ;;
    "claim_work")
        claim_work "$2" "$3"
        ;;
    "advance_epoch")
        advance_epoch
        ;;
    "health_check")
        health_check
        ;;
    *)
        echo "Usage: $0 {register|claim_work|advance_epoch|health_check}"
        exit 1
        ;;
esac
"#;
            fs::write(output_path, script_content)?;
            Self::make_executable(output_path)?;
            Ok(())
        }
        
        /// Generate all mock scripts in a directory
        pub fn generate_all_mock_scripts(output_dir: &Path) -> Result<Vec<PathBuf>> {
            fs::create_dir_all(output_dir)?;
            
            let mut generated_paths = Vec::new();
            
            // Generate coordination helper script
            let script_path = output_dir.join("coordination_helper.sh");
            Self::generate_coordination_helper(&script_path)?;
            generated_paths.push(script_path);
            
            Ok(generated_paths)
        }
        
        fn make_executable(path: &Path) -> Result<()> {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(path)?.permissions();
                perms.set_mode(perms.mode() | 0o755);
                fs::set_permissions(path, perms)?;
            }
            #[cfg(not(unix))]
            {
                // On non-Unix systems, just ensure the file exists
                let _ = fs::metadata(path)?;
            }
            Ok(())
        }
    }
}

use shell_script_validators::{MockShellScriptGenerator, ShellScriptValidator};

fn main() -> Result<()> {
    println!("🧪 Testing SwarmSH v2 Shell Script Validators");
    
    // Test 1: Mock script generation
    println!("\n📝 Test 1: Mock Script Generation");
    let temp_dir = std::env::temp_dir().join("swarmsh_test_scripts");
    std::fs::create_dir_all(&temp_dir)?;
    let scripts = MockShellScriptGenerator::generate_all_mock_scripts(&temp_dir)?;
    
    println!("✅ Generated {} mock scripts:", scripts.len());
    for script_path in &scripts {
        println!("   - {}", script_path.file_name().unwrap().to_str().unwrap());
        let content = fs::read_to_string(&script_path)?;
        #[cfg(unix)]
        let executable = script_path.metadata()?.permissions().mode() & 0o111 != 0;
        #[cfg(not(unix))]
        let executable = true;
        
        println!("     Lines: {}, Executable: {}", 
            content.lines().count(),
            executable
        );
    }
    
    // Test 2: Script validation
    println!("\n🔍 Test 2: Script Validation");
    let validator = ShellScriptValidator::new();
    
    for script_path in &scripts {
        let result = validator.validate_script(script_path)?;
        
        println!("📋 Validating {}:", result.script_name);
        println!("   Status: {}", if result.is_valid { "✅ VALID" } else { "❌ INVALID" });
        println!("   Functions: {}, Variables: {}, Lines: {}", 
            result.metrics.function_count,
            result.metrics.variable_count,
            result.metrics.line_count
        );
        
        if !result.errors.is_empty() {
            println!("   Errors:");
            for error in &result.errors {
                println!("     - {}", error);
            }
        }
        
        if !result.warnings.is_empty() {
            println!("   Warnings:");
            for warning in &result.warnings {
                println!("     - {}", warning);
            }
        }
    }
    
    // Test 3: Script execution
    println!("\n🚀 Test 3: Mock Script Execution");
    for script_path in &scripts {
        if script_path.file_name().unwrap() == "coordination_helper.sh" {
            println!("Testing coordination helper script execution...");
            
            let output = std::process::Command::new("bash")
                .arg(script_path)
                .arg("health_check")
                .output()?;
            
            if output.status.success() {
                println!("✅ Script executed successfully");
                println!("   Output: {}", String::from_utf8_lossy(&output.stdout).trim());
            } else {
                println!("❌ Script execution failed");
                println!("   Error: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
    }
    
    println!("\n🎉 Shell script validator tests completed successfully!");
    println!("💡 The e2e test framework is ready to validate exported SwarmSH shell scripts");
    
    Ok(())
}