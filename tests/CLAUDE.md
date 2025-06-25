# SwarmSH v2 Testing Framework

This directory contains comprehensive tests for SwarmSH v2's agent coordination system.

## Test Architecture

### Test Categories
- **Unit Tests**: Individual module functionality (`cargo test --lib`)
- **Integration Tests**: Multi-component coordination behavior
- **Shell Export Tests**: Validate Rust → Shell conversion
- **Coordination Tests**: Zero-conflict guarantee validation
- **Performance Tests**: Benchmark coordination efficiency

### Key Test Files
- `coordination_tests.rs` - Core coordination behavior validation
- `shell_export_tests.rs` - Template generation and shell functionality
- `zero_conflict_tests.rs` - Mathematical guarantee verification
- `performance_tests.rs` - Latency and throughput benchmarks

## Testing Guidelines

### Critical Test Requirements
- **Zero-Conflict Validation**: Mathematical proof of conflict-free operation
- **Nanosecond Precision**: Timestamp accuracy verification
- **Shell Export Parity**: Rust and shell behavior must match exactly
- **OTEL Telemetry**: Verify telemetry data quality and completeness

### Coordination Testing Patterns
```rust
#[tokio::test]
async fn test_zero_conflict_work_claiming() {
    // Setup multiple agents
    let agents = setup_test_agents(5).await;
    
    // Create work items
    let work_items = create_test_work(100);
    
    // Parallel work claiming
    let handles: Vec<_> = agents.into_iter().map(|agent| {
        tokio::spawn(async move {
            claim_work_parallel(agent, &work_items).await
        })
    }).collect();
    
    // Verify zero conflicts
    let results = join_all(handles).await;
    assert_no_work_conflicts(&results);
    assert_all_work_claimed(&work_items);
}
```

### Shell Export Testing
```rust
#[test]
fn test_shell_export_functionality() {
    // Generate shell script from template
    let shell_script = generate_shell_script("coordination_helper.sh.tera")?;
    
    // Validate shell syntax
    assert!(validate_shell_syntax(&shell_script));
    
    // Test coordination behavior
    let coordination_result = test_shell_coordination(&shell_script)?;
    assert_eq!(coordination_result.conflicts, 0);
    assert!(coordination_result.nanosecond_precision);
}
```

### Performance Testing
```rust
#[tokio::test]  
async fn benchmark_agent_coordination() {
    let start = Instant::now();
    
    // Coordinate 1000 agents with 10000 work items
    let result = coordinate_agents(1000, 10000).await?;
    
    let duration = start.elapsed();
    
    // Performance targets
    assert!(duration < Duration::from_secs(30)); // <30s for 1000 agents
    assert_eq!(result.conflicts, 0); // Zero conflicts
    assert!(result.efficiency > 0.84); // >84% efficiency (DLSS target)
}
```

## Test Data & Fixtures

### Agent Test Configurations
```rust
fn create_test_agent_config(id: u64) -> AgentConfig {
    AgentConfig {
        id: format!("test_agent_{}", id),
        coordination_enabled: true,
        health_monitoring: true,
        work_claiming_strategy: WorkClaimingStrategy::PullBased,
        nanosecond_precision: true,
    }
}
```

### Work Item Generation
```rust
fn generate_test_work_items(count: usize) -> Vec<WorkItem> {
    (0..count).map(|i| WorkItem {
        id: format!("work_{}", i),
        payload: format!("test_payload_{}", i),
        priority: Priority::Normal,
        created_at: SystemTime::now(),
    }).collect()
}
```

## Validation Requirements

### Zero-Conflict Guarantees
- Multiple agents cannot claim the same work item
- File operations are atomic and use advisory locking
- Nanosecond-precision timestamps prevent race conditions
- Work state transitions are deterministic

### Shell Export Validation
- Generated shell scripts pass shellcheck
- Shell coordination matches Rust behavior exactly
- No external dependencies required
- UNIX compatibility across different shells

### OTEL Telemetry Testing
```rust
#[test]
fn test_telemetry_completeness() {
    let telemetry_data = collect_test_telemetry();
    
    // Verify 99.2% observability coverage
    assert!(telemetry_data.coverage > 0.992);
    
    // Check semantic convention compliance
    assert_valid_otel_spans(&telemetry_data.spans);
    assert_valid_otel_metrics(&telemetry_data.metrics);
}
```

## Test Commands

### Running Tests
```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests
cargo test --test coordination_tests
cargo test --test shell_export_tests

# Performance benchmarks
cargo test --release benchmark

# Shell export validation
./dev.sh test-templates
make test-shell-export
```

### Continuous Integration
- All tests must pass before merge
- Performance benchmarks tracked for regression
- Shell export functionality validated on multiple UNIX systems
- Zero-conflict guarantees verified with stress testing

## Mock and Test Utilities

### Test Helpers
```rust
// Coordination test utilities
pub fn setup_test_workspace() -> TempDir { /* ... */ }
pub fn create_test_agents(count: usize) -> Vec<Agent> { /* ... */ }
pub fn assert_no_conflicts(results: &[CoordinationResult]) { /* ... */ }

// Shell export test utilities  
pub fn generate_test_shell_script(template: &str) -> String { /* ... */ }
pub fn validate_shell_syntax(script: &str) -> bool { /* ... */ }
pub fn test_shell_coordination(script: &str) -> CoordinationResult { /* ... */ }
```

## Success Criteria
- ✅ Zero coordination conflicts in all test scenarios
- ✅ Nanosecond precision maintained under load
- ✅ Shell export matches Rust behavior exactly
- ✅ 99.2% observability coverage achieved
- ✅ Performance targets met (84% efficiency)
- ✅ OTEL semantic conventions validated

Remember: Tests must prove mathematical zero-conflict guarantees and shell export parity!
