# SwarmSH v2 - API Reference

**Complete API documentation for the SwarmSH v2 coordination library.**

This reference covers all public types and methods in the core library. For usage examples, see [README.md - Tutorials](./README.md#-tutorials).

---

## Core Types

### `CoordinationEngine`

Main interface for distributed work coordination.

#### Creation

```rust
impl CoordinationEngine {
    /// Initialize the coordination engine with default configuration
    pub async fn new(data_dir: &str) -> Result<Self>

    /// Initialize with custom configuration
    pub async fn with_config(config: CoordinationConfig) -> Result<Self>

    /// Get the configured data directory
    pub fn data_dir(&self) -> &str
}
```

#### Agent Operations

```rust
impl CoordinationEngine {
    /// Register a new agent in the system
    pub async fn register_agent(&self, spec: AgentSpec) -> Result<()>

    /// Unregister an agent
    pub async fn unregister_agent(&self, agent_id: &str) -> Result<()>

    /// Get all registered agents
    pub async fn list_agents(&self) -> Result<Vec<AgentSpec>>

    /// Get a specific agent
    pub async fn get_agent(&self, agent_id: &str) -> Result<Option<AgentSpec>>

    /// Update agent capacity
    pub async fn update_agent_capacity(&self, agent_id: &str, capacity: f64) -> Result<()>
}
```

#### Work Operations

```rust
impl CoordinationEngine {
    /// Create a new work item
    pub async fn create_work(&self, spec: WorkSpec) -> Result<String>

    /// Agent claims available work
    pub async fn claim_work(&self, agent_id: &str) -> Result<Option<Work>>

    /// Agent claims specific work by ID
    pub async fn claim_work_by_id(&self, agent_id: &str, work_id: &str) -> Result<bool>

    /// Mark work as completed
    pub async fn complete_work(&self, agent_id: &str, work_id: &str) -> Result<()>

    /// Mark work as failed
    pub async fn fail_work(&self, agent_id: &str, work_id: &str, reason: &str) -> Result<()>

    /// Abandon work (return to queue without failure)
    pub async fn abandon_work(&self, agent_id: &str, work_id: &str) -> Result<()>

    /// Get work status
    pub async fn get_work_status(&self, work_id: &str) -> Result<WorkStatus>

    /// List work items by status
    pub async fn list_work(&self, status: WorkState) -> Result<Vec<Work>>
}
```

#### Pattern Operations

```rust
impl CoordinationEngine {
    /// Register a coordination pattern
    pub async fn register_pattern(&self, pattern: Box<dyn CoordinationPattern>) -> Result<()>

    /// Set the active coordination pattern
    pub async fn set_active_pattern(&self, name: &str) -> Result<()>

    /// Get the active pattern name
    pub fn active_pattern(&self) -> &str
}
```

#### Health & Monitoring

```rust
impl CoordinationEngine {
    /// Check system health
    pub async fn check_health(&self) -> Result<HealthStatus>

    /// Get performance metrics
    pub async fn get_metrics(&self) -> Result<Metrics>

    /// Clear old locks and state (maintenance)
    pub async fn cleanup_old_locks(&self) -> Result<usize>
}
```

---

### `AgentSpec`

Defines an agent's capabilities and configuration.

```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AgentSpec {
    /// Unique agent identifier
    pub id: String,

    /// Agent role (e.g., "worker", "coordinator")
    pub role: String,

    /// Resource capacity (0.0 to 1.0)
    /// 1.0 = fully available, 0.5 = 50% available
    pub capacity: f64,

    /// Areas of expertise (for filtering work)
    pub specializations: Vec<String>,

    /// Maximum concurrent work items
    pub work_capacity: Option<usize>,
}
```

#### Methods

```rust
impl AgentSpec {
    /// Validate agent specification
    pub fn validate(&self) -> Result<()>

    /// Check if agent has required specialization
    pub fn has_specialization(&self, specialization: &str) -> bool
}
```

#### Constraints

- `id`: Must be unique, non-empty, alphanumeric + underscore
- `capacity`: Must be between 0.0 and 1.0 (inclusive)
- `work_capacity`: If specified, must be > 0

#### Example

```rust
let agent = AgentSpec {
    id: "worker_001".to_string(),
    role: "processor".to_string(),
    capacity: 0.8,
    specializations: vec!["data_processing".to_string(), "ml_inference".to_string()],
    work_capacity: Some(10),
};

agent.validate()?;
engine.register_agent(agent).await?;
```

---

### `WorkSpec`

Defines a work item to be distributed.

```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkSpec {
    /// Unique work identifier
    pub id: String,

    /// Type of work (for routing/filtering)
    pub task_type: String,

    /// Priority (1-100, higher = more urgent)
    pub priority: u32,

    /// Estimated duration in seconds
    pub estimated_duration_secs: u64,
}
```

#### Methods

```rust
impl WorkSpec {
    /// Validate work specification
    pub fn validate(&self) -> Result<()>

    /// Check if this work is high priority
    pub fn is_urgent(&self) -> bool  // returns priority > 80
}
```

#### Constraints

- `id`: Must be unique, non-empty
- `priority`: Must be 1-100
- `estimated_duration_secs`: Must be > 0

#### Example

```rust
let work = WorkSpec {
    id: "job_001".to_string(),
    task_type: "process_batch".to_string(),
    priority: 50,
    estimated_duration_secs: 3600,
};

work.validate()?;
let work_id = engine.create_work(work).await?;
```

---

### `Work`

Represents an active work item (immutable view).

```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Work {
    /// Work specification
    pub spec: WorkSpec,

    /// Current state
    pub state: WorkState,

    /// Agent currently processing (if claimed)
    pub claimed_by: Option<String>,

    /// Timestamp when work was created
    pub created_at: SystemTime,

    /// Timestamp when work was claimed
    pub claimed_at: Option<SystemTime>,

    /// Number of times work was attempted
    pub attempt_count: u32,

    /// Version token for conflict detection
    #[serde(rename = "_version")]
    pub version: u64,
}
```

#### Methods

```rust
impl Work {
    /// Check if work is still pending
    pub fn is_pending(&self) -> bool

    /// Check if work is currently active
    pub fn is_active(&self) -> bool

    /// Get time since creation
    pub fn age(&self) -> Duration

    /// Get time since claimed
    pub fn claim_age(&self) -> Option<Duration>
}
```

---

### `WorkState`

Enumeration of possible work states.

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum WorkState {
    /// Work is waiting to be claimed
    Pending,

    /// Work is being processed
    Active,

    /// Work completed successfully
    Completed,

    /// Work failed and needs retry
    Failed,

    /// Work abandoned by agent
    Abandoned,
}
```

---

### `HealthStatus`

System health information.

```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HealthStatus {
    /// Is the system healthy?
    pub is_healthy: bool,

    /// Detected bottlenecks
    pub detected_bottlenecks: Vec<Bottleneck>,

    /// System load (0.0 to 1.0)
    pub system_load: f64,

    /// Available work items
    pub pending_work_count: usize,

    /// Active work items
    pub active_work_count: usize,

    /// Free disk space in bytes
    pub free_disk_space: u64,
}
```

```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bottleneck {
    pub name: String,
    pub description: String,
    pub severity: u32,  // 0-100
}
```

---

### `Metrics`

Performance metrics.

```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Metrics {
    /// Total agents registered
    pub total_agents: usize,

    /// Currently active agents
    pub active_agents: usize,

    /// Average work claiming latency (ms)
    pub avg_claim_latency_ms: f64,

    /// Average work completion time (s)
    pub avg_completion_time_secs: f64,

    /// Total work items processed
    pub total_work_processed: u64,

    /// Work failure rate (0.0 to 1.0)
    pub failure_rate: f64,
}
```

---

### `CoordinationConfig`

Configuration for the coordination engine.

```rust
#[derive(Clone, Debug)]
pub struct CoordinationConfig {
    /// Directory for coordination state
    pub data_dir: String,

    /// File lock timeout in seconds
    pub lock_timeout_secs: u64,

    /// Maximum agents allowed
    pub max_agents: usize,

    /// Enable health checks
    pub enable_health_checks: bool,

    /// Health check interval in seconds
    pub health_check_interval_secs: u64,
}
```

#### Default Values

```rust
impl Default for CoordinationConfig {
    fn default() -> Self {
        CoordinationConfig {
            data_dir: "./data".to_string(),
            lock_timeout_secs: 30,
            max_agents: 100,
            enable_health_checks: true,
            health_check_interval_secs: 10,
        }
    }
}
```

---

## Traits

### `CoordinationPattern`

Implement this trait to create custom coordination patterns.

```rust
pub trait CoordinationPattern: Send + Sync {
    /// Pattern name
    fn name(&self) -> &str;

    /// Determine if agent can claim work
    fn can_claim_work(&self, agent_id: &str, work_id: &str) -> bool;

    /// Called when work is claimed
    fn on_work_claimed(&self, agent_id: &str, work_id: &str) {}

    /// Called when work completes
    fn on_work_completed(&self, agent_id: &str, work_id: &str) {}

    /// Called when work fails
    fn on_work_failed(&self, agent_id: &str, work_id: &str, reason: &str) {}

    /// Get pattern configuration
    fn config(&self) -> serde_json::Value {
        serde_json::json!({})
    }
}
```

#### Example Implementation

```rust
pub struct MyCustomPattern;

impl CoordinationPattern for MyCustomPattern {
    fn name(&self) -> &str {
        "my_pattern"
    }

    fn can_claim_work(&self, agent_id: &str, work_id: &str) -> bool {
        // Your custom logic here
        true
    }

    fn on_work_completed(&self, agent_id: &str, work_id: &str) {
        println!("Agent {} completed work {}", agent_id, work_id);
    }
}
```

---

## Built-in Patterns

### Atomic Pattern

**Name**: `"atomic"`

First agent to claim work wins. No complex logic.

```rust
engine.set_active_pattern("atomic").await?;
```

### Scrum at Scale

**Name**: `"scrum_at_scale"`

Team-based work distribution with sprint management.

```rust
engine.set_active_pattern("scrum_at_scale").await?;
```

### Roberts Rules

**Name**: `"roberts_rules"`

Parliamentary procedure for consensus decisions.

```rust
engine.set_active_pattern("roberts_rules").await?;
```

### Real-Time

**Name**: `"real_time"`

Dynamic priority-based distribution.

```rust
engine.set_active_pattern("real_time").await?;
```

---

## Error Handling

### `SwarmError`

All public APIs return `Result<T, SwarmError>`.

```rust
pub enum SwarmError {
    // File system errors
    FileNotFound(String),
    PermissionDenied(String),
    IoError(String),

    // State errors
    AgentNotFound(String),
    WorkNotFound(String),
    InvalidState(String),

    // Validation errors
    ValidationError(String),

    // Lock errors
    LockAcquisitionFailed(String),
    LockTimeout(String),

    // Other
    InternalError(String),
}
```

#### Error Handling Pattern

```rust
use swarmsh_v2::SwarmError;

match engine.claim_work("agent_001").await {
    Ok(Some(work)) => {
        println!("Claimed work: {}", work.spec.id);
    }
    Ok(None) => {
        println!("No work available");
    }
    Err(SwarmError::LockAcquisitionFailed(msg)) => {
        eprintln!("Lock error: {}", msg);
        // Handle lock error specifically
    }
    Err(e) => {
        eprintln!("Error: {}", e);
        // Handle other errors
    }
}
```

---

## Feature Flags

Compile with optional features:

```toml
# In your Cargo.toml
swarmsh-v2 = { version = "2.1", features = ["full-telemetry", "ai-integration"] }
```

| Feature | Enables | Default |
|---------|---------|---------|
| `jaeger` | Jaeger export | ✅ |
| `prometheus` | Prometheus metrics | ✅ |
| `otlp` | OpenTelemetry Protocol | ✅ |
| `stdout` | Console output | ✅ |
| `shell-export` | Shell script generation | ✅ |
| `ai-integration` | Ollama/Claude integration | ✅ |
| `full-telemetry` | All telemetry features | ❌ |
| `revolutionary-platform` | All features combined | ❌ |

---

## Environment Variables

Configuration via environment variables:

| Variable | Type | Default | Purpose |
|----------|------|---------|---------|
| `SWARMSH_DATA_DIR` | String | `./data` | Coordination data directory |
| `SWARMSH_LOCK_TIMEOUT` | u64 | `30` | Lock timeout in seconds |
| `SWARMSH_MAX_AGENTS` | usize | `100` | Maximum agents allowed |
| `RUST_LOG` | String | `info` | Logging level |
| `OTEL_EXPORTER_OTLP_ENDPOINT` | String | None | OTEL collector endpoint |
| `SWARMSH_TELEMETRY` | bool | `false` | Enable telemetry |

#### Example

```bash
export SWARMSH_DATA_DIR=/data/swarmsh
export SWARMSH_LOCK_TIMEOUT=60
export RUST_LOG=debug
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317

cargo run --example my_program
```

---

## Async Runtime

SwarmSH v2 uses Tokio async runtime.

```rust
#[tokio::main]
async fn main() {
    let engine = CoordinationEngine::new("./data").await.unwrap();

    // Async operations
    let agents = engine.list_agents().await.unwrap();

    // Spawning tasks
    tokio::spawn(async {
        engine.claim_work("agent_001").await.ok();
    });
}
```

---

## Thread Safety

All types implement `Send + Sync`:

```rust
// Safe to share across threads
let engine = Arc::new(engine);

// Safe to clone for each thread
for i in 0..10 {
    let engine = engine.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            engine.claim_work(&format!("agent_{}", i)).await.ok();
        });
    });
}
```

---

## Serialization

All public types support JSON serialization:

```rust
use serde_json;

let agent = AgentSpec { /* ... */ };

// To JSON
let json = serde_json::to_string_pretty(&agent)?;

// From JSON
let agent: AgentSpec = serde_json::from_str(&json)?;

// To file
serde_json::to_writer(std::fs::File::create("agent.json")?, &agent)?;
```

---

## Complete Example

```rust
use swarmsh_v2::coordination::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create engine
    let engine = CoordinationEngine::new("./data").await?;

    // Register agents
    for i in 1..=3 {
        let agent = AgentSpec {
            id: format!("worker_{}", i),
            role: "processor".to_string(),
            capacity: 1.0,
            specializations: vec!["compute".to_string()],
            work_capacity: Some(10),
        };
        engine.register_agent(agent).await?;
    }

    // Create work
    let work = WorkSpec {
        id: "job_001".to_string(),
        task_type: "process_batch".to_string(),
        priority: 50,
        estimated_duration_secs: 3600,
    };
    engine.create_work(work).await?;

    // Agents claim work
    if let Some(work) = engine.claim_work("worker_1").await? {
        println!("Claimed: {}", work.spec.id);

        // Do work...

        // Mark complete
        engine.complete_work("worker_1", &work.spec.id).await?;
    }

    // Check health
    let health = engine.check_health().await?;
    println!("System healthy: {}", health.is_healthy);

    Ok(())
}
```

---

## See Also

- [README.md](./README.md) - Complete documentation
- [GETTING_STARTED.md](./GETTING_STARTED.md) - Installation and first steps
- [TROUBLESHOOTING.md](./TROUBLESHOOTING.md) - Common issues and solutions
- [FMEA_ANALYSIS.md](./FMEA_ANALYSIS.md) - Failure mode analysis

---

**Generated from SwarmSH v2 v2.1.0 - Last updated: 2025-11-16**
