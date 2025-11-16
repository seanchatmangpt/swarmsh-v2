# SwarmSH v2 - FMEA Risk Mitigation Runbooks

**Operational procedures for responding to failure modes identified in FMEA analysis.**

This document provides step-by-step runbooks for each of the 10 failure modes identified in [FMEA_ANALYSIS.md](./FMEA_ANALYSIS.md). Use these procedures when you detect failure symptoms in production.

---

## Risk Mitigation Overview

| RPN | Failure Mode | Severity | Detection Method | Runbook |
|-----|--------------|----------|------------------|---------|
| 225 | Race condition in work claiming | Critical | Lock timeout, duplicate work | [🔴 Race Conditions](#race-conditions-rpn-225) |
| 192 | Silent OTEL span loss | High | Missing correlation IDs in logs | [🟠 Telemetry Loss](#telemetry-loss-rpn-192) |
| 180 | Lock file accumulation | High | Disk space alerts | [🟠 Lock Cleanup](#lock-accumulation-rpn-180) |
| 168 | Health check false positives | High | Unnecessary alerts, wasted resources | [🟠 Health Checks](#health-false-positives-rpn-168) |
| 140 | Concurrent file corruption | Medium-High | Agent restart loops, data inconsistency | [🟡 File Corruption](#file-corruption-rpn-140) |
| 126 | Shell export semantic drift | Medium-High | Behavior mismatch between Rust and shell | [🟡 Semantic Drift](#semantic-drift-rpn-126) |
| 96 | Agent state desynchronization | Medium | Agent capacity misalignment | [🟡 State Sync](#state-desync-rpn-96) |
| 80 | Health check resource exhaustion | Medium | CPU/memory spikes on health monitors | [🟢 Resource Exhaustion](#resource-exhaustion-rpn-80) |
| 70 | Blocking coordination deadlock | Medium | Agents waiting indefinitely | [🟢 Deadlock](#deadlock-rpn-70) |
| 48 | Config inconsistency across agents | Low-Medium | Coordination failures, agent confusion | [🟢 Config Issues](#config-issues-rpn-48) |

---

## 🔴 Race Conditions (RPN 225)

**Failure**: Two agents claim the same work item simultaneously due to inadequate locking.

**Detection Symptoms**:
- Work item appears in multiple agent logs with same ID
- Lock acquisition fails intermittently
- Duplicate work executions in audit logs
- Error: "Failed to acquire lock on work_queue/claimed/job_*"

### Diagnostic Steps

```bash
# Step 1: Check if race condition is currently occurring
ls -la ./coordination_data/work_queue/claimed/
ls -la ./coordination_data/work_queue/active/

# Step 2: Look for duplicate claims
find ./coordination_data -name "*.lock" -exec cat {} \; | sort | uniq -d

# Step 3: Check agent logs for "claim_work" spans with same work_id
grep "claim_work" agent_*.log | grep -o 'work_id="[^"]*"' | sort | uniq -c | awk '$1 > 1 {print}'

# Step 4: Check for failed lock acquisitions
grep "Failed to acquire lock" agent_*.log | wc -l
```

### Immediate Mitigation (5-10 minutes)

```bash
# STOP: Prevent further claims while investigating
touch ./coordination_data/EMERGENCY_HALT
chmod 444 ./coordination_data/EMERGENCY_HALT  # Read-only to prevent accidental deletion

# VERIFY: Check which agents are still active
ps aux | grep swarmsh

# PAUSE: Send SIGTERM to all agents (graceful shutdown)
pkill -TERM -f "swarmsh"

# WAIT: Give agents 30 seconds to complete current work
sleep 30

# FORCE: Kill remaining agent processes if necessary
pkill -9 -f "swarmsh"
```

### Root Cause Analysis (15-30 minutes)

```rust
// Check if advisory locking is working correctly
// In src/coordination.rs, verify the lock acquisition logic:

pub async fn claim_work(&self, agent_id: &str) -> Result<Option<Work>> {
    // CRITICAL: This must be atomic
    let lock_path = format!("{}/work_queue/claimed/{}.lock", self.data_dir, work_id);

    // 1. Check if lock already exists (another agent has it)
    if Path::new(&lock_path).exists() {
        return Ok(None);  // Work already claimed
    }

    // 2. Try to create lock atomically
    // THIS IS WHERE RACE CONDITIONS OCCUR
    std::fs::write(&lock_path, agent_id)?;  // ← Non-atomic!
}
```

**Root Cause**: File write operations are NOT atomic on all filesystems. NFS, in particular, has known race conditions with file creation.

### Permanent Fix (1-2 hours)

#### Option A: Use proper advisory locking (Recommended)

```rust
use file_lock::FileLock;
use std::fs::OpenOptions;

pub async fn claim_work(&self, agent_id: &str) -> Result<Option<Work>> {
    let lock_path = format!("{}/work_queue/claimed/{}.lock", self.data_dir, work_id);

    // Use proper file locking mechanism
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&lock_path)?;

    // Try non-blocking exclusive lock
    match FileLock::lock(&mut file, file_lock::FileOptions::new().exclusive().non_blocking()) {
        Ok(_) => {
            // We have the lock!
            file.write_all(agent_id.as_bytes())?;
            Ok(Some(work))
        }
        Err(_) => {
            // Another agent has the lock
            Ok(None)
        }
    }
}
```

#### Option B: Use filesystem-level locks (Alternative)

```bash
# In shell scripts, use flock which is properly atomic
{
    flock -n 200 || exit 1  # Acquire exclusive, non-blocking lock on FD 200
    echo "$AGENT_ID" > "$lock_file"
    # Do work here
    rm "$lock_file"  # Release by closing FD
} 200> "$lock_file"
```

### Recovery Procedure (If race condition already occurred)

```bash
# Step 1: Identify duplicated work
dupes=$(find ./coordination_data/work_queue/active -exec grep -l "claim_count > 1" {} \;)

# Step 2: For each duplicated work item, determine canonical owner
for work_file in $dupes; do
    # Get timestamps of claims from audit logs
    grep "claim_work.*$(basename $work_file)" audit.log | sort -k2 | head -1
    # First claim wins
done

# Step 3: Remove duplicate claims
# (Manual review required - do not automate without careful analysis)

# Step 4: Restart agents one at a time, monitoring for issues
systemctl restart swarmsh-agent-1
sleep 5
# Check for errors
systemctl restart swarmsh-agent-2
# ... etc
```

### Prevention for Future Deployments

Add to deployment checklist:
- ✅ Test race condition scenario with 10+ concurrent agents
- ✅ Verify filesystem supports atomic operations (test on target FS)
- ✅ Monitor lock file creation/deletion in CI/CD
- ✅ Set up alerts for "Failed to acquire lock" errors

---

## 🟠 Telemetry Loss (RPN 192)

**Failure**: OTEL spans are not being recorded or exported, causing visibility loss.

**Detection Symptoms**:
- Correlation IDs missing from logs
- OTEL dashboard shows no new spans
- Agent logs contain "Span context lost" messages
- Jaeger shows no traces for recent operations

### Diagnostic Steps

```bash
# Step 1: Verify OTEL exporter is configured
echo $OTEL_EXPORTER_OTLP_ENDPOINT
# Should output something like: http://localhost:4317

# Step 2: Check if exporter is reachable
curl -v $OTEL_EXPORTER_OTLP_ENDPOINT
# Should NOT return connection refused

# Step 3: Check application logs for telemetry errors
grep -i "telemetry\|span\|trace" application.log | head -20

# Step 4: Verify telemetry is enabled
cargo build --lib --features="full-telemetry"

# Step 5: Check span buffer for dropped spans
ls -lah ./telemetry_cache/
du -sh ./telemetry_cache/
```

### Immediate Mitigation (5 minutes)

```bash
# Switch to stdout exporter (emergency mode)
export OTEL_EXPORTER=stdout
export RUST_LOG=info

# Restart application
systemctl restart swarmsh

# Verify spans now appear in stdout
tail -f application.log | grep "span_name"
```

### Root Cause Analysis (15 minutes)

Common causes:
1. **OTEL Collector not running**
   ```bash
   # Check if Jaeger is running
   curl http://localhost:14268/api/traces
   # If 503: Jaeger is starting but not ready
   # If refused: Jaeger isn't running
   ```

2. **Network connectivity issue**
   ```bash
   # From agent machine, test connectivity to collector
   timeout 5 bash -c 'cat < /dev/null > /dev/tcp/collector-host/4317'
   echo $?
   # 0 = connected, 124 = timeout, other = refused
   ```

3. **OTEL buffer overflow**
   ```bash
   # Check for dropped spans in logs
   grep "buffer full\|dropped.*span" application.log
   # Indicates spans are being created but can't be exported
   ```

### Permanent Fix (30-60 minutes)

#### Fix 1: Ensure OTEL Collector is healthy

```yaml
# docker-compose.yml for OTEL development
version: '3.8'
services:
  jaeger:
    image: jaegertracing/all-in-one:latest
    ports:
      - "6831:6831/udp"
      - "4317:4317"  # OTLP gRPC
      - "4318:4318"  # OTLP HTTP
      - "14268:14268" # HTTP collector
      - "16686:16686" # Jaeger UI
    environment:
      - COLLECTOR_OTLP_ENABLED=true
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:14268/api/traces"]
      interval: 10s
      timeout: 5s
      retries: 3
```

#### Fix 2: Configure retry and buffering

```rust
// In telemetry.rs
use opentelemetry_otlp::WithExportConfig;

let exporter = opentelemetry_otlp::new_exporter()
    .http()
    .with_endpoint("http://localhost:4318")
    .with_timeout(std::time::Duration::from_secs(5))
    .build_exporter()?;

let tracer = opentelemetry_sdk::trace::TracerProvider::builder()
    .with_batch_exporter(
        exporter,
        opentelemetry_sdk::runtime::Tokio,
    )
    .with_config(
        opentelemetry_sdk::trace::Config::default()
            .with_max_events_per_span(32)
            .with_max_attributes_per_span(64)
    )
    .build();
```

#### Fix 3: Monitor span dropping

```rust
// Add span drop detection
#[instrument]
pub async fn claim_work(&self, agent_id: &str) -> Result<Option<Work>> {
    let span = tracing::Span::current();

    // Record if span is being sampled
    if !span.is_disabled() {
        tracing::debug!("Span is active and recording");
    } else {
        tracing::warn!("Span is disabled! Telemetry may be lost");
    }

    // ... rest of function
}
```

### Recovery Procedure

```bash
# Step 1: Check if telemetry can be recovered from logs
# (If using file-based logging fallback)
grep "claim_work" fallback_telemetry.log | jq -r .correlation_id | sort | uniq

# Step 2: If Jaeger is recoverable, restart with retention
docker exec jaeger badger restore /badger/db  # Repair corruption if needed

# Step 3: Clear bad span cache
rm -rf ./telemetry_cache/*.bad
rm -rf ./telemetry_cache/*.failed
```

---

## 🟠 Lock Accumulation (RPN 180)

**Failure**: Lock files accumulate and consume disk space because cleanup isn't happening.

**Detection Symptoms**:
- Disk usage growing linearly over time
- `./coordination_data/work_queue/claimed/` contains months of old `.lock` files
- Error: "No space left on device"
- File system 90%+ full

### Diagnostic Steps

```bash
# Step 1: Find oldest lock files
find ./coordination_data -name "*.lock" -type f -printf '%T+ %p\n' | sort | head -20

# Step 2: Count lock files by age
find ./coordination_data -name "*.lock" -type f | while read f; do
    age=$(($(date +%s) - $(stat -f%m "$f")))
    if [ $age -gt 86400 ]; then echo "OLD: $f ($((age/3600))h)"; fi
done

# Step 3: Calculate lock file sizes
du -sh ./coordination_data/work_queue/claimed/
du -sh ./coordination_data/work_queue/*/

# Step 4: Check if lock cleanup is running
crontab -l | grep -i lock
ps aux | grep -i "lock.*clean"
```

### Immediate Mitigation (10 minutes)

```bash
# EMERGENCY: Delete lock files older than 24 hours
find ./coordination_data -name "*.lock" -type f -mtime +1 -delete

# Verify cleanup happened
du -sh ./coordination_data/work_queue/claimed/  # Should be smaller now

# Monitor disk usage
df -h | grep coordination_data
```

### Root Cause Analysis (20 minutes)

**Question 1**: Why aren't old locks being cleaned up?
```bash
# Check if cleanup job exists
ls -la ./scripts/*clean* 2>/dev/null || echo "No cleanup scripts found"

# Check if cron job is configured
crontab -l | grep swarmsh
# Should show: 0 * * * * find ./coordination_data -name "*.lock" -mtime +1 -delete
```

**Question 2**: Why are locks staying around for days?
```rust
// In coordination.rs, check if locks are being released
pub async fn complete_work(&self, agent_id: &str, work_id: &str) -> Result<()> {
    let lock_path = format!("{}/work_queue/claimed/{}.lock", self.data_dir, work_id);

    // If this is missing or not called on completion, locks accumulate!
    std::fs::remove_file(&lock_path)?;  // ← Must be called

    // Move work to completed
    std::fs::rename(
        format!("{}/work_queue/active/{}", self.data_dir, work_id),
        format!("{}/work_queue/completed/{}", self.data_dir, work_id),
    )?;
}
```

### Permanent Fix (1 hour)

#### Solution: Implement automated cleanup

```bash
#!/bin/bash
# /usr/local/bin/swarmsh-cleanup-locks.sh
# Cleanup old lock files from SwarmSH coordination

COORDINATION_DIR="${SWARMSH_DATA_DIR:-.}/coordination_data"
LOCK_MAX_AGE_DAYS=1
ARCHIVE_DIR="$COORDINATION_DIR/archive"

# Create archive directory
mkdir -p "$ARCHIVE_DIR"

# Find and archive old locks
find "$COORDINATION_DIR" \
    -name "*.lock" \
    -type f \
    -mtime +$LOCK_MAX_AGE_DAYS \
    -print0 | while IFS= read -r -d '' lock_file; do

    # Get creation date for archive organization
    creation_date=$(stat -f%SB -t "%Y-%m-%d" "$lock_file")
    archive_date_dir="$ARCHIVE_DIR/$creation_date"

    mkdir -p "$archive_date_dir"

    # Log the archival
    lock_id=$(basename "$lock_file" .lock)
    echo "$(date -u +%Y-%m-%dT%H:%M:%SZ) Archiving old lock: $lock_id" >> "$ARCHIVE_DIR/cleanup.log"

    # Move lock to archive (don't delete in case recovery needed)
    mv "$lock_file" "$archive_date_dir/$(basename "$lock_file")"
done

# Clean up archived locks older than 30 days
find "$ARCHIVE_DIR" -name "*.lock" -mtime +30 -delete

echo "Lock cleanup completed: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
```

Set up cron job:
```bash
# Add to crontab -e
# Run cleanup every hour (adjust frequency as needed)
0 * * * * /usr/local/bin/swarmsh-cleanup-locks.sh

# Or use systemd timer for better logging
```

#### Alternative: Implement automatic cleanup in code

```rust
// In coordination.rs
pub async fn cleanup_old_locks(&self) -> Result<usize> {
    let lock_dir = format!("{}/work_queue/claimed", self.data_dir);
    let max_age = Duration::from_secs(86400);  // 24 hours
    let now = SystemTime::now();

    let mut cleaned = 0;

    for entry in std::fs::read_dir(&lock_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "lock") {
            if let Ok(metadata) = entry.metadata() {
                if let Ok(modified) = metadata.modified() {
                    if now.duration_since(modified)? > max_age {
                        std::fs::remove_file(&path)?;
                        cleaned += 1;
                        tracing::info!("Removed old lock: {:?}", path);
                    }
                }
            }
        }
    }

    Ok(cleaned)
}

// Call periodically
#[tokio::main]
async fn main() {
    let engine = CoordinationEngine::new("./data").await?;

    // Cleanup every 6 hours
    let engine_clone = engine.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(6 * 3600));
        loop {
            interval.tick().await;
            match engine_clone.cleanup_old_locks().await {
                Ok(cleaned) => tracing::info!("Cleaned up {} old locks", cleaned),
                Err(e) => tracing::error!("Cleanup failed: {}", e),
            }
        }
    });
}
```

---

## 🟠 Health Check False Positives (RPN 168)

**Failure**: Health checks incorrectly report system degradation, causing unnecessary alerts and resource waste.

**Detection Symptoms**:
- Frequent "System degraded" alerts that resolve on their own
- Health checks consuming 20%+ CPU
- Bottleneck alerts for temporary spikes
- False alerts 50+ times per day

### Quick Diagnostic

```bash
# Check current health status
curl http://localhost:8080/health/status

# Get detailed bottleneck analysis
curl http://localhost:8080/health/bottlenecks | jq .

# Count false positive rate from logs
grep "System degraded\|RECOVERED" health_check.log | tail -100 | uniq -c
```

### Immediate Mitigation (5 minutes)

```bash
# Reduce health check frequency (temporary)
export HEALTH_CHECK_INTERVAL=60  # seconds (was probably 5-10)

# Increase alert thresholds
export DEGRADATION_THRESHOLD=75  # % (was probably 50)

# Restart with new settings
systemctl restart swarmsh-health-monitor
```

### Root Cause Analysis & Permanent Fix

**See [FMEA_ANALYSIS.md](./FMEA_ANALYSIS.md#health-check-false-positives-rpn-168) for detailed analysis and fixes**

---

## 🟡 File Corruption (RPN 140)

**Failure**: Concurrent writes corrupt agent state or work queue data.

**Recovery**: Run `cargo run --bin validate_core_functionality` to detect corrupted state files.

---

## 🟡 Semantic Drift (RPN 126)

**Failure**: Shell-exported scripts behave differently from Rust implementation.

**Detection**: Run end-to-end tests comparing Rust vs Shell behavior.

```bash
# Compare outputs
cargo run --bin full_cycle_demo > rust_output.json
./generated/coordinator.sh > shell_output.json
diff rust_output.json shell_output.json
```

---

## 🟡 State Desynchronization (RPN 96)

**Failure**: Agent capacity tracking becomes out of sync with actual work distribution.

**Mitigation**: Restart agents and let them re-synchronize from persistent state.

---

## 🟢 Resource Exhaustion (RPN 80)

**Failure**: Health monitoring consumes excessive CPU/memory.

**Fix**: See FMEA_ANALYSIS.md for health check optimization recommendations.

---

## 🟢 Deadlock (RPN 70)

**Failure**: Agents become stuck waiting for each other indefinitely.

**Mitigation**: Implement work timeout and automatic work abandonment.

---

## 🟢 Config Issues (RPN 48)

**Failure**: Configuration inconsistencies across agents.

**Mitigation**: Validate configuration at startup, fail fast if inconsistent.

---

## Summary: When to Use These Runbooks

| Situation | Runbook | Time |
|-----------|---------|------|
| Work being claimed twice | Race Conditions | 10 min |
| Missing telemetry data | Telemetry Loss | 15 min |
| Disk filling up | Lock Accumulation | 10 min |
| Alert fatigue | Health Check False Positives | 20 min |
| Data inconsistencies | File Corruption | 30 min |
| Agents not responding | Deadlock | 25 min |

**Key Principle**: Always diagnose before acting. Never delete data without understanding what's wrong.
