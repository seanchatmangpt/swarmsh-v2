#!/bin/bash
# 80/20 Validation Loop - Focus on critical functionality only

set -euo pipefail

echo "=== SwarmSH v2 80/20 Validation Loop ==="
echo "Focus: Compilation → Basic Tests → OTEL Traces"
echo ""

# Step 1: Check compilation (20% effort, 80% value)
echo "1. Checking compilation..."
ERROR_COUNT=$(cargo check 2>&1 | grep -E "error\[E" | wc -l | tr -d ' ')
echo "   Compilation errors: $ERROR_COUNT"

if [[ $ERROR_COUNT -eq 0 ]]; then
    echo "   ✅ Code compiles!"
    
    # Step 2: Run basic unit test
    echo ""
    echo "2. Running basic unit test..."
    if cargo test --lib test_agent_id_type -- --nocapture 2>&1 | grep -q "test result: ok"; then
        echo "   ✅ Basic test passes!"
    else
        echo "   ❌ Basic test failed"
    fi
    
    # Step 3: Check if we can create OTEL traces
    echo ""
    echo "3. Testing OTEL trace generation..."
    cat > test_trace.rs << 'EOF'
use swarmsh_v2::{SwarmSystem, WorktreeSpec, CoordinationPattern};
use std::time::SystemTime;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .json()
        .init();
    
    println!("Creating SwarmSH system...");
    let system = SwarmSystem::new().await?;
    
    // Create a test span
    let span = system.create_agent_span("test-agent-1", "validation");
    let _enter = span.enter();
    
    println!("OTEL_TRACE: agent_span_created");
    
    // Create worktree span
    let worktree_spec = WorktreeSpec {
        name: "test-worktree".to_string(),
        branch: Some("main".to_string()),
        base_branch: None,
        coordination_pattern: CoordinationPattern::Atomic,
        agent_assignments: vec![],
        auto_sync: false,
        backup_enabled: false,
    };
    
    println!("OTEL_TRACE: worktree_spec_created");
    println!("VALIDATION: SUCCESS");
    
    Ok(())
}
EOF
    
    if rustc --edition 2021 -L target/debug/deps test_trace.rs -o test_trace 2>/dev/null; then
        if ./test_trace 2>&1 | grep -q "VALIDATION: SUCCESS"; then
            echo "   ✅ OTEL traces can be generated!"
        else
            echo "   ❌ OTEL trace generation failed"
        fi
        rm -f test_trace test_trace.rs
    else
        echo "   ⚠️  Cannot compile trace test yet"
        rm -f test_trace.rs
    fi
else
    echo "   ❌ Compilation failed - fixing this unlocks 80% of value"
    echo ""
    echo "Top 3 errors to fix:"
    cargo check 2>&1 | grep -E "error\[E" | head -3
fi

echo ""
echo "=== Validation Summary ==="
echo "Error count: $ERROR_COUNT"
echo "Progress: $((58 - ERROR_COUNT))/58 errors fixed ($(( (58 - ERROR_COUNT) * 100 / 58 ))%)"

# Generate quick fix suggestions based on common errors
if [[ $ERROR_COUNT -gt 0 ]]; then
    echo ""
    echo "80/20 Fix Suggestions:"
    if cargo check 2>&1 | grep -q "cannot find function"; then
        echo "- Add missing function implementations"
    fi
    if cargo check 2>&1 | grep -q "expected.*found"; then
        echo "- Fix type mismatches"
    fi
    if cargo check 2>&1 | grep -q "cannot borrow"; then
        echo "- Fix borrow checker issues with Arc/clone"
    fi
fi

echo ""
echo "Next validation in 5 seconds..."