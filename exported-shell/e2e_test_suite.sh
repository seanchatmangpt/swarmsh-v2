#!/bin/bash
# SwarmSH v2 E2E Test Suite - Complete System Validation
# Generated: $(date)
# Version: 2.0.0

set -euo pipefail

# Test configuration
TEST_DIR="/tmp/swarmsh_e2e_test_$(date +%s)"
EXPORT_DIR="$(cd "$(dirname "$0")" && pwd)"

# Test results
PASSED_TESTS=0
FAILED_TESTS=0
TEST_RESULTS=()

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test framework functions
run_test() {
    local test_name="$1"
    local test_function="$2"
    
    echo -e "\n${YELLOW}Running test: $test_name${NC}"
    
    if $test_function; then
        echo -e "${GREEN}✅ PASSED: $test_name${NC}"
        ((PASSED_TESTS++))
        TEST_RESULTS+=("PASS: $test_name")
    else
        echo -e "${RED}❌ FAILED: $test_name${NC}"
        ((FAILED_TESTS++))
        TEST_RESULTS+=("FAIL: $test_name")
    fi
}

# Setup test environment
setup_test_env() {
    echo "Setting up test environment..."
    mkdir -p "$TEST_DIR"/{work,agents,telemetry,health,analytics}
    cd "$TEST_DIR"
    
    # Copy scripts
    cp "$EXPORT_DIR"/*.sh .
    chmod +x *.sh
}

# Cleanup test environment
cleanup_test_env() {
    echo "Cleaning up test environment..."
    # Kill any remaining processes
    pkill -f "swarmsh_e2e_test" || true
    
    # Remove test directory
    rm -rf "$TEST_DIR"
}

# Test 1: Basic Coordination
test_basic_coordination() {
    echo "Testing basic coordination functionality..."
    
    # Start coordination helper
    local agent_id=$(./coordination_helper.sh register worker 0.8 scrum_at_scale)
    
    if [[ -z "$agent_id" ]]; then
        echo "Failed to register agent"
        return 1
    fi
    
    echo "Registered agent: $agent_id"
    
    # Test all coordination patterns
    for pattern in scrum_at_scale roberts_rules realtime atomic; do
        if ! ./coordination_helper.sh coordinate "$pattern" start "$agent_id"; then
            echo "Failed to start $pattern coordination"
            return 1
        fi
    done
    
    return 0
}

# Test 2: Multi-Agent Swarm
test_multi_agent_swarm() {
    echo "Testing multi-agent swarm orchestration..."
    
    # Create work items
    ./agent_swarm_orchestrator.sh create_work "$TEST_DIR/work" 10
    
    # Start swarm with 3 agents
    timeout 15s ./agent_swarm_orchestrator.sh start_swarm 3 scrum_at_scale "$TEST_DIR/work" &
    local swarm_pid=$!
    
    # Wait for swarm to process work
    sleep 10
    
    # Check work completion
    local completed=$(find "$TEST_DIR/work" -name "*.completed_*" | wc -l)
    
    # Kill swarm
    kill $swarm_pid 2>/dev/null || true
    wait $swarm_pid 2>/dev/null || true
    
    echo "Completed work items: $completed out of 10"
    
    if [[ $completed -ge 8 ]]; then
        return 0
    else
        echo "Insufficient work completion: $completed/10"
        return 1
    fi
}

# Test 3: Telemetry Collection
test_telemetry_collection() {
    echo "Testing OTEL-compatible telemetry..."
    
    # Start telemetry daemon
    ./swarmsh_telemetry.sh daemon 2 &
    local telemetry_pid=$!
    
    # Generate telemetry data
    local trace_id=$(./swarmsh_telemetry.sh start_span "test_operation")
    ./swarmsh_telemetry.sh event "$trace_id" "test_event" '{"key": "value"}'
    ./swarmsh_telemetry.sh end_span "$trace_id" "ok"
    
    # Record metrics
    ./swarmsh_telemetry.sh metric "test_metric" 42 "gauge" '{"test": "true"}'
    
    # Agent telemetry
    local agent_span=$(./swarmsh_telemetry.sh agent_span "register" "test_agent_123")
    ./swarmsh_telemetry.sh end_span "$agent_span" "ok"
    
    # Work telemetry
    local work_span=$(./swarmsh_telemetry.sh work_span "claim" "work_123" "test_agent_123")
    ./swarmsh_telemetry.sh end_span "$work_span" "ok"
    
    # Wait for flush
    sleep 3
    
    # Export telemetry
    ./swarmsh_telemetry.sh export json "$TEST_DIR/telemetry_export.json"
    
    # Kill daemon
    kill $telemetry_pid 2>/dev/null || true
    
    # Verify export
    if [[ -f "$TEST_DIR/telemetry_export.json" ]]; then
        local span_count=$(jq '.spans | length' "$TEST_DIR/telemetry_export.json")
        local metric_count=$(jq '.metrics | length' "$TEST_DIR/telemetry_export.json")
        
        echo "Exported spans: $span_count, metrics: $metric_count"
        
        if [[ $span_count -ge 3 ]] && [[ $metric_count -ge 1 ]]; then
            return 0
        fi
    fi
    
    echo "Telemetry export validation failed"
    return 1
}

# Test 4: Health Monitoring
test_health_monitoring() {
    echo "Testing health monitoring with bottleneck detection..."
    
    # Start health monitor
    ./health_monitor.sh monitor 2 &
    local health_pid=$!
    
    sleep 2
    
    # Check component health
    for component in coordinator agents work_queue telemetry system; do
        if ! ./health_monitor.sh check "$component"; then
            echo "Health check failed for: $component"
        fi
    done
    
    # Simulate bottleneck
    ./health_monitor.sh bottleneck "work_queue" "backlog" "100 items pending"
    
    # Check remediation
    AUTO_REMEDIATION=true ./health_monitor.sh remediate "work_queue" "backlog"
    
    # Generate report
    ./health_monitor.sh report
    
    # Check status
    ./health_monitor.sh status
    
    # Kill monitor
    kill $health_pid 2>/dev/null || true
    
    # Verify health data
    if [[ -d "$HEALTH_DATA_DIR" ]]; then
        local component_count=$(find "$HEALTH_DATA_DIR/components" -name "*.json" 2>/dev/null | wc -l)
        local bottleneck_count=$(find "$HEALTH_DATA_DIR/bottlenecks" -name "*.json" 2>/dev/null | wc -l)
        
        echo "Health components: $component_count, bottlenecks: $bottleneck_count"
        
        if [[ $component_count -ge 5 ]] && [[ $bottleneck_count -ge 1 ]]; then
            return 0
        fi
    fi
    
    return 1
}

# Test 5: DLSS Analytics
test_dlss_analytics() {
    echo "Testing DLSS analytics engine..."
    
    # Generate test data
    mkdir -p "$TELEMETRY_DIR"/spans
    for i in {1..10}; do
        cat > "$TELEMETRY_DIR/spans/span_$i.json" <<EOF
{
    "span_id": "span_$i",
    "name": "test_operation_$i",
    "start_time": $(date +%s%N),
    "end_time": $(($(date +%s%N) + 1000000)),
    "status": "ok"
}
EOF
    done
    
    # Create value stream map
    ./analytics_dlss.sh vsm "test_process"
    
    # Detect waste
    ./analytics_dlss.sh waste "all"
    
    # Pareto analysis
    ./analytics_dlss.sh pareto "bottlenecks"
    
    # Generate optimization report
    ./analytics_dlss.sh optimize
    
    # Verify analytics output
    if [[ -d "$ANALYTICS_DIR" ]]; then
        local vsm_count=$(find "$ANALYTICS_DIR/value_stream" -name "*.json" 2>/dev/null | wc -l)
        local waste_count=$(find "$ANALYTICS_DIR/waste" -name "*.json" 2>/dev/null | wc -l)
        local report_count=$(find "$ANALYTICS_DIR/reports" -name "*.json" 2>/dev/null | wc -l)
        
        echo "VSM: $vsm_count, Waste analyses: $waste_count, Reports: $report_count"
        
        if [[ $vsm_count -ge 1 ]] && [[ $waste_count -ge 1 ]] && [[ $report_count -ge 1 ]]; then
            return 0
        fi
    fi
    
    return 1
}

# Test 6: Zero-Conflict Guarantees
test_zero_conflict() {
    echo "Testing zero-conflict coordination guarantees..."
    
    # Create shared work directory
    local work_dir="$TEST_DIR/conflict_test"
    mkdir -p "$work_dir"
    
    # Create work items
    for i in {1..20}; do
        echo '{"id": "'$i'", "data": "test"}' > "$work_dir/work_$i.todo"
    done
    
    # Start multiple agents claiming work concurrently
    local claim_results=()
    for agent_num in {1..5}; do
        (
            local agent_id="agent_$agent_num"
            local claimed=0
            
            for attempt in {1..10}; do
                if ./coordination_helper.sh claim_work "$work_dir" "$agent_id" >/dev/null 2>&1; then
                    ((claimed++))
                fi
            done
            
            echo "$agent_id:$claimed"
        ) &
    done
    
    # Wait for all agents
    wait
    
    # Verify no conflicts
    local total_claimed=$(find "$work_dir" -name "*.claimed_*" | wc -l)
    local double_claimed=$(find "$work_dir" -name "*.claimed_*" | \
                          sed 's/.*work_\([0-9]*\)\.claimed.*/\1/' | \
                          sort | uniq -d | wc -l)
    
    echo "Total claimed: $total_claimed, Double claimed: $double_claimed"
    
    if [[ $double_claimed -eq 0 ]] && [[ $total_claimed -le 20 ]]; then
        echo "Zero-conflict guarantee verified!"
        return 0
    else
        echo "Conflict detected! Double claims: $double_claimed"
        return 1
    fi
}

# Test 7: Nanosecond Precision
test_nanosecond_precision() {
    echo "Testing nanosecond precision timestamps..."
    
    # Generate multiple IDs
    local ids=()
    for i in {1..10}; do
        local id=$(./coordination_helper.sh register | tail -1)
        ids+=("$id")
        sleep 0.001  # 1ms delay
    done
    
    # Verify all IDs are unique
    local unique_count=$(printf '%s\n' "${ids[@]}" | sort -u | wc -l)
    
    echo "Generated ${#ids[@]} IDs, $unique_count unique"
    
    if [[ $unique_count -eq ${#ids[@]} ]]; then
        # Verify nanosecond format
        for id in "${ids[@]}"; do
            if [[ ! "$id" =~ agent_[0-9]{19} ]]; then
                echo "Invalid nanosecond ID format: $id"
                return 1
            fi
        done
        
        echo "All IDs have nanosecond precision!"
        return 0
    else
        echo "Duplicate IDs detected"
        return 1
    fi
}

# Test 8: Integration Test
test_full_integration() {
    echo "Running full system integration test..."
    
    # Start all components
    echo "Starting telemetry daemon..."
    TELEMETRY_DIR="$TEST_DIR/telemetry" ./swarmsh_telemetry.sh daemon 2 &
    local telemetry_pid=$!
    
    echo "Starting health monitor..."
    HEALTH_DATA_DIR="$TEST_DIR/health" ./health_monitor.sh monitor 5 &
    local health_pid=$!
    
    echo "Starting analytics daemon..."
    ANALYTICS_DIR="$TEST_DIR/analytics" ./analytics_dlss.sh daemon 10 &
    local analytics_pid=$!
    
    sleep 2
    
    # Run integrated workflow
    echo "Starting integrated swarm..."
    
    # Export environment for scripts
    export TELEMETRY_SCRIPT="./swarmsh_telemetry.sh"
    export TELEMETRY_DIR="$TEST_DIR/telemetry"
    export HEALTH_DATA_DIR="$TEST_DIR/health"
    export ANALYTICS_DIR="$TEST_DIR/analytics"
    export WORK_DIR="$TEST_DIR/work"
    
    # Create work
    ./agent_swarm_orchestrator.sh create_work "$WORK_DIR" 15
    
    # Start swarm
    timeout 20s ./agent_swarm_orchestrator.sh start_swarm 4 scrum_at_scale "$WORK_DIR"
    
    # Wait for processing
    sleep 5
    
    # Collect results
    local completed_work=$(find "$WORK_DIR" -name "*.completed_*" 2>/dev/null | wc -l)
    local telemetry_spans=$(find "$TELEMETRY_DIR/spans" -name "*.json" 2>/dev/null | wc -l)
    local health_checks=$(find "$HEALTH_DATA_DIR/components" -name "*.json" 2>/dev/null | wc -l)
    local analytics_reports=$(find "$ANALYTICS_DIR/reports" -name "*.json" 2>/dev/null | wc -l)
    
    # Kill daemons
    kill $telemetry_pid $health_pid $analytics_pid 2>/dev/null || true
    
    echo "Integration results:"
    echo "  Completed work: $completed_work/15"
    echo "  Telemetry spans: $telemetry_spans"
    echo "  Health checks: $health_checks"
    echo "  Analytics reports: $analytics_reports"
    
    if [[ $completed_work -ge 10 ]] && [[ $telemetry_spans -gt 0 ]] && \
       [[ $health_checks -gt 0 ]] && [[ $analytics_reports -gt 0 ]]; then
        return 0
    else
        echo "Integration test failed - insufficient activity"
        return 1
    fi
}

# Main test runner
main() {
    echo "🧪 SwarmSH v2 E2E Test Suite"
    echo "============================"
    echo "Testing exported shell implementation"
    echo ""
    
    # Setup
    setup_test_env
    
    # Run all tests
    run_test "Basic Coordination" test_basic_coordination
    run_test "Multi-Agent Swarm" test_multi_agent_swarm
    run_test "Telemetry Collection" test_telemetry_collection
    run_test "Health Monitoring" test_health_monitoring
    run_test "DLSS Analytics" test_dlss_analytics
    run_test "Zero-Conflict Guarantees" test_zero_conflict
    run_test "Nanosecond Precision" test_nanosecond_precision
    run_test "Full Integration" test_full_integration
    
    # Summary
    echo ""
    echo "======================================="
    echo "Test Summary:"
    echo "  Passed: $PASSED_TESTS"
    echo "  Failed: $FAILED_TESTS"
    echo ""
    
    for result in "${TEST_RESULTS[@]}"; do
        echo "  $result"
    done
    
    # Cleanup
    cleanup_test_env
    
    # Exit code
    if [[ $FAILED_TESTS -eq 0 ]]; then
        echo ""
        echo -e "${GREEN}✅ All tests passed!${NC}"
        exit 0
    else
        echo ""
        echo -e "${RED}❌ Some tests failed${NC}"
        exit 1
    fi
}

# Run tests
main "$@"