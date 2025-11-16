# SwarmSH v2 - Troubleshooting Guide

**Common issues, diagnosis procedures, and solutions.**

---

## Quick Reference Table

| Symptom | Root Cause | Solution | Time |
|---------|-----------|----------|------|
| Binaries won't compile | Import errors in bin/* | Use core library instead | 2 min |
| No OTEL spans | Collector not running | Start Jaeger, check endpoint | 5 min |
| Disk filling up | Lock files accumulating | Run cleanup script | 5 min |
| Agents stuck | Deadlock in file locking | Restart agents, check logs | 15 min |
| Work done twice | Race condition in claiming | Check file locking impl | 20 min |
| Slow operations | Lock contention | Monitor with health checks | 10 min |
| Config not applying | Config read before startup | Restart with new env vars | 2 min |
| Tests failing | Stale data from previous runs | Clean `./target` and data dirs | 5 min |

---

## Compilation Issues

### Problem: "binary 'X' failed to compile"

```
error[E0433]: cannot find function `from_yaml` in module `serde_yaml`
error[E0599]: no method named `with_debug_flag` found
```

**Root Cause**: Binary implementations reference non-existent imports or methods

**Diagnosis**:
```bash
# Check which binaries fail
cargo build 2>&1 | grep "error\[" | head -10

# Try building just the library
cargo build --lib
# If this works, the issue is in bin/* files
```

**Solution**:

Option A: Use the library (Recommended)
```bash
# Build just the working part
cargo build --lib

# Use in your code
cargo run --example basic_coordination
```

Option B: Fix the binary
```bash
# Open the failing binary
vim src/bin/failing_binary.rs

# Comment out problematic lines or fix imports
# Then try again
cargo build --bin failing_binary
```

**Prevention**: Check with `cargo check` before committing:
```bash
cargo check --lib  # Always passes
cargo check        # Will show issues early
```

---

## Runtime Issues

### Problem: "No agents found" or empty agent list

**Symptoms**:
```bash
$ cargo run --example list_agents
No agents registered
```

**Root Cause**: Either agents were never registered or data directory is wrong

**Diagnosis**:
```bash
# Step 1: Check if coordination data exists
ls -la ./swarmsh_data/agents/
# Should show: agent_*.json files

# Step 2: Verify SWARMSH_DATA_DIR
echo "Current data dir: ${SWARMSH_DATA_DIR:-.}/swarmsh_data"

# Step 3: Check if data is in wrong location
find . -path ./target -prune -o -name "agent_*.json" -print

# Step 4: Look at actual file contents
cat ./swarmsh_data/agents/agent_001.json | jq .
```

**Solution**:

If agents exist but aren't found:
```bash
# Verify data directory matches
export SWARMSH_DATA_DIR=./swarmsh_data
cargo run --example list_agents
```

If agents don't exist:
```bash
# Create agents first
cargo run --example basic_coordination
```

If using wrong data directory:
```bash
# Move data to correct location
mv /old/location/swarmsh_data ./swarmsh_data
export SWARMSH_DATA_DIR=./swarmsh_data
```

---

### Problem: "Failed to acquire lock"

**Symptoms**:
```
Error: Failed to acquire lock on work_queue/claimed/job_001.lock
Multiple agents claim same work
Duplicate work execution in logs
```

**Root Cause**: Either:
1. File locking not working on your filesystem (NFS issue)
2. Multiple agents trying to claim same work simultaneously
3. Old lock files not cleaned up

**Diagnosis**:
```bash
# Step 1: Check current lock situation
ls -la ./swarmsh_data/work_queue/claimed/
# Old locks should be sparse, not hundreds of files

# Step 2: Test atomic operations
flock -n -x 200 201>/tmp/test.lock || echo "Locking failed"
rm /tmp/test.lock 2>/dev/null

# Step 3: Check filesystem type
df -T ./swarmsh_data
# NFS or network mounts are problematic

# Step 4: Monitor for race conditions in logs
grep "claim_work" application.log | grep -o 'work_id="[^"]*"' | sort | uniq -c | awk '$1 > 1'
```

**Solution for filesystem issues**:

If using NFS:
```bash
# Option 1: Use local storage instead
mkdir -p /var/swarmsh
export SWARMSH_DATA_DIR=/var/swarmsh

# Option 2: Enable NFS sync options
# In /etc/fstab:
# nfs_server:/export /mnt/swarmsh nfs noac,hard,intr 0 0
#                                       ^^^^ <- Disable caching
```

If old locks accumulated:
```bash
# Clean old locks
find ./swarmsh_data -name "*.lock" -mtime +1 -delete

# See FMEA_RISK_MITIGATION.md for permanent fix
```

**Prevention**: See [POKA_YOKE_GUIDE.md](./POKA_YOKE_GUIDE.md#poka-yoke-pattern-1-atomic-operations)

---

### Problem: "No space left on device"

**Symptoms**:
```
Error: No space left on device
Disk usage: 100%
Work coordination halts
```

**Root Cause**: Lock files or state files accumulating without cleanup

**Diagnosis**:
```bash
# Step 1: Check disk usage
df -h ./swarmsh_data
du -sh ./swarmsh_data

# Step 2: Find largest directories
du -sh ./swarmsh_data/work_queue/*/

# Step 3: Count files by type
find ./swarmsh_data -type f | wc -l
find ./swarmsh_data -name "*.lock" | wc -l
find ./swarmsh_data -name "*.json" | wc -l

# Step 4: Identify oldest files
find ./swarmsh_data -type f -printf '%T+ %p\n' | sort | head -20
```

**Solution**:

Immediate cleanup:
```bash
# EMERGENCY: Remove locks older than 24 hours
find ./swarmsh_data -name "*.lock" -mtime +1 -delete

# Verify disk freed
df -h ./swarmsh_data

# List remaining files
du -sh ./swarmsh_data/
```

Permanent fix: See [FMEA_RISK_MITIGATION.md - Lock Accumulation](./FMEA_RISK_MITIGATION.md#lock-accumulation-rpn-180)

**Prevention**:
```bash
# Add to crontab
0 * * * * find /path/to/swarmsh_data -name "*.lock" -mtime +1 -delete
```

---

### Problem: OTEL spans not appearing in Jaeger/Collector

**Symptoms**:
```bash
$ curl http://localhost:16686  # Jaeger web UI works
# But no traces appear
# Or: OTEL_EXPORTER_OTLP_ENDPOINT gives connection error
```

**Root Cause**: Either:
1. OTEL collector not running
2. Wrong endpoint configuration
3. Spans being dropped due to buffer overflow
4. Telemetry disabled

**Diagnosis**:
```bash
# Step 1: Check if collector is reachable
curl -v http://localhost:4317
# Should return 200, not connection refused

# Step 2: Check OTEL config
echo $OTEL_EXPORTER_OTLP_ENDPOINT
# Should be set to collector endpoint

# Step 3: Enable debug logging
export RUST_LOG=debug
cargo run --example basic_coordination 2>&1 | grep -i "telemetry\|span\|trace"

# Step 4: Check for span drops
grep -i "dropped\|buffer full" application.log
```

**Solution for collector not running**:

Start Jaeger locally (development):
```bash
# Docker
docker run -d \
  -p 16686:16686 \
  -p 4317:4317 \
  --name jaeger \
  jaegertracing/all-in-one:latest

# Or docker-compose (see GETTING_STARTED.md)
docker-compose up -d jaeger

# Verify it's running
curl http://localhost:14268/api/traces
# Should return JSON with trace list
```

Configure correct endpoint:
```bash
# For Jaeger running locally
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317

# For cloud provider
export OTEL_EXPORTER_OTLP_ENDPOINT=https://your-otel-endpoint:443

# Check it's working
curl -v $OTEL_EXPORTER_OTLP_ENDPOINT
```

Switch to stdout for debugging:
```bash
# See spans immediately in console
export OTEL_EXPORTER=stdout
export RUST_LOG=info
cargo run --example basic_coordination 2>&1 | grep -i "span"
```

---

### Problem: Tests failing with "file already exists" or permission errors

**Symptoms**:
```
error: failed to write to ... Permission denied
error: ... already exists
thread 'test_xxx' panicked
```

**Root Cause**: Previous test runs left files/directories that weren't cleaned up

**Diagnosis**:
```bash
# Step 1: List test directories
ls -la ./target/debug/deps/test_*.d

# Step 2: Check for leftover data
ls -la ./test_data/ 2>/dev/null || echo "No test_data directory"

# Step 3: Verify directory permissions
ls -la ./swarmsh_data/
stat ./swarmsh_data/
```

**Solution**:

Clean test data:
```bash
# Option 1: Clean and rebuild
cargo clean
cargo test --lib

# Option 2: Just clean test data
rm -rf ./test_data
rm -rf ./swarmsh_data/test*
cargo test --lib

# Option 3: Run individual test with cleanup
cargo test --lib test_xxx -- --test-threads=1
```

Prevent future issues:
```rust
// In test setup
#[tokio::test]
async fn my_test() {
    // Clean up before test
    let _ = std::fs::remove_dir_all("./test_data");
    std::fs::create_dir_all("./test_data").unwrap();

    // ... test code ...

    // Clean up after test
    let _ = std::fs::remove_dir_all("./test_data");
}
```

---

### Problem: "Configuration value not applied"

**Symptoms**:
```bash
$ export SWARMSH_LOCK_TIMEOUT=5
$ cargo run --example basic_coordination
# Still uses default 30 second timeout
```

**Root Cause**: Configuration read before the environment variable was set, or not restarted

**Diagnosis**:
```bash
# Check if env var is set
echo $SWARMSH_LOCK_TIMEOUT

# Check what the application sees
cargo run --example basic_coordination -- --show-config 2>&1 | grep -i timeout

# Check when config is read (should be at startup)
grep -i "config.*init\|reading.*config" application.log
```

**Solution**:

Ensure env var is set before running:
```bash
# Set variable first
export SWARMSH_LOCK_TIMEOUT=5
echo $SWARMSH_LOCK_TIMEOUT  # Verify it's set

# Then run
cargo run --example basic_coordination
```

Restart after changing config:
```bash
# If application is running:
pkill -f swarmsh
sleep 2

# Then restart with new config
export SWARMSH_LOCK_TIMEOUT=5
cargo run --example basic_coordination
```

Create config file for persistence:
```bash
# Create config file
cat > swarmsh.env << 'EOF'
SWARMSH_DATA_DIR=./swarmsh_data
SWARMSH_LOCK_TIMEOUT=30
SWARMSH_MAX_AGENTS=100
RUST_LOG=info
EOF

# Load before running
source swarmsh.env
cargo run --example basic_coordination
```

---

## Performance Issues

### Problem: System running slow, high lock contention

**Symptoms**:
```bash
# Operations taking >100ms when should be <10ms
# CPU usage high even with few agents
# Lots of "waiting for lock" in logs
```

**Diagnosis**:
```bash
# Check health metrics
curl http://localhost:8080/health/metrics | jq .lock_wait_time

# Monitor lock contention
watch -n 1 'ls -1 ./swarmsh_data/work_queue/claimed/*.lock | wc -l'

# Check RUST_LOG for bottleneck alerts
export RUST_LOG=info
cargo run --example basic_coordination 2>&1 | grep -i "bottleneck\|contention"
```

**Solution**: See [FMEA_RISK_MITIGATION.md - Health Check False Positives](./FMEA_RISK_MITIGATION.md#health-check-false-positives-rpn-168)

---

## Debugging Techniques

### Enable Verbose Logging

```bash
# Maximum verbosity
export RUST_LOG=debug
export RUST_BACKTRACE=full

cargo run --example basic_coordination 2>&1 | tee debug.log
```

### Use Strace for System Calls

```bash
# Watch what the application is doing at OS level
strace -e openat,read,write,unlink cargo run --example basic_coordination 2>&1 | grep swarmsh_data
```

### Inspect State Files

```bash
# View agent state
jq . ./swarmsh_data/agents/agent_001.json

# View work state
jq . ./swarmsh_data/work_queue/active/job_001.json

# Check lock files
ls -la ./swarmsh_data/work_queue/claimed/
```

### View OTEL Traces via Curl

```bash
# Query Jaeger HTTP API
curl http://localhost:14268/api/traces | jq '.data[] | {traceID, spans}'

# Or use the web UI
open http://localhost:16686
```

---

## When to Escalate

If you've tried these solutions and still have issues:

1. **Collect information**:
   ```bash
   cargo test --lib 2>&1 | tee test_output.log
   cargo build --lib 2>&1 | tee build_output.log
   ls -la ./swarmsh_data/ | tee file_listing.log
   ```

2. **Check related documentation**:
   - [FMEA_ANALYSIS.md](./FMEA_ANALYSIS.md) - Understand failure modes
   - [FMEA_RISK_MITIGATION.md](./FMEA_RISK_MITIGATION.md) - Detailed mitigation steps
   - [POKA_YOKE_GUIDE.md](./POKA_YOKE_GUIDE.md) - Prevention patterns
   - [README.md](./README.md) - Complete reference

3. **Review logs**:
   ```bash
   # Application logs
   grep -i "error\|warning\|panic" application.log | head -20

   # System logs
   journalctl -u swarmsh -n 50
   ```

---

## Quick Command Reference

```bash
# Diagnosis
cargo build --lib          # Test if core compiles
cargo test --lib           # Run all core tests
cargo check                # Quick syntax check
ps aux | grep swarmsh      # See running processes
ls -lah ./swarmsh_data/    # Inspect data directory

# Cleanup
rm -rf ./swarmsh_data      # Full reset
find ./swarmsh_data -name "*.lock" -delete
cargo clean                # Clean build artifacts

# Monitoring
tail -f application.log    # Watch logs
watch -n 1 'du -sh ./swarmsh_data'  # Watch disk usage

# Testing
cargo test --lib coordination::tests
cargo test --lib telemetry::tests
./test_coordination.sh      # Run full test suite if available
```

---

**Still stuck?** Check the [FAQ section in README.md](./README.md#getting-help) or create a GitHub issue with your logs.
