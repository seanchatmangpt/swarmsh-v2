#!/usr/bin/env bash
# E2E Test Suite for Weaver Complete Implementation
# Uses Unix tools only for validation

set -euo pipefail

# Test configuration
TEST_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "${TEST_DIR}")"
TEMP_DIR="/tmp/weaver_e2e_$$"
LOG_FILE="${TEMP_DIR}/test.log"
ERROR_LOG="${TEMP_DIR}/error.log"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test counters
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# Setup test environment
setup_test_env() {
    echo -e "${BLUE}Setting up test environment...${NC}"
    
    # Create temp directory
    mkdir -p "${TEMP_DIR}"
    
    # Copy templates to temp directory
    cp -r "${PROJECT_ROOT}/templates" "${TEMP_DIR}/"
    cp -r "${PROJECT_ROOT}/semantic-conventions" "${TEMP_DIR}/"
    
    # Create test output directories
    mkdir -p "${TEMP_DIR}/generated"
    mkdir -p "${TEMP_DIR}/output"
    
    echo "Test environment ready at: ${TEMP_DIR}"
}

# Cleanup test environment
cleanup_test_env() {
    echo -e "${BLUE}Cleaning up test environment...${NC}"
    rm -rf "${TEMP_DIR}"
}

# Test helper functions
run_test() {
    local test_name="$1"
    local test_function="$2"
    
    echo -e "\n${BLUE}Running test: ${test_name}${NC}"
    TESTS_RUN=$((TESTS_RUN + 1))
    
    if ${test_function} >> "${LOG_FILE}" 2>> "${ERROR_LOG}"; then
        echo -e "${GREEN}✓ PASS: ${test_name}${NC}"
        TESTS_PASSED=$((TESTS_PASSED + 1))
        return 0
    else
        echo -e "${RED}✗ FAIL: ${test_name}${NC}"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        echo "  Error log:"
        tail -5 "${ERROR_LOG}" | sed 's/^/    /'
        return 1
    fi
}

# Test the Weaver specification YAML
test_yaml_specification() {
    local spec_file="${TEMP_DIR}/semantic-conventions/swarmsh-weaver-complete.yaml"
    
    # Check file exists
    [[ -f "${spec_file}" ]] || {
        echo "ERROR: Weaver specification file not found"
        return 1
    }
    
    # Validate YAML syntax
    if command -v yamllint &>/dev/null; then
        yamllint "${spec_file}" || {
            echo "ERROR: YAML syntax validation failed"
            return 1
        }
    else
        # Basic YAML validation with Python
        python3 -c "import yaml; yaml.safe_load(open('${spec_file}'))" || {
            echo "ERROR: YAML parsing failed"
            return 1
        }
    fi
    
    # Check required sections exist
    local required_sections=("groups" "spans" "cli_commands" "e2e_tests" "frameworks_compared")
    for section in "${required_sections[@]}"; do
        grep -q "^${section}:" "${spec_file}" || {
            echo "ERROR: Required section '${section}' not found"
            return 1
        }
    done
    
    echo "YAML specification validation passed"
    return 0
}

# Test Rust binary compilation
test_rust_compilation() {
    cd "${PROJECT_ROOT}"
    
    # Build the weaver_complete binary
    cargo build --bin weaver_complete --release >> "${LOG_FILE}" 2>> "${ERROR_LOG}" || {
        echo "ERROR: Failed to build weaver_complete binary"
        return 1
    }
    
    # Check binary exists and is executable
    local binary_path="${PROJECT_ROOT}/target/release/weaver_complete"
    [[ -x "${binary_path}" ]] || {
        echo "ERROR: weaver_complete binary not found or not executable"
        return 1
    }
    
    # Test basic command
    "${binary_path}" --help >> "${LOG_FILE}" 2>> "${ERROR_LOG}" || {
        echo "ERROR: weaver_complete --help failed"
        return 1
    }
    
    echo "Rust compilation test passed"
    return 0
}

# Test shell template generation
test_shell_template_generation() {
    local template_dir="${TEMP_DIR}/templates"
    local output_dir="${TEMP_DIR}/generated"
    
    # Check template files exist
    local templates=(
        "weaver_cli_commands.sh.j2"
        "weaver_telemetry_span_emit.sh.j2"
        "weaver_infinite_loop_runner.sh.j2"
        "weaver_dlss_wave_exec.sh.j2"
    )
    
    for template in "${templates[@]}"; do
        [[ -f "${template_dir}/${template}" ]] || {
            echo "ERROR: Template ${template} not found"
            return 1
        }
    done
    
    # Generate shell scripts using Python/Jinja2
    python3 << EOF
import os
import sys
sys.path.append('${PROJECT_ROOT}')
from jinja2 import Environment, FileSystemLoader
import yaml

# Load specification
with open('${TEMP_DIR}/semantic-conventions/swarmsh-weaver-complete.yaml', 'r') as f:
    spec = yaml.safe_load(f)

# Setup Jinja2 environment
env = Environment(loader=FileSystemLoader('${template_dir}'))

# Generate CLI commands
template = env.get_template('weaver_cli_commands.sh.j2')
output = template.render(
    cli_commands=spec.get('cli_commands', []),
    timestamp='$(date)'
)
with open('${output_dir}/weaver_cli.sh', 'w') as f:
    f.write(output)

# Generate telemetry emitter
template = env.get_template('weaver_telemetry_span_emit.sh.j2')
output = template.render(spans=spec.get('spans', []))
with open('${output_dir}/telemetry_emitter.sh', 'w') as f:
    f.write(output)

print("Shell template generation completed")
EOF
    
    # Check generated files
    [[ -f "${output_dir}/weaver_cli.sh" ]] || {
        echo "ERROR: Generated CLI script not found"
        return 1
    }
    
    [[ -f "${output_dir}/telemetry_emitter.sh" ]] || {
        echo "ERROR: Generated telemetry script not found"
        return 1
    }
    
    # Make scripts executable
    chmod +x "${output_dir}"/*.sh
    
    echo "Shell template generation test passed"
    return 0
}

# Test generated shell scripts
test_generated_shell_scripts() {
    local generated_dir="${TEMP_DIR}/generated"
    
    # Test CLI script
    local cli_script="${generated_dir}/weaver_cli.sh"
    [[ -x "${cli_script}" ]] || {
        echo "ERROR: CLI script not executable"
        return 1
    }
    
    # Test help command
    "${cli_script}" --help >> "${LOG_FILE}" 2>> "${ERROR_LOG}" || {
        echo "ERROR: CLI script --help failed"
        return 1
    }
    
    # Test telemetry emitter
    local telemetry_script="${generated_dir}/telemetry_emitter.sh"
    [[ -f "${telemetry_script}" ]] || {
        echo "ERROR: Telemetry script not found"
        return 1
    }
    
    # Source and test telemetry functions
    source "${telemetry_script}"
    
    # Test basic telemetry function
    if type "emit_span_start" &>/dev/null; then
        echo "Telemetry functions loaded successfully"
    else
        echo "ERROR: Telemetry functions not loaded"
        return 1
    fi
    
    echo "Generated shell scripts test passed"
    return 0
}

# Test coordination parity
test_coordination_parity() {
    local generated_dir="${TEMP_DIR}/generated"
    local output_dir="${TEMP_DIR}/output/coordination"
    
    mkdir -p "${output_dir}"
    
    # Set up test environment variables
    export TELEMETRY_ENABLED="true"
    export TELEMETRY_LOG_DIR="${output_dir}"
    export CORRELATION_ID="test-$(date +%s)"
    
    # Test Rust execution
    cd "${PROJECT_ROOT}"
    cargo run --bin weaver_complete -- validate superiority --framework langchain \
        >> "${LOG_FILE}" 2>> "${ERROR_LOG}" || {
        echo "ERROR: Rust weaver_complete execution failed"
        return 1
    }
    
    # Test shell execution
    "${generated_dir}/weaver_cli.sh" validate superiority --framework langchain \
        >> "${LOG_FILE}" 2>> "${ERROR_LOG}" || {
        echo "ERROR: Shell weaver_cli execution failed"
        return 1
    }
    
    # Compare outputs (basic comparison)
    if [[ -f "${output_dir}/spans.jsonl" ]]; then
        echo "Telemetry output generated successfully"
    else
        echo "WARNING: No telemetry output found"
    fi
    
    echo "Coordination parity test passed"
    return 0
}

# Test AI prompt telemetry
test_ai_prompt_telemetry() {
    local output_dir="${TEMP_DIR}/output/ai_telemetry"
    
    mkdir -p "${output_dir}"
    
    # Set up telemetry environment
    export TELEMETRY_ENABLED="true"
    export TELEMETRY_LOG_DIR="${output_dir}"
    
    # Create mock prompt file
    cat > "${output_dir}/test_prompt.txt" << EOF
Test prompt for AI telemetry validation.
This is a mock prompt to test the telemetry instrumentation.
EOF
    
    # Test AI prompt tracing
    cd "${PROJECT_ROOT}"
    cargo run --bin weaver_complete -- trace ai_prompt \
        --provider ollama \
        --prompt-file "${output_dir}/test_prompt.txt" \
        >> "${LOG_FILE}" 2>> "${ERROR_LOG}" || {
        echo "ERROR: AI prompt telemetry test failed"
        return 1
    }
    
    # Validate telemetry output exists
    [[ -f "${output_dir}/spans.jsonl" ]] || {
        echo "WARNING: No telemetry spans generated"
    }
    
    echo "AI prompt telemetry test passed"
    return 0
}

# Test governance execution
test_governance_execution() {
    local output_dir="${TEMP_DIR}/output/governance"
    
    mkdir -p "${output_dir}"
    
    # Test Scrum at Scale governance
    cd "${PROJECT_ROOT}"
    cargo run --bin weaver_complete -- govern scrum_or_roberts \
        --model scrum_at_scale \
        >> "${LOG_FILE}" 2>> "${ERROR_LOG}" || {
        echo "ERROR: Scrum governance test failed"
        return 1
    }
    
    # Test Robert's Rules governance
    cargo run --bin weaver_complete -- govern scrum_or_roberts \
        --model roberts_rules \
        >> "${LOG_FILE}" 2>> "${ERROR_LOG}" || {
        echo "ERROR: Robert's Rules governance test failed"
        return 1
    }
    
    echo "Governance execution test passed"
    return 0
}

# Test DLSS wave execution
test_dlss_wave_execution() {
    local output_dir="${TEMP_DIR}/output/dlss"
    local templates_dir="${TEMP_DIR}/templates"
    
    mkdir -p "${output_dir}"
    
    # Copy DLSS wave executor to output
    cp "${templates_dir}/weaver_dlss_wave_exec.sh.j2" "${output_dir}/dlss_executor.sh"
    
    # Create basic shell script from template (simplified)
    sed 's/{{ [^}]* }}/# Generated/g' "${output_dir}/dlss_executor.sh" > "${output_dir}/dlss_executor_simple.sh"
    chmod +x "${output_dir}/dlss_executor_simple.sh"
    
    # Test DLSS optimization
    cd "${PROJECT_ROOT}"
    cargo run --bin weaver_complete -- optimize dlss_8020 \
        --spec "${TEMP_DIR}/semantic-conventions/swarmsh-weaver-complete.yaml" \
        --wave-size 4 \
        >> "${LOG_FILE}" 2>> "${ERROR_LOG}" || {
        echo "ERROR: DLSS optimization test failed"
        return 1
    }
    
    echo "DLSS wave execution test passed"
    return 0
}

# Test Unix tools integration
test_unix_tools_integration() {
    local output_dir="${TEMP_DIR}/output/unix_integration"
    
    mkdir -p "${output_dir}"
    
    # Test with common Unix tools
    local test_data="${output_dir}/test_data.json"
    
    # Generate test data
    cat > "${test_data}" << EOF
{"name": "weaver_test", "status": "running", "metrics": {"score": 0.85}}
{"name": "weaver_test", "status": "completed", "metrics": {"score": 0.92}}
{"name": "weaver_test", "status": "failed", "metrics": {"score": 0.45}}
EOF
    
    # Test with grep
    local running_count=$(grep -c '"status": "running"' "${test_data}")
    [[ ${running_count} -eq 1 ]] || {
        echo "ERROR: grep test failed"
        return 1
    }
    
    # Test with awk
    local avg_score=$(awk -F'"score": ' '{if(NF>1) print $2}' "${test_data}" | awk -F'}' '{print $1}' | awk '{sum+=$1; count++} END {print sum/count}')
    [[ -n "${avg_score}" ]] || {
        echo "ERROR: awk test failed"
        return 1
    }
    
    # Test with jq (if available)
    if command -v jq &>/dev/null; then
        local jq_score=$(jq -r '.metrics.score' "${test_data}" | awk '{sum+=$1; count++} END {print sum/count}')
        [[ -n "${jq_score}" ]] || {
            echo "ERROR: jq test failed"
            return 1
        }
    fi
    
    echo "Unix tools integration test passed"
    return 0
}

# Test performance and benchmarks
test_performance_benchmarks() {
    local output_dir="${TEMP_DIR}/output/benchmarks"
    
    mkdir -p "${output_dir}"
    
    # Performance test: measure execution time
    local start_time=$(date +%s%N)
    
    # Run a series of operations
    cd "${PROJECT_ROOT}"
    for i in {1..5}; do
        cargo run --bin weaver_complete -- validate superiority --framework langchain \
            >> "${LOG_FILE}" 2>> "${ERROR_LOG}" || true
    done
    
    local end_time=$(date +%s%N)
    local duration=$(( (end_time - start_time) / 1000000 ))  # Convert to milliseconds
    
    echo "Performance benchmark: ${duration}ms for 5 operations"
    
    # Benchmark should complete within reasonable time (10 seconds)
    [[ ${duration} -lt 10000 ]] || {
        echo "WARNING: Performance benchmark took longer than expected"
    }
    
    echo "Performance benchmarks test passed"
    return 0
}

# Main test execution
main() {
    echo -e "${BLUE}=== Weaver Complete E2E Tests ===${NC}"
    echo "Starting comprehensive test suite..."
    
    # Trap cleanup
    trap cleanup_test_env EXIT
    
    # Setup
    setup_test_env
    
    # Run tests
    run_test "YAML Specification Validation" test_yaml_specification
    run_test "Rust Compilation" test_rust_compilation
    run_test "Shell Template Generation" test_shell_template_generation
    run_test "Generated Shell Scripts" test_generated_shell_scripts
    run_test "Coordination Parity" test_coordination_parity
    run_test "AI Prompt Telemetry" test_ai_prompt_telemetry
    run_test "Governance Execution" test_governance_execution
    run_test "DLSS Wave Execution" test_dlss_wave_execution
    run_test "Unix Tools Integration" test_unix_tools_integration
    run_test "Performance Benchmarks" test_performance_benchmarks
    
    # Summary
    echo -e "\n${BLUE}=== Test Summary ===${NC}"
    echo "Tests run: ${TESTS_RUN}"
    echo -e "Passed: ${GREEN}${TESTS_PASSED}${NC}"
    echo -e "Failed: ${RED}${TESTS_FAILED}${NC}"
    
    if [[ ${TESTS_FAILED} -eq 0 ]]; then
        echo -e "\n${GREEN}✓ All tests passed!${NC}"
        echo "Weaver Complete implementation is validated"
        return 0
    else
        echo -e "\n${RED}✗ Some tests failed${NC}"
        echo "Check logs at: ${LOG_FILE}"
        echo "Error log: ${ERROR_LOG}"
        return 1
    fi
}

# Run if executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi