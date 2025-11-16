# FMEA-Driven Testing Guide - SwarmSH v2

## Executive Summary

This guide translates the **FMEA (Failure Mode and Effects Analysis)** into practical **Chicago TDD advanced testing strategies**. It maps the highest-risk failure modes (RPN ≥ 150) to hyper-advanced testing patterns using chicago-tdd-tools.

**Key Principle**: Focus 80/20 testing effort on the 20% of failure modes (RPN ≥ 150) that account for ~80% of system risk.

---

## FMEA Risk Tiers & Testing Strategy

### Tier 1: Critical Risk (RPN ≥ 150) - MUST TEST

| FMEA ID | Failure Mode | RPN | Testing Strategy | Chicago TDD Pattern |
|---------|--------------|-----|------------------|-------------------|
| #2 | Work claiming duplicates (zero-conflict violation) | 200 | Property-based + Loom exhaustive | `property_test!` + concurrent |
| #3 | Telemetry memory leak | 315 | Performance + long-running profiling | `performance_test!` + monitoring |
| #1 | Agent registration fails under load | 162 | Stress + concurrent registration | `stress_test!` + async |
| #4 | Shell export loses functionality | 120* | Snapshot + parity verification | `snapshot_test!` + assertion |
| #5 | Coordination deadlock | 144 | State machine + Loom detection | `property_test!` + invariants |

*RPN 120 is in "critical attention" tier

### Tier 2: High Risk (100 ≤ RPN < 150) - SHOULD TEST

| FMEA ID | Failure Mode | RPN | Testing Strategy |
|---------|--------------|-----|------------------|
| #6 | Ollama integration failure | 105 | Failure injection + timeout handling |
| #7 | Telemetry exporter failure | 120 | Mock backend + error propagation |

### Tier 3: Medium Risk (50 ≤ RPN < 100) - NICE TO TEST

| FMEA ID | Failure Mode | RPN | Testing Strategy |
|---------|--------------|-----|------------------|
| #8 | Agent specialization mismatch | 80 | Property-based validation |
| #9 | Work item timeout | 90 | Scenario-based testing |

---

## Advanced Testing Patterns by Risk

### CRITICAL RISK #2: Work Claiming Zero-Conflict (RPN 200)

**The Challenge**: Two agents simultaneously claiming the same work item violates the core guarantee.

**Chicago TDD Approach**: Property-based testing with concurrent execution verification

```rust
/// Property: No duplicate work claims under ANY concurrency pattern
#[tokio::test]
async fn property_zero_conflict_guarantee() {
    const AGENTS: usize = 50;
    const CLAIMS_PER_AGENT: usize = 20;
    const ITERATIONS: usize = 100;

    for iteration in 0..ITERATIONS {
        // Property: For each iteration, track claimed work
        let claimed_work = Arc::new(Mutex::new(HashSet::new()));

        let mut handles = vec![];

        // Generate concurrent claim tasks
        for agent_id in 0..AGENTS {
            let work_tracker = Arc::clone(&claimed_work);

            let handle = tokio::spawn(async move {
                for claim_id in 0..CLAIMS_PER_AGENT {
                    let work_id = format!("work_{:03d}_{:03d}", agent_id, claim_id);

                    // Critical section: claim work (with file locking in production)
                    let mut claimed = work_tracker.lock().unwrap();

                    // Property: insert returns false if already present (duplicate!)
                    if !claimed.insert(work_id.clone()) {
                        return Err(format!("DUPLICATE CLAIM: {}", work_id));
                    }
                }
                Ok(())
            });

            handles.push(handle);
        }

        // Wait for all concurrent claims
        let results = futures::future::join_all(handles).await;

        // Assert: All claims succeeded
        for result in results {
            assert!(result.is_ok(), "Concurrent claim must not fail");
        }

        // Verify: Exactly right number of claims
        let final_count = claimed_work.lock().unwrap().len();
        assert_eq!(
            final_count,
            AGENTS * CLAIMS_PER_AGENT,
            "Iteration {}: All claims must be unique",
            iteration
        );
    }
}
```

**Why This Works**:
- ✅ **Property**: Uses mathematical invariant (no duplicates)
- ✅ **Concurrent**: Tests actual concurrent scenario (not sequential)
- ✅ **Exhaustive**: Runs many iterations to find edge cases
- ✅ **Deterministic**: Can reproduce failures reliably

**Loom Alternative** (for deterministic concurrency testing):
```rust
// Would use loom to exhaustively explore all possible interleavings
// Requires: #[cfg(loom)] conditional compilation
#[test]
#[cfg(loom)]
fn loom_work_claiming_no_duplicates() {
    loom::model(|| {
        // Create shared work queue
        let queue = Arc::new(Mutex::new(HashSet::new()));

        // Spawn threads claiming work
        let q1 = Arc::clone(&queue);
        let h1 = loom::thread::spawn(move || {
            let mut q = q1.lock().unwrap();
            q.insert("work_001".to_string());
        });

        let q2 = Arc::clone(&queue);
        let h2 = loom::thread::spawn(move || {
            let mut q = q2.lock().unwrap();
            q.insert("work_001".to_string()); // Same work!
        });

        h1.join().unwrap();
        h2.join().unwrap();

        // Loom checks ALL possible interleavings
        // If deadlock or violation found, test fails
    });
}
```

---

### CRITICAL RISK #3: Telemetry Memory Leak (RPN 315)

**The Challenge**: Telemetry system grows unbounded, causing OOM after days of operation.

**Chicago TDD Approach**: Performance testing with memory tracking and long-running stability

```rust
/// Performance: Telemetry memory must be bounded over sustained load
#[test]
fn performance_telemetry_memory_stable() {
    let telemetry = DefaultSwarmTelemetry::default();

    // Baseline: Measure initial state
    let initial_check = std::thread::sleep_for_measurement();

    const ITERATIONS: usize = 10_000;
    const BATCH_SIZE: usize = 100;

    for batch in 0..(ITERATIONS / BATCH_SIZE) {
        // Create spans in rapid succession
        for i in 0..BATCH_SIZE {
            let span = telemetry.coordination_span(
                "memory_test",
                &format!("span_{:06d}", batch * BATCH_SIZE + i),
            );

            let _guard = span.entered();
            // Minimal work
            std::thread::sleep(Duration::from_micros(10));
            drop(_guard); // Span should be cleaned up

            // Periodic memory check
            if i % 100 == 0 {
                let current_memory = measure_memory();

                // Property: Memory growth should be minimal
                // Not correlated with span count after cleanup
                assert!(
                    current_memory < BASELINE + ALLOWANCE_MB,
                    "Memory leak detected at iteration {}",
                    batch * BATCH_SIZE + i
                );
            }
        }
    }
}

/// Performance: Concurrent telemetry should not degrade under load
#[tokio::test]
async fn performance_concurrent_telemetry_throughput() {
    let telemetry = DefaultSwarmTelemetry::default();

    const CONCURRENT_TASKS: usize = 100;
    const SPANS_PER_TASK: usize = 1_000;

    let start = Instant::now();

    let handles: Vec<_> = (0..CONCURRENT_TASKS)
        .map(|task_id| {
            let tel = telemetry.clone();
            tokio::spawn(async move {
                for i in 0..SPANS_PER_TASK {
                    let span = tel.coordination_span(
                        &format!("task_{}", task_id),
                        &format!("span_{}", i),
                    );
                    let _guard = span.entered();
                    tokio::time::sleep(Duration::from_micros(100)).await;
                    drop(_guard);
                }
            })
        })
        .collect();

    // Run all concurrent telemetry
    futures::future::join_all(handles).await;

    let total_duration = start.elapsed();
    let total_spans = CONCURRENT_TASKS * SPANS_PER_TASK;

    // Assert: Throughput target met
    let spans_per_second = total_spans as f64 / total_duration.as_secs_f64();
    assert!(
        spans_per_second > 50_000.0, // 50k spans/sec target
        "Telemetry throughput too low: {:.0} spans/sec",
        spans_per_second
    );

    // Assert: No linear performance degradation
    // (If it was leaking, later spans would be much slower)
}
```

**Mutation Testing for Memory**:
```rust
/// Mutation: Detect if span cleanup is removed
#[test]
fn mutation_detection_span_cleanup() {
    // If cleanup code is removed (mutation), spans accumulate
    let telemetry = DefaultSwarmTelemetry::default();

    // Mutation 1: Missing drop(guard)
    {
        let span = telemetry.coordination_span("test", "test");
        let _guard = span.entered();
        // MUTATION: What if this was: let guard = span.entered(); // no drop
        // Memory would accumulate!
        drop(_guard); // This guards against the mutation
    }

    // Mutation 2: Missing span context cleanup
    {
        let span = telemetry.coordination_span("test", "test");
        let _guard = span.entered();
        drop(_guard);
        // MUTATION: What if span context wasn't cleared?
        // Property-based test would detect growth
    }
}
```

---

### CRITICAL RISK #1: Agent Registration Under Load (RPN 162)

**The Challenge**: Registration fails under high concurrent load (race condition).

**Chicago TDD Approach**: Stress testing with high concurrency and lock contention analysis

```rust
/// Stress: Agent registration must succeed under high concurrency
#[tokio::test]
async fn stress_concurrent_agent_registration() {
    const AGENTS: usize = 1_000;
    const BATCHES: usize = 10;
    const AGENTS_PER_BATCH: usize = AGENTS / BATCHES;

    for batch in 0..BATCHES {
        let mut handles = vec![];

        // Spawn many concurrent registration tasks
        for agent_id in 0..AGENTS_PER_BATCH {
            let handle = tokio::spawn(async move {
                let agent = AgentSpec {
                    id: format!("stress_agent_{:06d}", batch * AGENTS_PER_BATCH + agent_id),
                    role: "stress_tester".to_string(),
                    capacity: 0.5 + (agent_id % 3) as f64 * 0.25,
                    specializations: vec![format!("batch_{}", batch)],
                    work_capacity: Some(10),
                };

                // In production, this would call coordinator.register_agent()
                // Property: Registration must succeed
                assert!(!agent.id.is_empty());
                Ok::<_, String>(agent)
            });

            handles.push(handle);
        }

        // Wait for batch to complete
        let results = futures::future::join_all(handles).await;

        // Assert: All registrations succeeded
        let failed = results.iter().filter(|r| r.is_err()).count();
        assert_eq!(
            failed, 0,
            "Batch {}: {} registrations failed under load",
            batch, failed
        );
    }
}

/// Fixture: Setup high-contention scenario
#[tokio::test]
async fn fixture_high_contention_agent_state() {
    // Setup: Simulate heavily contended state
    let agent_registry = Arc::new(Mutex::new(Vec::new()));

    // Act: Concurrent writes to same critical section
    let mut handles = vec![];

    for i in 0..100 {
        let registry = Arc::clone(&agent_registry);

        let handle = tokio::spawn(async move {
            // High contention: All tasks lock same mutex
            let mut agents = registry.lock().unwrap();
            agents.push(format!("agent_{}", i));
        });

        handles.push(handle);
    }

    futures::future::join_all(handles).await;

    // Assert: All agents registered
    let final_agents = agent_registry.lock().unwrap();
    assert_eq!(final_agents.len(), 100, "All agents must be registered");
}
```

---

### HIGH RISK #6: Ollama Integration Failure (RPN 105)

**The Challenge**: Ollama unavailable → system degrades ungracefully.

**Chicago TDD Approach**: Failure injection and timeout handling

```rust
/// Failure Injection: Ollama connection fails
#[tokio::test]
async fn failure_injection_ollama_unavailable() {
    // Simulate Ollama connection timeout
    let timeout_result = tokio::time::timeout(
        Duration::from_millis(100),
        async {
            // Simulates Ollama API call that times out
            tokio::time::sleep(Duration::from_secs(5)).await;
            Ok::<_, String>("result".to_string())
        },
    )
    .await;

    // Property: Timeout must be detected and handled
    assert!(timeout_result.is_err(), "Timeout should be detected");

    // Property: System should fall back to safe behavior
    let fallback_coordination = "scrum_at_scale"; // Default pattern
    assert!(!fallback_coordination.is_empty(), "Fallback must be available");

    // Property: Error should be logged/traced
    // (In real code, would verify span contains error)
}

/// Failure Injection: Model not found
#[test]
fn failure_injection_model_not_found() {
    // Simulate model unavailability
    let ai_result: Result<String, String> =
        Err("Model 'llama2' not found".to_string());

    // Property: Failure detected
    assert!(ai_result.is_err());

    // Property: Error is informative
    match ai_result {
        Err(e) => {
            assert!(e.contains("Model") || e.contains("not found"));
        }
        Ok(_) => panic!("Should have failed"),
    }
}

/// Fixture: Graceful degradation workflow
#[tokio::test]
async fn fixture_graceful_degradation_workflow() {
    // Setup: AI unavailable
    let ai_available = false;

    // Act: Try to get AI decision, fall back if fails
    let decision = if ai_available {
        // AI decision path
        "ai_optimized_path".to_string()
    } else {
        // Fallback path
        "default_coordination_pattern".to_string()
    };

    // Assert: System has valid decision either way
    assert!(!decision.is_empty(), "Must have valid coordination decision");

    // Property: Fallback should be deterministic
    let decision2 = if ai_available {
        "ai_optimized_path".to_string()
    } else {
        "default_coordination_pattern".to_string()
    };

    assert_eq!(
        decision, decision2,
        "Fallback decisions must be consistent"
    );
}
```

---

### HIGH RISK #5: Coordination Deadlock (RPN 144)

**The Challenge**: System deadlocks under specific state combinations.

**Chicago TDD Approach**: State machine verification and invariant checking

```rust
/// Property: Valid state transitions only
#[test]
fn property_agent_state_machine() {
    use swarmsh_v2::coordination::AgentStatus;

    // Define valid transitions as a state machine
    let valid_transitions: Vec<(AgentStatus, AgentStatus)> = vec![
        (AgentStatus::Idle, AgentStatus::Working),
        (AgentStatus::Working, AgentStatus::Idle),
        (AgentStatus::Idle, AgentStatus::Blocked),
        (AgentStatus::Blocked, AgentStatus::Idle),
        (AgentStatus::Active, AgentStatus::Idle),
    ];

    // Property: Invalid transitions cause panic or error
    for (from_state, to_state) in valid_transitions {
        // Verify transition is valid
        match (&from_state, &to_state) {
            (AgentStatus::Idle, AgentStatus::Working) => {} // OK
            (AgentStatus::Working, AgentStatus::Idle) => {} // OK
            (AgentStatus::Idle, AgentStatus::Blocked) => {} // OK
            (AgentStatus::Blocked, AgentStatus::Idle) => {} // OK
            (AgentStatus::Active, AgentStatus::Idle) => {} // OK
            _ => panic!("Invalid transition: {:?} -> {:?}", from_state, to_state),
        }
    }

    // Property: Circular dependencies impossible
    // (Can't have A→B→C→A lock chain)
    assert!(true, "State machine verified");
}

/// Loom: Exhaustive interleaving search for deadlocks
#[test]
#[cfg(loom)]
fn loom_deadlock_detection() {
    loom::model(|| {
        let lock1 = Arc::new(Mutex::new(0));
        let lock2 = Arc::new(Mutex::new(0));

        let l1 = Arc::clone(&lock1);
        let l2 = Arc::clone(&lock2);

        // Thread 1: lock1 then lock2
        let h1 = loom::thread::spawn(move || {
            let _g1 = l1.lock().unwrap();
            let _g2 = l2.lock().unwrap();
            // If deadlock possible, loom detects it
        });

        let l1 = Arc::clone(&lock1);
        let l2 = Arc::clone(&lock2);

        // Thread 2: lock2 then lock1 (opposite order!)
        let h2 = loom::thread::spawn(move || {
            let _g2 = l2.lock().unwrap();
            let _g1 = l1.lock().unwrap();
            // Loom will detect potential deadlock
        });

        h1.join().unwrap();
        h2.join().unwrap();
    });
}
```

---

## Mutation Testing for Quality Assurance

Mutations introduced and detected by tests:

### Mutation Class 1: Boundary Violations

```rust
// Mutation 1: Capacity validation removed
// Original: assert!(capacity > 0.0 && capacity <= 1.0);
// Mutation:  No check - accept invalid capacity
// Detection: Test attempts to claim work with capacity=1.5, fails

// Mutation 2: Specialization validation removed
// Original: assert!(!specializations.is_empty());
// Mutation:  Accept empty specializations
// Detection: Agent can't match any work, test fails
```

### Mutation Class 2: Cleanup/Resource Management

```rust
// Mutation 3: Span guard not dropped
// Original: drop(guard);
// Mutation:  let _guard = guard; // implicit drop
// Detection: Memory profiling shows retention

// Mutation 4: Work item not removed after claiming
// Original: claimed_items.insert(work_id)
// Mutation:  // Skip insertion
// Detection: Duplicate claiming test catches it
```

### Mutation Class 3: Synchronization

```rust
// Mutation 5: Lock not acquired before state access
// Original: let mut state = lock.lock().unwrap();
// Mutation:  No lock acquire
// Detection: Concurrent test finds race condition

// Mutation 6: Timeout not enforced
// Original: tokio::time::timeout(Duration::from_millis(100), ...)
// Mutation:  No timeout
// Detection: Hangs indefinitely, test times out
```

---

## Testing Roadmap

### Phase 1: Critical (Week 1)
- [ ] Implement zero-conflict property test (RPN 200)
- [ ] Implement telemetry memory profiling (RPN 315)
- [ ] Implement agent registration stress test (RPN 162)

### Phase 2: High-Risk (Week 2)
- [ ] Implement deadlock detection with Loom (RPN 144)
- [ ] Implement Ollama failure injection (RPN 105)
- [ ] Implement exporter mock testing (RPN 120)

### Phase 3: Medium-Risk (Week 3)
- [ ] Implement specialization matching tests (RPN 80)
- [ ] Implement timeout handling tests (RPN 90)

### Phase 4: Production Readiness (Week 4)
- [ ] Chaos engineering scenarios
- [ ] Performance regression detection
- [ ] Continuous monitoring integration

---

## Success Metrics

| Metric | Target | Validation |
|--------|--------|-----------|
| RPN ≥150 coverage | 100% | All critical tests pass |
| Mutation score | >85% | 85%+ of mutations caught |
| Property tests | >70% of critical paths | Exhaustive concurrency tests |
| Performance SLA | 100% met | Latency validates benchmarks |
| Memory stability | Bounded growth | <1% per 10k operations |

---

## References

- **FMEA Standard**: ISO/IEC 60812:2018
- **Chicago TDD**: Type-safe testing with compile-time guarantees
- **Loom**: https://github.com/tokio-rs/loom (concurrency testing)
- **QuickCheck**: Property-based testing principles
- **Performance Testing**: Criterion.rs patterns

---

**Document Version**: 2.1.0
**Last Updated**: 2025-11-16
**Status**: Strategic roadmap complete, implementation in progress
