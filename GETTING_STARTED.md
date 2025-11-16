# Getting Started with SwarmSH v2

**Step-by-step guide to install, configure, and run your first distributed coordination system.**

This is the fastest way to get SwarmSH v2 working on your system. For detailed reference documentation, see [README.md](./README.md).

---

## Prerequisites

Before starting, verify you have:

| Tool | Minimum Version | Purpose | Check Command |
|------|-----------------|---------|----------------|
| Rust | 1.70+ | Compilation | `rustc --version` |
| Cargo | 1.70+ | Package management | `cargo --version` |
| Bash | 4.0+ | Shell scripts | `bash --version` |
| Git | 2.0+ | Version control | `git --version` |

**Total setup time**: 5-10 minutes

---

## Step 1: Clone the Repository

```bash
# Clone the project
git clone https://github.com/seanchatmangpt/swarmsh-v2.git
cd swarmsh-v2

# Verify you're on the main branch (or your preferred branch)
git status
```

---

## Step 2: Install Dependencies

### Option A: Automatic Installation (Recommended)

```bash
# Build the core library (this is the stable part)
cargo build --lib

# Run verification tests
cargo test --lib coordination::tests

# Expected output:
# test coordination::tests::test_basic_agent_registration ... ok
# test coordination::tests::test_work_queue_basic_operations ... ok
```

### Option B: Build Everything (Some binaries may fail)

```bash
# This will attempt to build all binaries
# WARNING: ~60% of binaries have compilation issues
cargo build

# If you see compilation errors, don't worry
# - The core library compiled successfully
# - Binary issues don't affect core functionality
# Use the library directly instead of binaries
```

---

## Step 3: Verify Installation

Create a simple test file to verify everything works:

```bash
# Create test file
cat > test_coordination.rs << 'EOF'
use swarmsh_v2::coordination::{CoordinationEngine, AgentSpec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize coordination engine
    let engine = CoordinationEngine::new("./test_data").await?;

    // Create a test agent
    let agent = AgentSpec {
        id: "test_agent_001".to_string(),
        role: "worker".to_string(),
        capacity: 1.0,
        specializations: vec!["testing".to_string()],
        work_capacity: Some(10),
    };

    // Register the agent
    engine.register_agent(agent).await?;

    // List agents to verify registration
    let agents = engine.list_agents().await?;
    println!("✓ Successfully registered {} agents", agents.len());

    Ok(())
}
EOF

# Compile and run the test
rustc --edition 2021 test_coordination.rs -L target/debug/deps \
    --extern swarmsh_v2=target/debug/libswarmsh_v2.rlib \
    -o test_coordination 2>/dev/null || \
    cargo run --example basic_coordination
```

---

## Step 4: Configure Your Environment

### Basic Configuration

```bash
# Create a data directory
mkdir -p ./swarmsh_data

# Set environment variables
export SWARMSH_DATA_DIR=./swarmsh_data
export RUST_LOG=info

# Verify configuration
echo "Data dir: $SWARMSH_DATA_DIR"
echo "Logging: $RUST_LOG"
```

### Optional: OTEL Configuration

For production deployments with observability:

```bash
# Option 1: Use local Jaeger (for development)
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317

# Option 2: Use cloud OTEL endpoint
export OTEL_EXPORTER_OTLP_ENDPOINT=https://your-otel-endpoint

# Option 3: Use stdout exporter (for debugging)
export OTEL_EXPORTER=stdout
```

---

## Step 5: Run Your First Coordination Example

### Example 1: Basic Agent Registration

Create and run this example:

```bash
# Create example file
mkdir -p examples
cat > examples/basic_coordination.rs << 'EOF'
use swarmsh_v2::coordination::{CoordinationEngine, AgentSpec};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 SwarmSH v2 - Basic Coordination Example\n");

    // Step 1: Initialize the coordination engine
    println!("Step 1: Initializing coordination engine...");
    let engine = CoordinationEngine::new("./swarmsh_data").await?;
    println!("✓ Coordination engine ready\n");

    // Step 2: Register multiple agents
    println!("Step 2: Registering agents...");
    for i in 1..=3 {
        let agent = AgentSpec {
            id: format!("agent_{:03}", i),
            role: "worker".to_string(),
            capacity: 1.0,
            specializations: vec!["data_processing".to_string()],
            work_capacity: Some(5 + i),
        };

        engine.register_agent(agent).await?;
        println!("  ✓ Registered agent_{:03}", i);
    }
    println!();

    // Step 3: List all registered agents
    println!("Step 3: Listing registered agents...");
    let agents = engine.list_agents().await?;
    for agent in &agents {
        println!(
            "  - {}: {} (capacity: {}, work_capacity: {:?})",
            agent.id, agent.role, agent.capacity, agent.work_capacity
        );
    }
    println!("\n✓ Successfully registered {} agents\n", agents.len());

    // Step 4: Check system health
    println!("Step 4: Checking system health...");
    let health = engine.check_health().await?;
    println!("  System healthy: {}", health.is_healthy);
    if !health.is_healthy {
        println!("  Bottlenecks detected:");
        for bottleneck in health.detected_bottlenecks {
            println!("    - {}: {}", bottleneck.name, bottleneck.description);
        }
    }
    println!();

    println!("✅ Example completed successfully!");
    Ok(())
}
EOF

# Run the example
cargo run --example basic_coordination
```

Expected output:
```
🚀 SwarmSH v2 - Basic Coordination Example

Step 1: Initializing coordination engine...
✓ Coordination engine ready

Step 2: Registering agents...
  ✓ Registered agent_001
  ✓ Registered agent_002
  ✓ Registered agent_003

Step 3: Listing registered agents...
  - agent_001: worker (capacity: 1.0, work_capacity: Some(6))
  - agent_002: worker (capacity: 1.0, work_capacity: Some(7))
  - agent_003: worker (capacity: 1.0, work_capacity: Some(8))

✓ Successfully registered 3 agents

Step 4: Checking system health...
  System healthy: true

✅ Example completed successfully!
```

### Example 2: Work Distribution

```bash
# Create work distribution example
cat > examples/work_distribution.rs << 'EOF'
use swarmsh_v2::coordination::{CoordinationEngine, AgentSpec, WorkSpec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📋 SwarmSH v2 - Work Distribution Example\n");

    let engine = CoordinationEngine::new("./swarmsh_data").await?;

    // Register agents
    println!("Registering agents...");
    for i in 1..=2 {
        let agent = AgentSpec {
            id: format!("worker_{}", i),
            role: "processor".to_string(),
            capacity: 0.5,  // 50% capacity
            specializations: vec!["data_processing".to_string()],
            work_capacity: Some(10),
        };
        engine.register_agent(agent).await?;
    }
    println!("✓ Registered 2 agents\n");

    // Create work items
    println!("Creating work items...");
    let mut work_ids = vec![];
    for i in 1..=5 {
        let work = WorkSpec {
            id: format!("job_{:03}", i),
            task_type: "process_data".to_string(),
            priority: 5,
            estimated_duration_secs: 60,
        };

        let id = engine.create_work(work).await?;
        work_ids.push(id.clone());
        println!("  ✓ Created: {}", id);
    }
    println!("\n✓ Created {} work items\n", work_ids.len());

    // Agents claim work
    println!("Agents claiming work...");
    let claims = vec!["worker_1", "worker_2"];
    for (agent_id, work_id) in claims.iter().zip(work_ids.iter()) {
        match engine.claim_work(agent_id).await? {
            Some(work) => println!("  ✓ {} claimed work: {}", agent_id, work.id),
            None => println!("  ✗ {} could not claim work", agent_id),
        }
    }
    println!();

    println!("✅ Work distribution example completed!");
    Ok(())
}
EOF

cargo run --example work_distribution
```

---

## Step 6: Set Up Health Monitoring (Optional)

For production deployments, monitor system health:

```bash
# Create health monitoring script
cat > monitor_health.sh << 'EOF'
#!/bin/bash

SWARMSH_DATA_DIR="${SWARMSH_DATA_DIR:-.}/swarmsh_data"
HEALTH_CHECK_INTERVAL="${HEALTH_CHECK_INTERVAL:-30}"

echo "🏥 SwarmSH v2 Health Monitor"
echo "Data directory: $SWARMSH_DATA_DIR"
echo "Check interval: ${HEALTH_CHECK_INTERVAL}s"
echo ""

while true; do
    # Check if data directory exists
    if [ ! -d "$SWARMSH_DATA_DIR" ]; then
        echo "✗ $(date '+%H:%M:%S') - Data directory not found!"
        sleep $HEALTH_CHECK_INTERVAL
        continue
    fi

    # Check for lock files (indicator of activity)
    lock_count=$(find "$SWARMSH_DATA_DIR" -name "*.lock" | wc -l)
    echo "✓ $(date '+%H:%M:%S') - Lock files: $lock_count"

    # Check disk usage
    usage=$(du -sh "$SWARMSH_DATA_DIR" | cut -f1)
    echo "  Disk usage: $usage"

    # Check for any error files
    if find "$SWARMSH_DATA_DIR" -name "*.error" -type f | grep -q .; then
        echo "  ⚠️  Error files detected"
    fi

    sleep $HEALTH_CHECK_INTERVAL
done
EOF

chmod +x monitor_health.sh
./monitor_health.sh
```

---

## Step 7: Deploy to Shell (Optional)

Export your coordination logic to portable shell scripts:

```bash
# Generate shell script from template
cargo run --bin swarmsh-exporter \
    --template coordination_helper \
    --output ./generated_coordinator.sh 2>/dev/null || \
    echo "Note: swarmsh-exporter binary may not compile. Use core library instead."

# Make it executable
chmod +x ./generated_coordinator.sh

# Use in any environment
export SWARMSH_DATA_DIR=/data/swarmsh
./generated_coordinator.sh register-agent --id agent_shell_001 --role worker
```

---

## Common Issues & Troubleshooting

### Issue 1: "Compilation failed" for binaries

**Cause**: Binary executables have import errors (~60% fail)

**Solution**: Use the core library directly
```bash
# Instead of trying to run binaries
cargo build  # Will fail on binaries

# Use the library
cargo build --lib  # This works
cargo test --lib   # This works
```

### Issue 2: Data directory permission error

**Cause**: SWARMSH_DATA_DIR is not writable

**Solution**:
```bash
# Check permissions
ls -ld $SWARMSH_DATA_DIR

# Fix if needed
chmod 755 $SWARMSH_DATA_DIR
chmod 755 -R $SWARMSH_DATA_DIR/*
```

### Issue 3: OTEL spans not showing up

**Cause**: OTEL collector not configured or unreachable

**Solution**:
```bash
# Check if collector is running
curl -v $OTEL_EXPORTER_OTLP_ENDPOINT

# Switch to stdout for debugging
export OTEL_EXPORTER=stdout
export RUST_LOG=debug
cargo run --example basic_coordination
```

### Issue 4: Lock files accumulating

**Cause**: Cleanup not running or cleanup disabled

**Solution**: See [FMEA_RISK_MITIGATION.md](./FMEA_RISK_MITIGATION.md#lock-accumulation-rpn-180)

```bash
# Manual cleanup
find ./swarmsh_data -name "*.lock" -mtime +1 -delete
```

---

## Next Steps

After completing this guide, you can:

1. **Read [README.md](./README.md)** for detailed documentation structure
2. **Study [FMEA_ANALYSIS.md](./FMEA_ANALYSIS.md)** to understand failure modes
3. **Learn [POKA_YOKE_GUIDE.md](./POKA_YOKE_GUIDE.md)** to prevent common mistakes
4. **Review examples/** for more advanced use cases
5. **Check tests/** directory for additional patterns

---

## Quick Reference

```bash
# Build
cargo build --lib                   # Core library (stable)
cargo build                         # Everything (some will fail)

# Test
cargo test --lib                    # Core tests
cargo test coordination::tests      # Specific module tests

# Run
cargo run --example basic_coordination
cargo run --bin swarmsh-coordinator  # May fail

# Monitor
./monitor_health.sh                 # Health monitoring
grep "claim_work" application.log   # Work claiming activity

# Clean
rm -rf ./swarmsh_data              # Clear coordination data
find ./swarmsh_data -name "*.lock" -delete  # Clear locks only
```

---

## Support

- **Installation issues**: Check prerequisites above
- **Runtime errors**: See [FMEA_RISK_MITIGATION.md](./FMEA_RISK_MITIGATION.md)
- **API questions**: See [README.md](./README.md) Reference section
- **Design questions**: See [README.md](./README.md) Explanation section

**You're ready to go!** 🚀
