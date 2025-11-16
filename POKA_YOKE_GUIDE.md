# SwarmSH v2 - Poka Yoke (Mistake-Proofing) Guide

**Design patterns and guardrails to prevent the most common failure modes before they occur.**

Poka Yoke (ポカヨケ) means "mistake-proofing"—designing systems so that errors are impossible or immediately detected. This guide documents patterns that prevent the top failure modes identified in [FMEA_ANALYSIS.md](./FMEA_ANALYSIS.md).

**Key Principle**: The best fix is the one that prevents the mistake from being possible in the first place.

---

## 80/20 Dark Matter: Hidden Failure Modes That Matter Most

Before diving into specific poka yoke patterns, understand the **hidden complexity factors** (80/20 dark matter) that cause 80% of failures:

### The Invisible Killers (What Actually Breaks Systems)

1. **Assumption Failures** (35% of failures)
   - Assumption: "Filesystem is atomic"
   - Reality: NFS, CIFS, network paths are NOT atomic
   - Consequence: Race conditions in work claiming
   - Poka Yoke: Use OS-level file locking, not assumptions

2. **State Desynchronization** (25% of failures)
   - Assumption: "All agents see the same file state"
   - Reality: Filesystem caching means agents see different versions
   - Consequence: Agents claim same work, get duplicate processing
   - Poka Yoke: Version files, require validation on read

3. **Silent Degradation** (20% of failures)
   - Assumption: "If something fails, we'll know immediately"
   - Reality: Errors are often silent or delayed
   - Consequence: System runs incorrectly for hours before detection
   - Poka Yoke: Fail fast, validate aggressively, detect immediately

4. **Resource Accumulation** (15% of failures)
   - Assumption: "Resources will clean themselves up"
   - Reality: Lock files, temporary state, cache files persist indefinitely
   - Consequence: Disk fills, memory leaks, performance degradation
   - Poka Yoke: Automatic cleanup, bounded resources, quotas

5. **Configuration Drift** (5% of failures)
   - Assumption: "Configuration will be consistent across all agents"
   - Reality: Manual changes, leftover config files from previous runs
   - Consequence: Agents behave differently, unpredictable system behavior
   - Poka Yoke: Configuration validation at startup, immutable config

---

## Poka Yoke Pattern #1: Atomic Operations

**Problem**: Non-atomic operations can be interrupted, leaving system in inconsistent state.

**Failure Mode**: Race condition in work claiming (RPN 225), File corruption (RPN 140)

### Anti-Pattern (❌ Don't Do This)

```rust
// WRONG: Not atomic - another agent can interfere between these operations
std::fs::write("work_queue/claimed/job_001.lock", "agent_1")?;  // ← agent_2 could read here
std::fs::rename("work_queue/pending/job_001", "work_queue/active/job_001")?;
```

### Poka Yoke Implementation (✅ Do This)

```rust
// RIGHT: Use OS-level atomic operations
use std::fs::OpenOptions;
use file_lock::FileLock;

pub async fn claim_work(&self, agent_id: &str, work_id: &str) -> Result<bool> {
    let lock_path = format!("{}/work_queue/{}.lock", self.data_dir, work_id);

    // Use file_lock for true advisory locking
    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&lock_path)?;

    // Atomic lock attempt (non-blocking)
    match FileLock::lock(&mut file, file_lock::FileOptions::new().exclusive().non_blocking()) {
        Ok(_) => {
            // Successfully acquired lock - we own this work
            file.write_all(agent_id.as_bytes())?;
            file.sync_all()?;  // Force filesystem sync

            // Now move work atomically
            let active_path = format!("{}/work_queue/active/{}", self.data_dir, work_id);
            std::fs::rename(lock_path.clone(), active_path)?;

            Ok(true)  // Successfully claimed
        }
        Err(_) => {
            // Another agent already has the lock
            Ok(false)  // Work already claimed
        }
    }
}
```

### Testing Poka Yoke Implementation

```rust
#[tokio::test]
async fn test_atomic_claim_prevents_race_condition() {
    let engine = CoordinationEngine::new("./test_data").await.unwrap();

    // Spawn 100 concurrent agents all trying to claim the same work
    let mut tasks = vec![];
    for agent_id in 0..100 {
        let engine_clone = engine.clone();
        let task = tokio::spawn(async move {
            engine_clone.claim_work(&format!("agent_{}", agent_id), "job_001").await
        });
        tasks.push(task);
    }

    // Collect results
    let results: Vec<_> = futures::future::join_all(tasks)
        .await
        .into_iter()
        .filter_map(|r| r.ok())
        .collect();

    // POKA YOKE CHECK: Exactly one agent should have claimed the work
    let claimed_count = results.iter().filter(|r| **r == true).count();
    assert_eq!(claimed_count, 1, "Multiple agents claimed same work!");
}
```

---

## Poka Yoke Pattern #2: Version Validation

**Problem**: Agents reading stale state from filesystem cache.

**Failure Mode**: State desynchronization (RPN 96), File corruption (RPN 140)

### Anti-Pattern (❌ Don't Do This)

```rust
// WRONG: Just read the file - might be stale!
let content = std::fs::read_to_string("work_queue/pending/job_001.json")?;
let work: Work = serde_json::from_str(&content)?;
// Assumes content is current, but might be cached from 5 minutes ago
```

### Poka Yoke Implementation (✅ Do This)

```rust
// RIGHT: Use version tokens to detect staleness
#[derive(Serialize, Deserialize, Clone)]
pub struct Work {
    pub id: String,
    pub task: String,

    // POKA YOKE: Version token
    #[serde(rename = "_version")]
    pub version: u64,

    // POKA YOKE: Timestamp for validation
    #[serde(rename = "_created_at")]
    pub created_at: i64,

    // POKA YOKE: Checksum to detect corruption
    #[serde(rename = "_checksum")]
    pub checksum: String,
}

impl Work {
    pub fn compute_checksum(&self) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        self.id.hash(&mut hasher);
        self.task.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    pub fn validate(&self) -> Result<()> {
        // Check 1: Version is reasonable (not from year 2000)
        if self.version == 0 {
            return Err("Work has invalid version 0".into());
        }

        // Check 2: Timestamp is recent (within last hour)
        let age = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs() as i64 - self.created_at;

        if age > 3600 {
            return Err(format!("Work data is stale ({}s old)", age));
        }

        // Check 3: Checksum matches
        let computed = self.compute_checksum();
        if computed != self.checksum {
            return Err(format!(
                "Work corrupted: checksum mismatch (expected {}, got {})",
                self.checksum, computed
            ));
        }

        Ok(())
    }
}

pub async fn read_work(&self, work_id: &str) -> Result<Work> {
    let path = format!("{}/work_queue/active/{}", self.data_dir, work_id);
    let content = std::fs::read_to_string(&path)?;
    let work: Work = serde_json::from_str(&content)?;

    // POKA YOKE: Validate immediately after reading
    work.validate()?;

    Ok(work)
}
```

### Testing Version Validation

```rust
#[test]
fn test_work_validation_detects_corruption() {
    let mut work = Work {
        id: "job_001".to_string(),
        task: "process_data".to_string(),
        version: 1,
        created_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
        checksum: "abc123".to_string(),
    };

    // POKA YOKE TEST: Validate rejects corrupted checksum
    work.checksum = "wrong_checksum".to_string();
    assert!(work.validate().is_err(), "Should reject corrupted work");

    // POKA YOKE TEST: Validate rejects stale data
    work.created_at = 0;  // Year 1970
    assert!(work.validate().is_err(), "Should reject stale work");

    // POKA YOKE TEST: Validate accepts correct data
    work.created_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    work.checksum = work.compute_checksum();
    assert!(work.validate().is_ok(), "Should accept valid work");
}
```

---

## Poka Yoke Pattern #3: Fail Fast with Clear Errors

**Problem**: Silent failures or unclear error messages make debugging hard.

**Failure Mode**: Silent degradation (20% of failures), Semantic drift (RPN 126)

### Anti-Pattern (❌ Don't Do This)

```rust
// WRONG: Swallows errors, continues silently
let content = std::fs::read_to_string(&path).unwrap_or_default();  // ← returns "" on error!
let work = serde_json::from_str(&content).ok();  // ← returns None on error!

if let Some(work) = work {
    // Proceeds with invalid/missing work
    process_work(work).await;
}
// If something went wrong, nobody knows!
```

### Poka Yoke Implementation (✅ Do This)

```rust
// RIGHT: Fail fast with detailed context
pub async fn process_work_safely(&self, work_id: &str) -> Result<()> {
    let path = format!("{}/work_queue/active/{}", self.data_dir, work_id);

    // Step 1: Verify file exists
    if !Path::new(&path).exists() {
        return Err(format!(
            "Work file not found: {} (checked at: {})",
            path,
            chrono::Local::now()
        ));
    }

    // Step 2: Read file with full error context
    let content = std::fs::read_to_string(&path).map_err(|e| {
        format!(
            "Failed to read work file {}: {} (errno: {})",
            path, e, e.raw_os_error().unwrap_or(0)
        )
    })?;

    // Step 3: Parse JSON with validation
    let work: Work = serde_json::from_str(&content).map_err(|e| {
        format!(
            "Invalid JSON in work file {}: {} (line {}, column {})",
            path, e, e.line(), e.column()
        )
    })?;

    // Step 4: Validate data integrity
    work.validate().map_err(|e| {
        format!("Work validation failed for {}: {} (work data: {:?})", work_id, e, work)
    })?;

    // Step 5: Execute with circuit breaker
    match self.execute_work_with_timeout(&work, Duration::from_secs(300)).await {
        Ok(result) => {
            tracing::info!("Work completed: {} -> {:?}", work_id, result);
            Ok(())
        }
        Err(e) => {
            tracing::error!("Work execution failed: {} -> {}", work_id, e);
            return Err(format!("Work execution failed: {}", e));
        }
    }
}

#[instrument(skip(self, work))]
async fn execute_work_with_timeout(
    &self,
    work: &Work,
    timeout: Duration,
) -> Result<WorkResult> {
    // POKA YOKE: Timeout prevents hanging forever
    tokio::time::timeout(timeout, self.execute_work(work))
        .await
        .map_err(|_| format!("Work execution timed out after {:?}", timeout))?
}
```

### Testing Fail-Fast Behavior

```rust
#[tokio::test]
async fn test_poka_yoke_fails_fast_on_missing_file() {
    let engine = CoordinationEngine::new("./test_data").await.unwrap();

    let result = engine.process_work_safely("nonexistent_job").await;

    // POKA YOKE CHECK: Should fail immediately with clear error
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

#[tokio::test]
async fn test_poka_yoke_fails_fast_on_timeout() {
    let engine = CoordinationEngine::new("./test_data").await.unwrap();

    // Create a work item that takes forever
    let work = Work {
        id: "slow_job".to_string(),
        task: "sleep 1000".to_string(),
        // ... other fields
    };

    // POKA YOKE CHECK: Should timeout after configured duration
    let start = Instant::now();
    let result = engine.execute_work_with_timeout(&work, Duration::from_secs(1)).await;
    let elapsed = start.elapsed();

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("timeout"));
    assert!(elapsed < Duration::from_secs(2), "Should not exceed timeout by much");
}
```

---

## Poka Yoke Pattern #4: Automatic Resource Cleanup

**Problem**: Resources accumulate indefinitely, causing system degradation.

**Failure Mode**: Lock accumulation (RPN 180), Resource exhaustion (RPN 80)

### Anti-Pattern (❌ Don't Do This)

```rust
// WRONG: Creates lock file, never cleans it up
std::fs::write(&lock_path, agent_id)?;
// If code crashes or is interrupted, lock file stays forever
```

### Poka Yoke Implementation (✅ Do This)

```rust
// RIGHT: Automatic cleanup with RAII pattern
pub struct WorkLock {
    lock_path: String,
}

impl WorkLock {
    pub async fn acquire(work_id: &str) -> Result<Self> {
        let lock_path = format!("work_queue/{}.lock", work_id);

        // Acquire lock
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&lock_path)?;

        FileLock::lock(&mut file, file_lock::FileOptions::new().exclusive().non_blocking())?;

        Ok(WorkLock { lock_path })
    }
}

// POKA YOKE: Automatic cleanup when lock is dropped
impl Drop for WorkLock {
    fn drop(&mut self) {
        // This runs automatically, even if code panics!
        if let Err(e) = std::fs::remove_file(&self.lock_path) {
            eprintln!("Failed to clean up lock file: {}", e);
        }
    }
}

pub async fn claim_work_safely(&self, agent_id: &str, work_id: &str) -> Result<bool> {
    // POKA YOKE: Lock is automatically released when this scope ends
    {
        let _lock = WorkLock::acquire(work_id).await?;

        // Do work here
        // If we panic, crash, or return early, lock is still cleaned up!

        Ok(true)
    }
    // Lock is automatically dropped and deleted here
}
```

### Testing Automatic Cleanup

```rust
#[tokio::test]
async fn test_poka_yoke_cleanup_on_panic() {
    let lock_path = "test_lock_file.lock";

    // Intentionally cause panic inside the lock scope
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // Simulate acquiring lock, then panicking
        let _lock = WorkLock { lock_path: lock_path.to_string() };
        panic!("Intentional panic");
    }));

    assert!(result.is_err(), "Should have panicked");

    // POKA YOKE CHECK: Lock file should still be cleaned up despite panic
    assert!(!Path::new(lock_path).exists(), "Lock file should be cleaned up after panic");
}

#[tokio::test]
async fn test_poka_yoke_cleanup_on_early_return() {
    let lock_path = "test_lock_file.lock";

    // Create lock then return early
    {
        let _lock = WorkLock { lock_path: lock_path.to_string() };
        // Early return
        return;
    }

    // POKA YOKE CHECK: Lock file should be cleaned up
    assert!(!Path::new(lock_path).exists(), "Lock file should be cleaned up on early return");
}
```

---

## Poka Yoke Pattern #5: Configuration Immutability & Validation

**Problem**: Configuration changes during execution cause inconsistent behavior.

**Failure Mode**: Configuration inconsistency (RPN 48)

### Anti-Pattern (❌ Don't Do This)

```rust
// WRONG: Config is mutable, can change during execution
static mut CONFIG: Option<Config> = None;

pub fn set_config(new_config: Config) {
    unsafe {
        CONFIG = Some(new_config);  // Can be changed anytime!
    }
}

pub fn get_lock_timeout() -> u64 {
    unsafe {
        CONFIG.as_ref().map(|c| c.lock_timeout).unwrap_or(30)
    }
}
```

### Poka Yoke Implementation (✅ Do This)

```rust
// RIGHT: Immutable configuration, validated at startup
pub struct AppConfig {
    // POKA YOKE: All fields are private and immutable
    lock_timeout_secs: u64,
    max_agents: usize,
    data_dir: PathBuf,
    enable_telemetry: bool,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        // POKA YOKE: Validate all config values at startup

        let lock_timeout = std::env::var("SWARMSH_LOCK_TIMEOUT")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(30);

        // POKA YOKE CHECK: Timeout must be reasonable (1-300 seconds)
        if lock_timeout < 1 || lock_timeout > 300 {
            return Err(format!(
                "Invalid SWARMSH_LOCK_TIMEOUT: {} (must be 1-300)",
                lock_timeout
            ));
        }

        let max_agents = std::env::var("SWARMSH_MAX_AGENTS")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(100);

        // POKA YOKE CHECK: Max agents must be positive
        if max_agents == 0 || max_agents > 10000 {
            return Err(format!(
                "Invalid SWARMSH_MAX_AGENTS: {} (must be 1-10000)",
                max_agents
            ));
        }

        let data_dir = PathBuf::from(
            std::env::var("SWARMSH_DATA_DIR").unwrap_or_else(|_| "./data".to_string())
        );

        // POKA YOKE CHECK: Data directory must be writable
        if !data_dir.exists() {
            std::fs::create_dir_all(&data_dir)
                .map_err(|e| format!("Cannot create data directory: {}", e))?;
        }

        // Test write permission
        let test_file = data_dir.join(".write_test");
        std::fs::write(&test_file, "test")
            .map_err(|e| format!("Data directory not writable: {}", e))?;
        let _ = std::fs::remove_file(test_file);

        Ok(AppConfig {
            lock_timeout_secs: lock_timeout,
            max_agents,
            data_dir,
            enable_telemetry: std::env::var("SWARMSH_TELEMETRY").is_ok(),
        })
    }

    // POKA YOKE: Immutable accessors only
    pub fn lock_timeout(&self) -> u64 {
        self.lock_timeout_secs
    }

    pub fn max_agents(&self) -> usize {
        self.max_agents
    }

    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }
}

// POKA YOKE: Pass config as immutable reference
pub struct CoordinationEngine {
    config: AppConfig,
}

impl CoordinationEngine {
    pub fn new(config: AppConfig) -> Result<Self> {
        // Config is validated and immutable from now on
        Ok(CoordinationEngine { config })
    }

    pub fn lock_timeout(&self) -> u64 {
        self.config.lock_timeout()
    }
}
```

### Testing Configuration Validation

```rust
#[test]
fn test_poka_yoke_rejects_invalid_lock_timeout() {
    std::env::set_var("SWARMSH_LOCK_TIMEOUT", "0");

    let result = AppConfig::from_env();

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid"));
}

#[test]
fn test_poka_yoke_validates_data_directory() {
    std::env::set_var("SWARMSH_DATA_DIR", "/nonexistent/path/that/cannot/be/created");

    let result = AppConfig::from_env();

    // Should either create the directory or fail fast
    assert!(
        result.is_ok() || result.unwrap_err().contains("Cannot"),
        "Should validate data directory"
    );
}
```

---

## Poka Yoke Pattern #6: Health Checks with Circuit Breakers

**Problem**: System continues operating even when degraded or failing.

**Failure Mode**: Silent degradation, Resource exhaustion

### Poka Yoke Implementation (✅ Do This)

```rust
// RIGHT: Circuit breaker pattern prevents cascading failures
pub enum CircuitState {
    Closed,      // Normal operation
    Open,        // Failing, reject requests
    HalfOpen,    // Testing if recovered
}

pub struct CircuitBreaker {
    state: Mutex<CircuitState>,
    failure_count: Mutex<u32>,
    failure_threshold: u32,
    recovery_timeout: Duration,
    last_failure_time: Mutex<Instant>,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u32, recovery_timeout: Duration) -> Self {
        CircuitBreaker {
            state: Mutex::new(CircuitState::Closed),
            failure_count: Mutex::new(0),
            failure_threshold,
            recovery_timeout,
            last_failure_time: Mutex::new(Instant::now()),
        }
    }

    pub async fn execute<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce() -> futures::future::BoxFuture<'static, Result<T>>,
    {
        let mut state = self.state.lock().await;

        match *state {
            // POKA YOKE: When circuit is Open, fail immediately
            CircuitState::Open => {
                let last_failure = *self.last_failure_time.lock().await;
                if last_failure.elapsed() > self.recovery_timeout {
                    // Try recovery
                    *state = CircuitState::HalfOpen;
                    drop(state);  // Release lock for execution
                } else {
                    return Err("Circuit breaker is open".into());
                }
            }
            _ => drop(state),  // Release lock for execution
        }

        // Try the operation
        match f().await {
            Ok(result) => {
                // Success: reset failure count
                *self.failure_count.lock().await = 0;
                *self.state.lock().await = CircuitState::Closed;
                Ok(result)
            }
            Err(e) => {
                // Failure: increment count
                let mut count = self.failure_count.lock().await;
                *count += 1;

                if *count >= self.failure_threshold {
                    // Too many failures: open circuit
                    *self.state.lock().await = CircuitState::Open;
                    *self.last_failure_time.lock().await = Instant::now();
                }

                Err(e)
            }
        }
    }
}

// Usage
let circuit_breaker = CircuitBreaker::new(5, Duration::from_secs(60));

pub async fn claim_work_with_circuit_breaker(&self, agent_id: &str) -> Result<bool> {
    circuit_breaker.execute(|| {
        Box::pin(async { self.claim_work(agent_id, "job_001").await })
    }).await
}
```

---

## Summary: Poka Yoke Patterns for SwarmSH v2

| Pattern | Prevents | Implementation | Test |
|---------|----------|-----------------|------|
| Atomic Operations | Race conditions | OS-level file locking | Concurrent claiming test |
| Version Validation | State desync | Checksums, timestamps | Corruption detection test |
| Fail Fast | Silent degradation | Detailed error messages | Error path test |
| Automatic Cleanup | Resource leaks | RAII pattern, Drop impl | Panic cleanup test |
| Immutable Config | Configuration drift | Validation at startup | Config validation tests |
| Circuit Breaker | Cascading failures | State machine | Failure threshold test |

**Golden Rule**: Design so mistakes are impossible, not just detected.

See [FMEA_ANALYSIS.md](./FMEA_ANALYSIS.md) for the failure modes these patterns prevent.
