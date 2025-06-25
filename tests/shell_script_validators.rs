//! Shell Script Validators and Mock Generators
//! 
//! Provides validation tools for exported shell scripts and mock script generation
//! for testing purposes when templates don't exist yet.

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
        
        // Agent orchestrator requirements
        requirements.insert("agent_swarm_orchestrator.sh".to_string(), ScriptRequirements {
            must_have_functions: vec![
                "init_swarm".to_string(),
                "distribute_work".to_string(),
                "coordinate_agents".to_string(),
                "status".to_string(),
            ],
            must_have_variables: vec![
                "SWARM_SIZE".to_string(),
                "COORDINATION_PATTERN".to_string(),
            ],
            must_support_operations: vec![
                "init_swarm".to_string(),
                "distribute_work".to_string(),
                "coordinate_agents".to_string(),
                "status".to_string(),
            ],
            should_be_executable: true,
            should_have_shebang: true,
            max_lines: Some(800),
        });
        
        // Telemetry script requirements
        requirements.insert("telemetry_spans.sh".to_string(), ScriptRequirements {
            must_have_functions: vec![
                "create_span".to_string(),
                "record_metric".to_string(),
                "export_traces".to_string(),
            ],
            must_have_variables: vec![
                "OTEL_EXPORTER".to_string(),
                "TRACE_ID".to_string(),
            ],
            must_support_operations: vec![
                "create_span".to_string(),
                "record_metric".to_string(),
                "export_traces".to_string(),
                "status".to_string(),
            ],
            should_be_executable: true,
            should_have_shebang: true,
            max_lines: Some(400),
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
            
            // Check line count
            if let Some(max_lines) = req.max_lines {
                if metrics.line_count > max_lines {
                    warnings.push(format!("Script is long ({} lines > {} max)", 
                        metrics.line_count, max_lines));
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
        
        let executable = script_path.metadata()
            .map(|m| m.permissions().mode() & 0o111 != 0)
            .unwrap_or(false);
        
        Ok(ScriptMetrics {
            line_count,
            function_count,
            variable_count,
            comment_lines,
            executable,
        })
    }
    
    fn has_function(&self, content: &str, func_name: &str) -> bool {
        let pattern = format!(r"^\s*{}\s*\(\s*\)\s*{{", regex::escape(func_name));
        Regex::new(&pattern).unwrap().is_match(content)
    }
    
    fn has_variable(&self, content: &str, var_name: &str) -> bool {
        let pattern = format!(r"^\s*{}\s*=", regex::escape(var_name));
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

init_system() {
    echo "Initializing SwarmSH coordination system..."
    rm -f "$AGENT_REGISTRY" "$WORK_QUEUE"
    COORDINATION_EPOCH=0
    echo "System initialized"
}

benchmark() {
    echo "Running coordination benchmark..."
    for i in {1..10}; do
        register_agent "bench_agent_$i"
        claim_work "bench_work_$i" "bench_agent_$i"
    done
    echo "Benchmark completed: 10 agents, 10 work items"
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
    "init_system")
        init_system
        ;;
    "benchmark")
        benchmark
        ;;
    *)
        echo "Usage: $0 {register|claim_work|advance_epoch|health_check|init_system|benchmark}"
        exit 1
        ;;
esac
"#;
        fs::write(output_path, script_content)?;
        Self::make_executable(output_path)?;
        Ok(())
    }
    
    /// Generate mock agent orchestrator script
    pub fn generate_agent_orchestrator(output_path: &Path) -> Result<()> {
        let script_content = r#"#!/bin/bash
# SwarmSH v2 - Agent Swarm Orchestrator Script
# Generated mock for testing purposes

set -euo pipefail

SWARM_SIZE=0
COORDINATION_PATTERN="scrum_at_scale"
SWARM_STATE="${TMPDIR:-/tmp}/swarmsh_swarm_state"

init_swarm() {
    local size="$1"
    SWARM_SIZE="$size"
    echo "Initializing swarm with $size agents..."
    for i in $(seq 1 "$size"); do
        echo "agent_$(date +%s%N)_$i" >> "$SWARM_STATE"
    done
    echo "Swarm of $size agents initialized"
}

distribute_work() {
    local work_count="$1"
    echo "Distributing $work_count work items..."
    for i in $(seq 1 "$work_count"); do
        echo "work_$(date +%s%N)_$i:pending" >> "${SWARM_STATE}.work"
    done
    echo "$work_count work items distributed"
}

coordinate_agents() {
    local pattern="${1:-scrum_at_scale}"
    COORDINATION_PATTERN="$pattern"
    echo "Coordinating agents using $pattern pattern..."
    case "$pattern" in
        "scrum_at_scale")
            echo "Executing Scrum at Scale coordination"
            ;;
        "roberts_rules")
            echo "Executing Roberts Rules coordination"
            ;;
        "realtime")
            echo "Executing real-time coordination"
            ;;
        "atomic")
            echo "Executing atomic coordination"
            ;;
        *)
            echo "Unknown coordination pattern: $pattern"
            return 1
            ;;
    esac
    echo "Coordination completed using $pattern"
}

status() {
    local agent_count=$(wc -l < "$SWARM_STATE" 2>/dev/null || echo 0)
    local work_count=$(wc -l < "${SWARM_STATE}.work" 2>/dev/null || echo 0)
    echo "Swarm Status:"
    echo "  Agents: $agent_count"
    echo "  Work Items: $work_count"
    echo "  Pattern: $COORDINATION_PATTERN"
}

benchmark() {
    echo "Running orchestrator benchmark..."
    init_swarm 5
    distribute_work 10
    coordinate_agents "scrum_at_scale"
    echo "Benchmark completed"
}

case "${1:-help}" in
    "init_swarm")
        init_swarm "$2"
        ;;
    "distribute_work")
        distribute_work "$2"
        ;;
    "coordinate_agents")
        coordinate_agents "${2:-scrum_at_scale}"
        ;;
    "status")
        status
        ;;
    "benchmark")
        benchmark
        ;;
    *)
        echo "Usage: $0 {init_swarm|distribute_work|coordinate_agents|status|benchmark}"
        exit 1
        ;;
esac
"#;
        fs::write(output_path, script_content)?;
        Self::make_executable(output_path)?;
        Ok(())
    }
    
    /// Generate mock telemetry script
    pub fn generate_telemetry_script(output_path: &Path) -> Result<()> {
        let script_content = r#"#!/bin/bash
# SwarmSH v2 - Telemetry Spans Script
# Generated mock for testing purposes

set -euo pipefail

OTEL_EXPORTER="stdout"
TRACE_ID=""
SPAN_DATA="${TMPDIR:-/tmp}/swarmsh_spans"

generate_trace_id() {
    echo "$(date +%s%N | sha256sum | cut -c1-16)"
}

create_span() {
    local operation="$1"
    local trace_id="${2:-$(generate_trace_id)}"
    TRACE_ID="$trace_id"
    local span_data="{\"operation\":\"$operation\",\"trace_id\":\"$trace_id\",\"timestamp\":\"$(date -Iseconds)\"}"
    echo "$span_data" >> "$SPAN_DATA"
    echo "Span created: $operation (trace: $trace_id)"
}

record_metric() {
    local metric_name="$1"
    local metric_value="$2"
    local metric_data="{\"metric\":\"$metric_name\",\"value\":$metric_value,\"timestamp\":\"$(date -Iseconds)\"}"
    echo "$metric_data" >> "${SPAN_DATA}.metrics"
    echo "Metric recorded: $metric_name = $metric_value"
}

export_traces() {
    local format="${1:-stdout}"
    case "$format" in
        "stdout")
            echo "=== OTEL Traces ==="
            cat "$SPAN_DATA" 2>/dev/null || echo "No traces recorded"
            echo "=== OTEL Metrics ==="
            cat "${SPAN_DATA}.metrics" 2>/dev/null || echo "No metrics recorded"
            ;;
        "json")
            echo "Exporting traces as JSON..."
            jq -s '.' "$SPAN_DATA" 2>/dev/null || echo "[]"
            ;;
        *)
            echo "Unknown export format: $format"
            return 1
            ;;
    esac
}

status() {
    local span_count=$(wc -l < "$SPAN_DATA" 2>/dev/null || echo 0)
    local metric_count=$(wc -l < "${SPAN_DATA}.metrics" 2>/dev/null || echo 0)
    echo "Telemetry Status:"
    echo "  Spans: $span_count"
    echo "  Metrics: $metric_count"
    echo "  Exporter: $OTEL_EXPORTER"
}

benchmark() {
    echo "Running telemetry benchmark..."
    for i in {1..5}; do
        create_span "benchmark.operation.$i"
        record_metric "benchmark.counter" "$i"
    done
    echo "Telemetry benchmark completed"
}

case "${1:-help}" in
    "create_span")
        create_span "$2" "${3:-}"
        ;;
    "record_metric")
        record_metric "$2" "$3"
        ;;
    "export_traces")
        export_traces "${2:-stdout}"
        ;;
    "status")
        status
        ;;
    "benchmark")
        benchmark
        ;;
    *)
        echo "Usage: $0 {create_span|record_metric|export_traces|status|benchmark}"
        exit 1
        ;;
esac
"#;
        fs::write(output_path, script_content)?;
        Self::make_executable(output_path)?;
        Ok(())
    }
    
    /// Generate mock Ollama integration script
    pub fn generate_ollama_script(output_path: &Path) -> Result<()> {
        let script_content = r#"#!/bin/bash
# SwarmSH v2 - Ollama Integration Script
# Generated mock for testing purposes

set -euo pipefail

OLLAMA_URL="${OLLAMA_URL:-http://localhost:11434}"
DEFAULT_MODEL="${DEFAULT_MODEL:-llama3.2}"

health() {
    if command -v curl >/dev/null 2>&1; then
        if curl -s --max-time 5 "$OLLAMA_URL/api/tags" >/dev/null 2>&1; then
            echo "Ollama health: OK"
            return 0
        else
            echo "Ollama health: FAIL (not reachable)"
            return 1
        fi
    else
        echo "Ollama health: UNKNOWN (curl not available)"
        return 1
    fi
}

check_model() {
    local model="$1"
    echo "Checking model availability: $model"
    if health; then
        echo "Model $model is available (mock response)"
        return 0
    else
        echo "Cannot check model - Ollama not available"
        return 1
    fi
}

complete() {
    local prompt="$1"
    local model="${2:-$DEFAULT_MODEL}"
    echo "Generating completion for: $prompt"
    if health; then
        echo "Mock AI Response: Based on the prompt '$prompt', I recommend proceeding with the coordination strategy that maximizes agent utilization while maintaining zero-conflict guarantees."
        return 0
    else
        echo "Cannot generate completion - Ollama not available"
        return 1
    fi
}

make_decision() {
    local decision_type="$1"
    local context="$2"
    echo "Making AI decision: $decision_type (context: $context)"
    
    case "$decision_type" in
        "agent_assignment")
            echo "Decision: Assign work to agent with highest capacity and matching specializations"
            ;;
        "sprint_planning")
            echo "Decision: Prioritize high-value items with manageable complexity"
            ;;
        "conflict_resolution")
            echo "Decision: Apply nanosecond-precision epoch advancement for conflict resolution"
            ;;
        *)
            echo "Decision: Apply default coordination strategy for unknown decision type"
            ;;
    esac
}

benchmark() {
    echo "Running Ollama integration benchmark..."
    if health; then
        for i in {1..3}; do
            complete "Test prompt $i" "$DEFAULT_MODEL"
            make_decision "test_decision" "context_$i"
        done
        echo "Ollama benchmark completed"
    else
        echo "Ollama benchmark skipped - service not available"
    fi
}

case "${1:-help}" in
    "health")
        health
        ;;
    "check_model")
        check_model "$2"
        ;;
    "complete")
        complete "$2" "${3:-$DEFAULT_MODEL}"
        ;;
    "make_decision")
        make_decision "$2" "$3"
        ;;
    "benchmark")
        benchmark
        ;;
    *)
        echo "Usage: $0 {health|check_model|complete|make_decision|benchmark}"
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
        
        // Generate each script individually
        let script_path = output_dir.join("coordination_helper.sh");
        Self::generate_coordination_helper(&script_path)?;
        generated_paths.push(script_path);
        
        let script_path = output_dir.join("agent_swarm_orchestrator.sh");
        Self::generate_agent_orchestrator(&script_path)?;
        generated_paths.push(script_path);
        
        let script_path = output_dir.join("telemetry_spans.sh");
        Self::generate_telemetry_script(&script_path)?;
        generated_paths.push(script_path);
        
        let script_path = output_dir.join("ollama_integration.sh");
        Self::generate_ollama_script(&script_path)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_mock_script_generation() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let scripts = MockShellScriptGenerator::generate_all_mock_scripts(temp_dir.path())?;
        
        assert_eq!(scripts.len(), 4);
        
        for script_path in scripts {
            assert!(script_path.exists());
            let content = fs::read_to_string(&script_path)?;
            assert!(content.starts_with("#!/bin/bash"));
        }
        
        Ok(())
    }
    
    #[test]
    fn test_script_validation() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let coord_script = temp_dir.path().join("coordination_helper.sh");
        MockShellScriptGenerator::generate_coordination_helper(&coord_script)?;
        
        let validator = ShellScriptValidator::new();
        let result = validator.validate_script(&coord_script)?;
        
        assert!(result.is_valid, "Validation errors: {:?}", result.errors);
        assert!(result.metrics.function_count > 0);
        
        Ok(())
    }
}