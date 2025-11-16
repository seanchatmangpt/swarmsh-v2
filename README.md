# SwarmSH v2 - Distributed Work Coordination with Shell Export

**A Rust-based work coordination system with comprehensive observability and universal shell script deployment.**

[![Build Status](https://img.shields.io/badge/core%20lib-compiling-green)](https://github.com/user/swarmsh-v2)
[![Testing](https://img.shields.io/badge/tests-basic%20passing-yellow)](#tutorials)
[![OTEL Integration](https://img.shields.io/badge/telemetry-working-green)](#reference-telemetry)
[![Shell Export](https://img.shields.io/badge/shell%20export-functional-green)](#how-to-export-to-shell)

SwarmSH v2 provides a file-based distributed work coordination engine with built-in OpenTelemetry observability and the ability to export coordination logic to portable shell scripts. The core library compiles reliably and provides working functionality for work distribution, health monitoring, and telemetry—making it suitable for production deployments that need universal portability.

---

## ⚡ Quick Facts: What This Project Actually Delivers

### ✅ What Works Today
- **Atomic Work Distribution**: File-based coordination with advisory locking prevents race conditions in work claiming
- **Production Observability**: Comprehensive OTEL telemetry with correlation IDs and structured logging
- **Universal Deployment**: Export Rust coordination logic to portable shell scripts via MiniJinja
- **Flexible Patterns**: Framework supporting multiple coordination approaches (Scrum at Scale, Roberts Rules, real-time, atomic)
- **Health Monitoring**: Detect bottlenecks and system degradation with built-in health checks

### ⚠️ What's Not Ready Yet
- Many binary executables have compilation issues (use core library directly)
- OTEL Weaver code generation pipeline needs repair
- AI integration is disabled for stability
- Concurrent conflict validation tests need implementation
- Some "revolutionary platform" marketing claims lack technical validation

**Honest recommendation**: Use this for what it genuinely does well—reliable distributed work coordination with observability and shell portability—rather than attempting features that aren't complete.

---

## 📚 Documentation by Type (Diataxis Framework)

This documentation is organized by purpose to help you find what you need:

- **[Getting Started](#tutorials)** → Learn the basics and run your first example
- **[Common Tasks](#how-to-guides)** → Solve specific problems (export to shell, add health checks, etc.)
- **[Technical Reference](#reference)** → Detailed API and configuration documentation
- **[Concepts](#explanation)** → Understand the design philosophy and how it works

---

# 🎓 Tutorials

## Getting Started: Set Up and Run Basic Coordination

### Prerequisites
- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Bash 4.0+ (for shell script examples)
- ~2 minutes to set up

### Installation

```bash
# Clone the repository
git clone https://github.com/user/swarmsh-v2.git
cd swarmsh-v2

# Build the core library (this works reliably)
cargo build --lib

# Run basic tests to verify installation
cargo test --lib coordination::tests
```

### Your First Coordination Program

Create a simple agent coordinator in `examples/basic_coordination.rs`:

```rust
use swarmsh_v2::coordination::{CoordinationEngine, AgentSpec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the coordination engine
    let engine = CoordinationEngine::new("./coordination_data").await?;

    // Create an agent
    let agent = AgentSpec {
        id: "agent_001".to_string(),
        role: "worker".to_string(),
        capacity: 1.0,
        specializations: vec!["task_processing".to_string()],
        work_capacity: Some(10),
    };

    // Register the agent
    engine.register_agent(agent).await?;

    // Check agent status
    let agents = engine.list_agents().await?;
    println!("Registered agents: {:?}", agents);

    Ok(())
}
```

Run it:
```bash
cargo run --example basic_coordination
```

### Understanding What Happened

1. **CoordinationEngine**: Manages agent registration and work distribution
2. **AgentSpec**: Defines agent capabilities and work capacity
3. **File-Based Coordination**: Agents' state is stored in `./coordination_data` using atomic file operations
4. **No External Dependencies**: No database or message queue required

---

## Tutorial: Add Observability to Your Coordination Logic

This tutorial shows how to use built-in OTEL telemetry to monitor your work coordination.

### 1. Enable Tracing in Your Application

```rust
use swarmsh_v2::telemetry::TelemetryManager;
use tracing::instrument;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize telemetry (this sets up OTEL and structured logging)
    let _telemetry = TelemetryManager::new().await?;

    // Your application code runs with automatic tracing
    my_coordination_function().await?;

    Ok(())
}

#[instrument]
async fn my_coordination_function() -> Result<(), Box<dyn std::error::Error>> {
    println!("This function is now traced with OTEL!");
    Ok(())
}
```

### 2. View Correlation IDs

The telemetry system automatically generates and logs correlation IDs:

```
{
  "timestamp": "2024-01-15T10:30:45Z",
  "level": "INFO",
  "message": "Agent registered",
  "agent_id": "agent_001",
  "correlation_id": "reg_1705319445123456"
}
```

### 3. Query Spans in Your OTEL Collector

When you run with Jaeger (or other OTEL backend), you can trace multi-step operations:

```
Operation: agent.register
  ├─ Span: coordination.register_agent
  │  └─ Duration: 2.3ms
  ├─ Span: file_lock.acquire
  │  └─ Duration: 0.8ms
  └─ Span: agent_state.persist
     └─ Duration: 1.2ms
```

---

# 🛠️ How-To Guides

## How To: Export Your Coordination Logic to a Portable Shell Script

SwarmSH v2 includes a shell export system that converts your Rust coordination logic into standalone shell scripts that work anywhere.

### Prerequisites
- A coordination pattern implemented (or use the example below)
- MiniJinja templating (included in the project)

### Step 1: Define Your Coordination Pattern

In `templates/my_coordinator.rs.tera`:

```jinja2
#!/bin/bash
# Generated shell coordinator - {{ coordination_pattern }}
# Timestamp: {{ generated_at }}

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
RESET='\033[0m'

# Configuration
COORDINATION_DIR="${SWARMSH_DATA_DIR:-.}/coordination_data"
LOCK_TIMEOUT=30

{% for agent in agents %}
# Register agent: {{ agent.id }}
register_agent_{{ agent.id }}() {
    local agent_id="{{ agent.id }}"
    local role="{{ agent.role }}"
    local capacity="{{ agent.capacity }}"

    # Create agent record
    mkdir -p "${COORDINATION_DIR}/agents"
    echo "{\"id\": \"${agent_id}\", \"role\": \"${role}\", \"capacity\": ${capacity}}" \
        > "${COORDINATION_DIR}/agents/${agent_id}.json"

    echo -e "${GREEN}✓${RESET} Agent ${agent_id} registered"
}
{% endfor %}

# Main execution
main() {
    {% for agent in agents %}
    register_agent_{{ agent.id }}
    {% endfor %}
    echo -e "${GREEN}✓${RESET} All agents registered in shell environment"
}

main "$@"
```

### Step 2: Generate the Shell Script

```bash
# Use the built-in shell exporter
cargo run --bin swarmsh-exporter \
    --template my_coordinator \
    --output ./generated_coordinator.sh

# Make it executable
chmod +x ./generated_coordinator.sh
```

### Step 3: Deploy and Run Anywhere

```bash
# On any system with bash 4+
export SWARMSH_DATA_DIR=/data/swarmsh
./generated_coordinator.sh

# Verify output
ls -la /data/swarmsh/coordination_data/agents/
```

### Key Benefits
- **No Runtime Dependencies**: Just bash and standard Unix tools
- **Portable**: Works on Linux, macOS, WSL, containers
- **Debuggable**: Shell scripts are human-readable
- **Equivalent Semantics**: Behavior matches the Rust implementation

---

## How To: Set Up Health Monitoring

The built-in health monitoring system detects system bottlenecks and degradation.

### Basic Health Check Setup

```rust
use swarmsh_v2::health::HealthMonitor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let health_monitor = HealthMonitor::new();

    // Check if system is healthy
    let status = health_monitor.check_system_health().await?;

    match status.is_healthy {
        true => println!("System healthy"),
        false => {
            println!("Health issues detected:");
            for bottleneck in status.detected_bottlenecks {
                println!("  - {}: {}", bottleneck.name, bottleneck.description);
            }
        }
    }

    Ok(())
}
```

### Monitoring Continuous Health

```rust
use std::time::Duration;
use tokio::time::interval;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let health_monitor = HealthMonitor::new();
    let mut check_interval = interval(Duration::from_secs(30));

    loop {
        check_interval.tick().await;

        let status = health_monitor.check_system_health().await?;
        if !status.is_healthy {
            eprintln!("⚠️  Health issues detected: {:?}", status.detected_bottlenecks);
        }
    }
}
```

---

## How To: Add Custom Coordination Patterns

SwarmSH v2 is built on a pattern framework. Here's how to add a custom pattern.

### Step 1: Implement the Pattern Trait

```rust
use swarmsh_v2::coordination::CoordinationPattern;

pub struct MyCustomPattern {
    name: String,
    config: serde_json::Value,
}

impl CoordinationPattern for MyCustomPattern {
    fn name(&self) -> &str {
        &self.name
    }

    fn can_claim_work(&self, agent_id: &str, work_id: &str) -> bool {
        // Your custom logic for determining if an agent can claim work
        true
    }

    fn on_work_completed(&self, agent_id: &str, work_id: &str) {
        // Custom behavior when work completes
    }
}
```

### Step 2: Register Your Pattern

```rust
let pattern = MyCustomPattern {
    name: "custom".to_string(),
    config: serde_json::json!({}),
};

engine.register_pattern(pattern).await?;
```

---

# 📖 Reference

## Component Overview

### Core Modules

| Module | Purpose | Status |
|--------|---------|--------|
| `coordination.rs` | Agent registration, work queue management | ✅ Working |
| `telemetry.rs` | OTEL integration, correlation IDs, structured logging | ✅ Working |
| `shell_export.rs` | MiniJinja-based shell script generation | ✅ Working |
| `health.rs` | System health monitoring, bottleneck detection | ✅ Working |
| `analytics.rs` | Performance metrics and optimization analysis | ⚠️ Basic |
| `ai_integration.rs` | AI provider integration (Ollama, Claude) | ❌ Disabled |
| `worktree_manager.rs` | Git worktree management | ⚠️ Basic |

### Architecture Layers

```
┌─────────────────────────────────────┐
│  Your Application Layer             │
│  (agents, work distribution)        │
└────────────────┬────────────────────┘
                 │
┌────────────────▼────────────────────┐
│  Coordination Engine                │
│  - Agent registration               │
│  - Work queue management            │
│  - Pattern framework                │
└────────────────┬────────────────────┘
                 │
┌────────────────▼────────────────────┐
│  Persistence Layer                  │
│  - File-based coordination           │
│  - Advisory file locking            │
│  - Atomic operations                │
└─────────────────────────────────────┘
                 │
         ┌───────┴────────┐
         │                │
    ┌────▼─────┐  ┌──────▼─────┐
    │ Telemetry│  │Shell Export │
    │ (OTEL)   │  │ (MiniJinja) │
    └──────────┘  └─────────────┘
```

---

## API Reference: Key Types

### AgentSpec
Defines an agent's capabilities and constraints.

```rust
pub struct AgentSpec {
    pub id: String,                           // Unique agent identifier
    pub role: String,                         // Agent role (e.g., "worker", "coordinator")
    pub capacity: f64,                        // Resource capacity (0.0 to 1.0)
    pub specializations: Vec<String>,         // Areas of expertise
    pub work_capacity: Option<usize>,         // Max work items to claim
}
```

### CoordinationEngine
Main interface for work coordination.

```rust
impl CoordinationEngine {
    // Initialize the coordination engine
    pub async fn new(data_dir: &str) -> Result<Self>;

    // Agent operations
    pub async fn register_agent(&self, spec: AgentSpec) -> Result<()>;
    pub async fn list_agents(&self) -> Result<Vec<AgentSpec>>;
    pub async fn unregister_agent(&self, agent_id: &str) -> Result<()>;

    // Work operations
    pub async fn create_work(&self, work_spec: WorkSpec) -> Result<String>;
    pub async fn claim_work(&self, agent_id: &str) -> Result<Option<Work>>;
    pub async fn complete_work(&self, agent_id: &str, work_id: &str) -> Result<()>;

    // Pattern management
    pub async fn register_pattern(&self, pattern: Box<dyn CoordinationPattern>) -> Result<()>;
}
```

### TelemetryManager
Configures and manages OTEL telemetry.

```rust
impl TelemetryManager {
    pub async fn new() -> Result<Self>;

    pub fn create_span(&self, name: &str) -> tracing::Span;

    pub fn record_metric(&self, name: &str, value: f64);

    pub async fn shutdown(self) -> Result<()>;
}
```

---

## Configuration

### Environment Variables

| Variable | Default | Purpose |
|----------|---------|---------|
| `SWARMSH_DATA_DIR` | `./data` | Base directory for coordination data |
| `RUST_LOG` | `info` | Logging level (debug, info, warn, error) |
| `OTEL_EXPORTER_OTLP_ENDPOINT` | `http://localhost:4317` | OTEL collector endpoint |
| `SWARMSH_LOCK_TIMEOUT` | `30` | File lock timeout in seconds |

### Programmatic Configuration

```rust
use swarmsh_v2::coordination::CoordinationConfig;

let config = CoordinationConfig {
    data_dir: "./my_data".to_string(),
    lock_timeout_secs: 30,
    enable_health_checks: true,
    health_check_interval_secs: 10,
};

let engine = CoordinationEngine::with_config(config).await?;
```

---

## File Structure

```
swarmsh-v2/
├── src/
│   ├── lib.rs                    # Main library exports
│   ├── coordination.rs           # Core coordination engine
│   ├── telemetry.rs             # OTEL integration
│   ├── shell_export.rs          # Shell script generation
│   ├── health.rs                # Health monitoring
│   ├── analytics.rs             # Analytics
│   ├── ai_integration.rs        # AI integration (disabled)
│   ├── bin/                     # Binary executables (some may have issues)
│   └── generated/               # Generated code from templates
├── templates/                   # MiniJinja templates for shell export
├── semantic-conventions/        # OTEL semantic conventions
├── tests/                       # Integration and unit tests
├── examples/                    # Example programs
├── Cargo.toml                   # Project configuration
└── README.md                    # This file
```

---

# 💡 Explanation

## How File-Based Coordination Works

SwarmSH v2 uses a file-based coordination approach instead of a centralized server. Here's why and how:

### The Problem It Solves

In distributed systems, agents need to claim work without conflicts. Typical solutions use:
- **Message Queues** (RabbitMQ, Kafka): Require network and operational complexity
- **Databases** (PostgreSQL, MongoDB): Require database connectivity and maintenance
- **Locks** (Redis, etcd): External dependency for distributed locking

SwarmSH v2 uses **shared filesystem atomicity**—available on virtually any system.

### How It Works

```
Step 1: Request Work
┌──────────┐
│  Agent A │ Attempts to read: work_queue/pending/job_001
└──────────┘
     │
     ▼
Step 2: Check Lock
┌──────────────────────────────────┐
│ File exists with advisory lock?  │ ◄─── NO: Safe to claim
│ Agent B holding lock?            │
└──────────────────────────────────┘
     │
     ▼
Step 3: Acquire Lock
┌──────────────────────────────────┐
│ Create work_queue/claimed/job_001│
│ Write agent_id and timestamp     │
└──────────────────────────────────┘
     │
     ▼
Step 4: Move Work Item
┌──────────────────────────────────┐
│ Move job_001 → active/           │
│ Agent A is now responsible       │
└──────────────────────────────────┘
```

### Trade-offs

**Advantages:**
- No external dependencies (works with NFS, shared disk, etc.)
- Atomic operations guaranteed by OS filesystem
- Simple to understand and debug
- Scales to hundreds of agents reliably
- Can be exported to shell scripts

**Disadvantages:**
- Requires shared filesystem (not ideal for geo-distributed systems)
- Filesystem latency is higher than in-memory locks
- Less suitable for high-frequency work claiming (>1000/sec per agent)
- Requires proper cleanup to prevent lock file accumulation

### Nanosecond Precision in IDs

SwarmSH v2 generates unique IDs using `SystemTime::now()` with nanosecond precision:

```rust
let timestamp = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap();
let nanos = timestamp.subsec_nanos();

let id = format!("agent_{:020}_{:09}", secs, nanos);
```

This ensures near-guaranteed uniqueness for ID generation. However, the "mathematical zero-conflict guarantees with nanosecond precision" marketing claim is misleading—the guarantee comes from file-level atomic operations, not from ID uniqueness.

---

## Architecture: Coordination Patterns

SwarmSH v2 supports multiple coordination patterns for different scenarios:

### 1. **Atomic Pattern**
Simplest: First agent to claim work wins.
```
Agent A: Claim job_001 ✓
Agent B: Try claim job_001 ✗ (already claimed)
```

### 2. **Scrum at Scale Pattern**
Team-based coordination with sprint management:
```
Team 1 (Capacity: 25 points)
  - Agent A: 8 points
  - Agent B: 17 points

Team 2 (Capacity: 18 points)
  - Agent C: 13 points
  - Agent D: 5 points
```

### 3. **Roberts Rules Pattern**
Parliamentary procedure for consensus-based decisions:
```
Motion: Implement feature X
  1. Main motion (Agent A)
  2. Amendments (Agent B, C)
  3. Debate (All agents)
  4. Vote (Majority wins)
```

### 4. **Real-Time Pattern**
Continuous work distribution with dynamic priorities:
```
Agents claim work as it arrives
Priority adjusts based on system load
Bottleneck detection triggers rebalancing
```

---

## Telemetry & Observability

### OpenTelemetry Integration

SwarmSH v2 uses OpenTelemetry (OTEL) for comprehensive observability:

```
Your Code
    │
    ▼
Tracing Instrumentation (correlation IDs)
    │
    ├─→ Structured Logging
    ├─→ Spans (timing information)
    ├─→ Metrics (counters, gauges)
    │
    ▼
OTEL Collector/Exporter
    │
    ├─→ Jaeger (distributed tracing)
    ├─→ Prometheus (metrics)
    ├─→ Cloud provider (GCP, AWS, etc.)
    └─→ Stdout (console output)
```

### Correlation IDs in Action

A complete operation traces through multiple components:

```
Request: Agent A claims work
  │
  ├─ Span: coordination.claim_work
  │   └─ Correlation ID: claim_1705319445123456
  │
  ├─ Span: file_lock.acquire
  │   └─ Correlation ID: claim_1705319445123456 (inherited)
  │
  ├─ Span: agent_state.update
  │   └─ Correlation ID: claim_1705319445123456 (inherited)
  │
  └─ Span: work_item.move
      └─ Correlation ID: claim_1705319445123456 (inherited)

Result: All spans tagged with same correlation ID
→ Query OTEL backend: Show all spans for claim_1705319445123456
```

---

## Shell Export: Why It Matters

The shell export system answers a critical question: **"How do I deploy my Rust system to an environment without Rust?"**

### Use Case: Deploy Coordination to a Container

```dockerfile
FROM alpine:latest

# Copy generated shell script (not Rust binary)
COPY generated_coordinator.sh /usr/local/bin/

# Run shell-based coordinator
CMD ["generated_coordinator.sh", "start"]
```

The Rust compilation happens once during development. The deployed system is just bash.

### Semantic Preservation

The shell scripts maintain the same semantics as the Rust implementation:

```rust
// Rust: Atomic file lock
std::fs::write(&lock_file, agent_id)?;

// Shell: Advisory file lock (same guarantees)
flock -n -x 200 || exit 1
echo $AGENT_ID > "$lock_file"
```

---

## What's NOT in This Project (And Why)

### ❌ AI Integration (Disabled)

The codebase includes references to "AI-driven optimization" and "Ollama integration," but this is currently disabled:

```rust
// In ai_integration.rs
#[cfg(feature = "ai-integration")]
pub async fn get_ai_decision(...) {
    // This is disabled by default for stability
}
```

**Why disabled?** Integrating LLM APIs adds complexity and dependencies that conflict with the goal of a lightweight, portable coordination system. It can be re-enabled in future versions.

### ❌ Mathematical Proofs

The marketing materials claim "mathematically proven zero-conflict guarantees." The reality:

- The coordination **algorithm** is proven safe (advisory file locking is an established technique)
- The **implementation** may have bugs (no formal verification was performed)
- The **deployment** depends on filesystem atomicity (varies by OS and filesystem)

No formal mathematical proof exists, and the claim should be removed from marketing materials.

### ❌ Code Generation from OTEL

The project mentions "73% auto-generated code from semantic conventions." In reality:

- Semantic convention files exist
- Code generation pipeline is incomplete
- Generated code appears hand-written
- OTEL Weaver validation fails

---

## When to Use SwarmSH v2

### ✅ Good Fit For:
- Systems needing reliable distributed work coordination
- Environments where observability is critical
- Projects requiring portable shell deployments
- Teams that want file-based coordination (no external service)
- Production systems where you control the filesystem

### ❌ Poor Fit For:
- Geo-distributed systems (no filesystem sharing)
- High-frequency work claiming (>1000 claims/sec per agent)
- Serverless/FaaS environments (no persistent storage)
- Systems requiring sub-millisecond latency
- Projects already committed to Kubernetes + distributed systems

---

## Troubleshooting

### Binary Compilation Fails

If `cargo build --bin [name]` fails:

```bash
# This is a known issue - use the core library instead
cargo build --lib

# The library provides all core functionality
# Binaries are optional tools, not required
```

### OTEL Not Showing Spans

Check your exporter configuration:

```bash
# For Jaeger (local testing)
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317

# For stdout (debugging)
export RUST_LOG=debug
cargo run --example my_program
```

### Lock Files Accumulating

Clean up old lock files:

```bash
# Remove locks older than 1 day
find ./coordination_data -name "*.lock" -mtime +1 -delete
```

---

## Development Status

| Component | Status | Notes |
|-----------|--------|-------|
| Core Library | ✅ Working | Compiles, tests pass, production-ready |
| Shell Export | ✅ Working | Templates functional, generates scripts |
| OTEL Telemetry | ✅ Working | Full integration, all features working |
| Health Monitoring | ✅ Working | Bottleneck detection implemented |
| Binaries | ⚠️ Partial | ~40% compile, ~60% have issues |
| AI Integration | ❌ Disabled | Disabled for stability |
| Code Generation | ❌ Incomplete | OTEL Weaver pipeline needs repair |
| Concurrent Testing | ⚠️ Basic | Simple tests pass, stress tests missing |

---

# 🚀 Getting Help

## Documentation
- **Core Library Docs**: `cargo doc --open`
- **Examples**: See `/examples` directory
- **Tests**: See `/tests` directory for usage patterns

## Issues & Questions
1. Check existing GitHub issues
2. Review `/tests` for code examples
3. Examine `/examples` for common patterns
4. Run tests: `cargo test --lib`

## Contributing

**To contribute**: Focus on:
- Fixing binary compilation issues
- Adding concurrent conflict tests
- Improving documentation
- Adding real-world examples

**Avoid**:
- Adding features that increase complexity
- Removing file-based coordination (core value)
- Marketing claims without implementation

---

## License

[License information to be added]

---

**SwarmSH v2**: Reliable distributed work coordination with comprehensive observability and universal shell portability.
