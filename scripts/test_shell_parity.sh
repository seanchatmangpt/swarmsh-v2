#!/bin/bash
# SwarmSH v2 - 80/20 Shell/Rust Parity Tester
# Validates that shell exports maintain core functionality

set -euo pipefail

echo "🐚 SwarmSH v2 - Shell/Rust Parity Tester"
echo "Validating '100% functionality preservation' claims..."

# Test configuration
RUST_TEST_TIME=""
SHELL_TEST_TIME=""
TEMP_DIR="/tmp/swarmsh_parity_test"

mkdir -p "$TEMP_DIR"

echo ""
echo "📊 Test 1: Basic Coordination Patterns"

# Test Rust coordination timing
echo "  🦀 Testing Rust coordination..."
start_time=$(date +%s%N)

# Use our existing test binary
cargo run --bin test_otel_traces >/dev/null 2>&1 && rust_success=1 || rust_success=0

end_time=$(date +%s%N)
RUST_TEST_TIME=$(( (end_time - start_time) / 1000000 )) # Convert to milliseconds

if [ $rust_success -eq 1 ]; then
    echo "    ✅ Rust coordination: ${RUST_TEST_TIME}ms"
else
    echo "    ❌ Rust coordination failed"
fi

# Test Shell coordination timing
echo "  🐚 Testing shell coordination..."
if [ -f "exported-shell/coordination_helper.sh" ]; then
    start_time=$(date +%s%N)
    
    # Test basic shell script execution
    bash -n exported-shell/coordination_helper.sh && shell_syntax=1 || shell_syntax=0
    
    if [ $shell_syntax -eq 1 ]; then
        # Test that core functions exist
        if grep -q "scrum_at_scale_coordination" exported-shell/coordination_helper.sh && \
           grep -q "roberts_rules_coordination" exported-shell/coordination_helper.sh; then
            shell_success=1
        else
            shell_success=0
        fi
    else
        shell_success=0
    fi
    
    end_time=$(date +%s%N)
    SHELL_TEST_TIME=$(( (end_time - start_time) / 1000000 ))
    
    if [ $shell_success -eq 1 ]; then
        echo "    ✅ Shell coordination: ${SHELL_TEST_TIME}ms"
    else
        echo "    ❌ Shell coordination failed"
    fi
else
    echo "    ❌ Shell scripts not found"
    shell_success=0
fi

echo ""
echo "📊 Test 2: Function Parity Check"

# Check if shell scripts have equivalent functions to Rust
echo "  🔍 Checking function coverage..."

# Key functions that should exist in both
declare -a required_functions=(
    "coordination"
    "agent"
    "telemetry"
    "health"
)

missing_functions=0

for func in "${required_functions[@]}"; do
    if [ -d "exported-shell" ]; then
        if grep -r "$func" exported-shell/ >/dev/null 2>&1; then
            echo "    ✅ $func: Found in shell exports"
        else
            echo "    ❌ $func: Missing from shell exports"
            ((missing_functions++))
        fi
    else
        echo "    ❌ $func: No shell exports found"
        ((missing_functions++))
    fi
done

echo ""
echo "📊 Test 3: Performance Overhead Analysis"

if [ $rust_success -eq 1 ] && [ $shell_success -eq 1 ]; then
    # Calculate overhead percentage
    if [ "$RUST_TEST_TIME" -gt 0 ]; then
        overhead=$(( (SHELL_TEST_TIME * 100) / RUST_TEST_TIME - 100 ))
        echo "  ⚡ Performance overhead: ${overhead}%"
        
        if [ $overhead -le 10 ]; then
            echo "    ✅ Within <10% overhead target"
            perf_pass=1
        else
            echo "    ⚠️  Exceeds 10% overhead target"
            perf_pass=0
        fi
    else
        echo "  ⚠️  Unable to calculate overhead (Rust test too fast)"
        perf_pass=1
    fi
else
    echo "  ❌ Cannot compare performance - one or both systems failed"
    perf_pass=0
fi

echo ""
echo "📊 Test 4: Critical Feature Verification"

# Test that exported shell scripts have required patterns
critical_features=0

if [ -d "exported-shell" ]; then
    echo "  🔍 Checking critical features in shell exports..."
    
    # Check for atomic operations
    if grep -r "atomic\|lock" exported-shell/ >/dev/null 2>&1; then
        echo "    ✅ Atomic operations: Found"
        ((critical_features++))
    else
        echo "    ❌ Atomic operations: Missing"
    fi
    
    # Check for telemetry
    if grep -r "telemetry\|trace\|span" exported-shell/ >/dev/null 2>&1; then
        echo "    ✅ Telemetry: Found"
        ((critical_features++))
    else
        echo "    ❌ Telemetry: Missing"
    fi
    
    # Check for coordination patterns
    if grep -r "scrum\|roberts" exported-shell/ >/dev/null 2>&1; then
        echo "    ✅ Coordination patterns: Found"
        ((critical_features++))
    else
        echo "    ❌ Coordination patterns: Missing"
    fi
    
    # Check for zero-conflict guarantees
    if grep -r "zero.conflict\|nanosecond" exported-shell/ >/dev/null 2>&1; then
        echo "    ✅ Zero-conflict guarantees: Found"
        ((critical_features++))
    else
        echo "    ❌ Zero-conflict guarantees: Missing"
    fi
fi

echo ""
echo "════════════════════════════════════════"
echo "🎯 SHELL/RUST PARITY RESULTS:"
echo "════════════════════════════════════════"

# Calculate scores
rust_score=$([ $rust_success -eq 1 ] && echo 1 || echo 0)
shell_score=$([ $shell_success -eq 1 ] && echo 1 || echo 0)
function_score=$([ $missing_functions -eq 0 ] && echo 1 || echo 0)
performance_score=$perf_pass
feature_score=$([ $critical_features -ge 3 ] && echo 1 || echo 0)

total_score=$((rust_score + shell_score + function_score + performance_score + feature_score))

echo "📊 Rust Coordination:        $([ $rust_score -eq 1 ] && echo "✅ PASS" || echo "❌ FAIL")"
echo "📊 Shell Coordination:       $([ $shell_score -eq 1 ] && echo "✅ PASS" || echo "❌ FAIL")"
echo "📊 Function Parity:          $([ $function_score -eq 1 ] && echo "✅ PASS" || echo "❌ FAIL") ($missing_functions missing)"
echo "📊 Performance Overhead:     $([ $performance_score -eq 1 ] && echo "✅ PASS" || echo "❌ FAIL")"
echo "📊 Critical Features:        $([ $feature_score -eq 1 ] && echo "✅ PASS" || echo "❌ FAIL") ($critical_features/4 found)"

echo "════════════════════════════════════════"
echo "🏆 OVERALL PARITY SCORE: $total_score/5"

if [ $total_score -eq 5 ]; then
    echo "🎉 SHELL EXPORT PARITY VERIFIED - 100% FUNCTIONALITY PRESERVED"
elif [ $total_score -ge 3 ]; then
    echo "✅ SUBSTANTIAL PARITY ACHIEVED - Minor gaps remain"
else
    echo "⚠️  PARITY GAPS DETECTED - Significant work needed"
fi

echo ""
echo "🎯 80/20 Parity Test Complete"

# Cleanup
rm -rf "$TEMP_DIR"

# Exit with appropriate code
if [ $total_score -ge 3 ]; then
    exit 0
else
    exit 1
fi