//! Performance benchmarks for SwarmSH v2 Worktree Operations
//!
//! Comprehensive benchmarks testing worktree lifecycle operations,
//! coordination patterns, and telemetry overhead.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use swarmsh_v2::{
    WorktreeManager, WorktreeSpec, CoordinationPattern, TelemetryManager,
    AgentCoordinator, WorkQueue, AgentSpec
};
use tempfile::TempDir;
use tokio::runtime::Runtime;

/// Benchmark setup fixture
struct WorktreeBenchFixture {
    temp_dir: TempDir,
    manager: WorktreeManager,
    telemetry: Arc<TelemetryManager>,
    coordinator: Arc<AgentCoordinator>,
    work_queue: Arc<WorkQueue>,
}

impl WorktreeBenchFixture {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        
        // Initialize git repository
        std::process::Command::new("git")
            .args(&["init"])
            .current_dir(temp_dir.path())
            .output()?;
            
        std::process::Command::new("git")
            .args(&["config", "user.email", "bench@swarmsh.dev"])
            .current_dir(temp_dir.path())
            .output()?;
            
        std::process::Command::new("git")
            .args(&["config", "user.name", "SwarmSH Benchmark"])
            .current_dir(temp_dir.path())
            .output()?;
        
        // Create initial commit
        tokio::fs::write(temp_dir.path().join("README.md"), "# SwarmSH Benchmark").await?;
        std::process::Command::new("git")
            .args(&["add", "README.md"])
            .current_dir(temp_dir.path())
            .output()?;
            
        std::process::Command::new("git")
            .args(&["commit", "-m", "Initial commit"])
            .current_dir(temp_dir.path())
            .output()?;
        
        std::env::set_current_dir(temp_dir.path())?;
        
        let telemetry = Arc::new(TelemetryManager::new().await?);
        let work_queue = Arc::new(WorkQueue::new(None).await?);
        let coordinator = Arc::new(AgentCoordinator::new(telemetry.clone(), work_queue.clone()).await?);
        let manager = WorktreeManager::new(
            temp_dir.path().join("worktrees"),
            telemetry.clone()
        ).await?;
        
        Ok(Self {
            temp_dir,
            manager,
            telemetry,
            coordinator,
            work_queue,
        })
    }
}

/// Benchmark worktree creation performance
fn bench_worktree_creation(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let fixture = rt.block_on(WorktreeBenchFixture::new()).unwrap();
    
    let mut group = c.benchmark_group("worktree_creation");
    group.sample_size(50); // Smaller sample size for expensive operations
    
    // Benchmark different coordination patterns
    let patterns = vec![
        CoordinationPattern::Atomic,
        CoordinationPattern::ScrumAtScale,
        CoordinationPattern::RobertsRules,
        CoordinationPattern::Realtime,
    ];
    
    for pattern in patterns {
        group.bench_with_input(
            BenchmarkId::new("create_worktree", format!("{:?}", pattern)),
            &pattern,
            |b, pattern| {
                let mut counter = 0;
                b.iter(|| {
                    counter += 1;
                    let spec = WorktreeSpec {
                        name: format!("bench-{}-{}", counter, SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()),
                        branch: Some("main".to_string()),
                        base_branch: None,
                        coordination_pattern: pattern.clone(),
                        agent_assignments: vec![],
                        auto_sync: false,
                        backup_enabled: false,
                    };
                    
                    rt.block_on(async {
                        let result = fixture.manager.create_worktree(spec).await;
                        black_box(result)
                    })
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark worktree operation throughput
fn bench_worktree_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let fixture = rt.block_on(WorktreeBenchFixture::new()).unwrap();
    
    // Pre-create worktrees for operation benchmarks
    let mut worktree_names = Vec::new();
    for i in 0..10 {
        let spec = WorktreeSpec {
            name: format!("ops-bench-{}", i),
            branch: Some("main".to_string()),
            base_branch: None,
            coordination_pattern: CoordinationPattern::Atomic,
            agent_assignments: vec![],
            auto_sync: false,
            backup_enabled: false,
        };
        
        let state = rt.block_on(fixture.manager.create_worktree(spec)).unwrap();
        worktree_names.push(state.name);
    }
    
    let mut group = c.benchmark_group("worktree_operations");
    
    // Benchmark list operations
    group.bench_function("list_worktrees", |b| {
        b.iter(|| {
            rt.block_on(async {
                let result = fixture.manager.list_worktrees().await;
                black_box(result)
            })
        });
    });
    
    // Benchmark status operations
    group.bench_function("get_worktree_status", |b| {
        let mut counter = 0;
        b.iter(|| {
            let name = &worktree_names[counter % worktree_names.len()];
            counter += 1;
            rt.block_on(async {
                let result = fixture.manager.get_worktree(name).await;
                black_box(result)
            })
        });
    });
    
    // Benchmark sync operations
    group.bench_function("sync_worktree", |b| {
        let mut counter = 0;
        b.iter(|| {
            let name = &worktree_names[counter % worktree_names.len()];
            counter += 1;
            rt.block_on(async {
                let result = fixture.manager.sync_worktree(name).await;
                black_box(result)
            })
        });
    });
    
    // Benchmark usage analytics
    group.bench_function("get_worktree_usage", |b| {
        let mut counter = 0;
        b.iter(|| {
            let name = &worktree_names[counter % worktree_names.len()];
            counter += 1;
            rt.block_on(async {
                let result = fixture.manager.get_worktree_usage(name).await;
                black_box(result)
            })
        });
    });
    
    group.finish();
}

/// Benchmark coordination pattern performance
fn bench_coordination_patterns(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let fixture = rt.block_on(WorktreeBenchFixture::new()).unwrap();
    
    // Register test agents
    rt.block_on(async {
        for i in 0..5 {
            let spec = AgentSpec {
                id: format!("bench-agent-{}", i),
                role: "benchmark-role".to_string(),
                capacity: 1.0,
                specializations: vec!["testing".to_string()],
                work_capacity: Some(5),
            };
            fixture.coordinator.register_agent(spec).await.unwrap();
        }
    });
    
    let mut group = c.benchmark_group("coordination_patterns");
    
    let patterns = vec![
        CoordinationPattern::Atomic,
        CoordinationPattern::ScrumAtScale,
        CoordinationPattern::RobertsRules,
        CoordinationPattern::Realtime,
    ];
    
    for pattern in patterns {
        group.bench_with_input(
            BenchmarkId::new("coordinate", format!("{:?}", pattern)),
            &pattern,
            |b, pattern| {
                b.iter(|| {
                    rt.block_on(async {
                        let result = fixture.coordinator.coordinate(pattern.clone()).await;
                        black_box(result)
                    })
                });
            },
        );
    }
    
    // Benchmark worktree coordination
    rt.block_on(async {
        // Create test worktrees for coordination
        for i in 0..3 {
            let spec = WorktreeSpec {
                name: format!("coord-bench-{}", i),
                branch: Some("main".to_string()),
                base_branch: None,
                coordination_pattern: CoordinationPattern::Atomic,
                agent_assignments: vec![],
                auto_sync: false,
                backup_enabled: false,
            };
            fixture.manager.create_worktree(spec).await.unwrap();
        }
    });
    
    for pattern in vec![CoordinationPattern::Atomic, CoordinationPattern::Realtime] {
        group.bench_with_input(
            BenchmarkId::new("worktree_coordinate", format!("{:?}", pattern)),
            &pattern,
            |b, pattern| {
                b.iter(|| {
                    rt.block_on(async {
                        let result = fixture.manager.coordinate_worktrees(pattern.clone()).await;
                        black_box(result)
                    })
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark telemetry overhead
fn bench_telemetry_overhead(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let fixture = rt.block_on(WorktreeBenchFixture::new()).unwrap();
    
    let mut group = c.benchmark_group("telemetry_overhead");
    
    // Benchmark telemetry report generation
    group.bench_function("generate_coordinator_telemetry", |b| {
        b.iter(|| {
            rt.block_on(async {
                let result = fixture.telemetry.generate_report().await;
                black_box(result)
            })
        });
    });
    
    group.bench_function("generate_worktree_telemetry", |b| {
        b.iter(|| {
            rt.block_on(async {
                let result = fixture.manager.generate_telemetry().await;
                black_box(result)
            })
        });
    });
    
    // Benchmark coordination with telemetry enabled vs disabled simulation
    group.bench_function("coordination_with_telemetry", |b| {
        b.iter(|| {
            rt.block_on(async {
                let result = fixture.coordinator.coordinate(CoordinationPattern::Atomic).await;
                black_box(result)
            })
        });
    });
    
    group.finish();
}

/// Benchmark AI integration performance
fn bench_ai_integration(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let fixture = rt.block_on(WorktreeBenchFixture::new()).unwrap();
    
    let mut group = c.benchmark_group("ai_integration");
    group.sample_size(20); // AI operations may be slower
    
    // Benchmark AI recommendations
    group.bench_function("ai_coordination_recommendations", |b| {
        b.iter(|| {
            rt.block_on(async {
                let result = fixture.coordinator.get_ai_recommendations(&CoordinationPattern::Atomic).await;
                black_box(result)
            })
        });
    });
    
    // Benchmark work distribution optimization
    group.bench_function("ai_work_optimization", |b| {
        b.iter(|| {
            rt.block_on(async {
                let result = fixture.coordinator.optimize_work_distribution().await;
                black_box(result)
            })
        });
    });
    
    // Benchmark worktree optimization
    rt.block_on(async {
        let spec = WorktreeSpec {
            name: "ai-bench-worktree".to_string(),
            branch: Some("main".to_string()),
            base_branch: None,
            coordination_pattern: CoordinationPattern::Atomic,
            agent_assignments: vec![],
            auto_sync: false,
            backup_enabled: false,
        };
        fixture.manager.create_worktree(spec).await.unwrap();
    });
    
    group.bench_function("ai_worktree_optimization", |b| {
        b.iter(|| {
            rt.block_on(async {
                let result = fixture.manager.optimize_worktree("ai-bench-worktree").await;
                black_box(result)
            })
        });
    });
    
    group.finish();
}

/// Benchmark concurrent operations
fn bench_concurrent_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let fixture = rt.block_on(WorktreeBenchFixture::new()).unwrap();
    
    let mut group = c.benchmark_group("concurrent_operations");
    group.sample_size(20);
    
    // Benchmark concurrent worktree creation
    group.bench_function("concurrent_worktree_creation", |b| {
        let mut counter = 0;
        b.iter(|| {
            counter += 1;
            rt.block_on(async {
                let handles = (0..3).map(|i| {
                    let manager = fixture.manager.clone();
                    let base_counter = counter;
                    tokio::spawn(async move {
                        let spec = WorktreeSpec {
                            name: format!("concurrent-{}-{}-{}", base_counter, i, SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()),
                            branch: Some("main".to_string()),
                            base_branch: None,
                            coordination_pattern: CoordinationPattern::Atomic,
                            agent_assignments: vec![],
                            auto_sync: false,
                            backup_enabled: false,
                        };
                        manager.create_worktree(spec).await
                    })
                }).collect::<Vec<_>>();
                
                let results = futures::future::join_all(handles).await;
                black_box(results)
            })
        });
    });
    
    // Benchmark concurrent coordination operations
    group.bench_function("concurrent_coordination", |b| {
        b.iter(|| {
            rt.block_on(async {
                let handles = (0..3).map(|_| {
                    let coordinator = fixture.coordinator.clone();
                    tokio::spawn(async move {
                        coordinator.coordinate(CoordinationPattern::Atomic).await
                    })
                }).collect::<Vec<_>>();
                
                let results = futures::future::join_all(handles).await;
                black_box(results)
            })
        });
    });
    
    group.finish();
}

/// Benchmark nanosecond precision operations
fn bench_nanosecond_precision(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let fixture = rt.block_on(WorktreeBenchFixture::new()).unwrap();
    
    let mut group = c.benchmark_group("nanosecond_precision");
    
    // Benchmark coordination epoch generation
    group.bench_function("coordination_epoch_generation", |b| {
        b.iter(|| {
            let epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
            black_box(epoch)
        });
    });
    
    // Benchmark rapid coordination operations for uniqueness
    group.bench_function("rapid_coordination_uniqueness", |b| {
        b.iter(|| {
            rt.block_on(async {
                let mut epochs = Vec::new();
                for _ in 0..10 {
                    let start = SystemTime::now();
                    let _result = fixture.coordinator.coordinate(CoordinationPattern::Atomic).await;
                    let epoch = start.duration_since(UNIX_EPOCH).unwrap().as_nanos();
                    epochs.push(epoch);
                }
                
                // Verify uniqueness
                epochs.sort();
                epochs.dedup();
                black_box(epochs.len() == 10)
            })
        });
    });
    
    group.finish();
}

/// Benchmark memory usage and allocation patterns
fn bench_memory_usage(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let fixture = rt.block_on(WorktreeBenchFixture::new()).unwrap();
    
    let mut group = c.benchmark_group("memory_usage");
    
    // Benchmark memory allocation during worktree operations
    group.bench_function("worktree_memory_allocation", |b| {
        let mut counter = 0;
        b.iter(|| {
            counter += 1;
            rt.block_on(async {
                // Create and immediately clean up to test allocation patterns
                let spec = WorktreeSpec {
                    name: format!("memory-test-{}", counter),
                    branch: Some("main".to_string()),
                    base_branch: None,
                    coordination_pattern: CoordinationPattern::Atomic,
                    agent_assignments: vec![],
                    auto_sync: false,
                    backup_enabled: false,
                };
                
                let result = fixture.manager.create_worktree(spec).await;
                if let Ok(state) = result {
                    let _cleanup = fixture.manager.remove_worktree(&state.name, true).await;
                }
                black_box(())
            })
        });
    });
    
    // Benchmark telemetry data structure allocation
    group.bench_function("telemetry_data_allocation", |b| {
        b.iter(|| {
            rt.block_on(async {
                let telemetry = fixture.manager.generate_telemetry().await;
                black_box(telemetry)
            })
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_worktree_creation,
    bench_worktree_operations,
    bench_coordination_patterns,
    bench_telemetry_overhead,
    bench_ai_integration,
    bench_concurrent_operations,
    bench_nanosecond_precision,
    bench_memory_usage
);

criterion_main!(benches);