//! Zero-Conflict Coordination Validation Tests
//! 
//! Comprehensive test suite to validate SwarmSH v2's core claims about
//! zero-conflict coordination and mathematical guarantees.

use swarmsh_v2::{
    coordination::{AgentCoordinator, AgentSpec, WorkQueue, WorkItem, CoordinationPattern},
    telemetry::TelemetryManager,
    SwarmSystem, AgentId, WorkId,
};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH, Duration, Instant};
use tokio::time::sleep;
use std::collections::HashSet;
use futures::future::join_all;

/// Test concurrent agent registration for conflicts
#[tokio::test]
async fn test_concurrent_agent_registration_no_conflicts() {
    println!("🧪 Testing concurrent agent registration for conflicts...");
    
    let telemetry = Arc::new(TelemetryManager::new().await.unwrap());
    let work_queue = Arc::new(WorkQueue::new(None).await.unwrap());
    let coordinator = Arc::new(
        AgentCoordinator::new(telemetry.clone(), work_queue).await.unwrap()
    );
    
    let num_agents = 50;
    let mut handles = Vec::new();
    
    // Launch concurrent agent registrations
    for i in 0..num_agents {
        let coord = coordinator.clone();
        let handle = tokio::spawn(async move {
            let agent_id = format!("agent_{}", i);
            let spec = AgentSpec {
                id: agent_id.clone(),
                role: format!("worker_{}", i),
                capacity: 1.0,
                specializations: vec!["test".to_string()],
                work_capacity: Some(10),
            };
            
            coord.register_agent(spec).await
        });
        handles.push(handle);
    }
    
    // Wait for all registrations to complete
    let results = join_all(handles).await;
    
    // Verify all registrations succeeded (zero conflicts)
    let mut success_count = 0;
    for result in results {
        match result {
            Ok(Ok(())) => success_count += 1,
            Ok(Err(e)) => println!("❌ Registration failed: {}", e),
            Err(e) => println!("❌ Task failed: {}", e),
        }
    }
    
    println!("✅ Concurrent registration test: {}/{} succeeded", success_count, num_agents);
    assert_eq!(success_count, num_agents, "All agent registrations should succeed without conflicts");
}

/// Test concurrent work claiming for conflicts
#[tokio::test]
async fn test_concurrent_work_claiming_no_conflicts() {
    println!("🧪 Testing concurrent work claiming for conflicts...");
    
    let telemetry = Arc::new(TelemetryManager::new().await.unwrap());
    let work_queue = Arc::new(WorkQueue::new(None).await.unwrap());
    
    // Add work items to queue
    let num_work_items = 20;
    for i in 0..num_work_items {
        let work = WorkItem {
            id: format!("work_{}", i),
            priority: 1.0,
            requirements: vec!["test".to_string()],
            estimated_duration_ms: 1000,
            created_at: SystemTime::now(),
        };
        work_queue.add_work(work).await.unwrap();
    }
    
    // Create competing agents
    let num_agents = 10;
    let mut handles = Vec::new();
    let claimed_work = Arc::new(tokio::sync::Mutex::new(HashSet::new()));
    
    for i in 0..num_agents {
        let queue = work_queue.clone();
        let claimed = claimed_work.clone();
        
        let handle = tokio::spawn(async move {
            let agent = AgentSpec {
                id: format!("agent_{}", i),
                role: "worker".to_string(),
                capacity: 1.0,
                specializations: vec!["test".to_string()],
                work_capacity: Some(5),
            };
            
            let mut local_claimed = Vec::new();
            
            // Each agent tries to claim work multiple times
            for _attempt in 0..5 {
                if let Ok(Some(work)) = queue.get_work_for_agent(&agent).await {
                    local_claimed.push(work.id.clone());
                    
                    // Check for conflicts
                    let mut claimed_set = claimed.lock().await;
                    if claimed_set.contains(&work.id) {
                        panic!("❌ CONFLICT DETECTED: Work {} claimed by multiple agents", work.id);
                    }
                    claimed_set.insert(work.id);
                }
                
                // Small delay to increase contention
                sleep(Duration::from_millis(1)).await;
            }
            
            local_claimed
        });
        handles.push(handle);
    }
    
    // Wait for all work claiming to complete
    let results = join_all(handles).await;
    
    let mut total_claimed = 0;
    let mut all_claimed_work = HashSet::new();
    
    for result in results {
        match result {
            Ok(claimed_work) => {
                total_claimed += claimed_work.len();
                for work_id in claimed_work {
                    if all_claimed_work.contains(&work_id) {
                        panic!("❌ CONFLICT: Work {} was claimed multiple times", work_id);
                    }
                    all_claimed_work.insert(work_id);
                }
            }
            Err(e) => println!("❌ Agent task failed: {}", e),
        }
    }
    
    println!("✅ Concurrent work claiming test: {} items claimed with zero conflicts", total_claimed);
    assert!(total_claimed <= num_work_items, "Cannot claim more work than available");
    assert_eq!(all_claimed_work.len(), total_claimed, "All claimed work should be unique");
}

/// Test nanosecond precision timing claims
#[tokio::test]
async fn test_nanosecond_precision_timing() {
    println!("🧪 Testing nanosecond precision timing claims...");
    
    let telemetry = Arc::new(TelemetryManager::new().await.unwrap());
    let work_queue = Arc::new(WorkQueue::new(None).await.unwrap());
    let coordinator = Arc::new(
        AgentCoordinator::new(telemetry, work_queue).await.unwrap()
    );
    
    let num_operations = 100;
    let mut timestamps = Vec::new();
    
    // Perform rapid operations and collect timestamps
    for i in 0..num_operations {
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        
        let spec = AgentSpec {
            id: format!("timing_agent_{}", i),
            role: "timing_test".to_string(),
            capacity: 1.0,
            specializations: vec!["timing".to_string()],
            work_capacity: Some(1),
        };
        
        coordinator.register_agent(spec).await.unwrap();
        
        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        timestamps.push((start, end));
    }
    
    // Verify nanosecond precision (all timestamps should be unique)
    let mut start_times: Vec<u128> = timestamps.iter().map(|(start, _)| *start).collect();
    start_times.sort();
    start_times.dedup();
    
    let unique_count = start_times.len();
    let precision_percentage = (unique_count as f64 / num_operations as f64) * 100.0;
    
    println!("✅ Timing precision test: {:.2}% unique nanosecond timestamps", precision_percentage);
    
    // Calculate operation latencies
    let latencies: Vec<u128> = timestamps.iter()
        .map(|(start, end)| end - start)
        .collect();
    
    let avg_latency = latencies.iter().sum::<u128>() / latencies.len() as u128;
    let min_latency = *latencies.iter().min().unwrap();
    let max_latency = *latencies.iter().max().unwrap();
    
    println!("⏱️  Operation latencies (nanoseconds):");
    println!("   Average: {}", avg_latency);
    println!("   Min: {}", min_latency);
    println!("   Max: {}", max_latency);
    
    // Verify nanosecond precision is maintained
    assert!(precision_percentage > 95.0, "Should maintain >95% unique nanosecond timestamps");
    assert!(avg_latency < 1_000_000, "Average operation should complete in <1ms");
}

/// Test coordination patterns work without conflicts
#[tokio::test]
async fn test_coordination_patterns_stability() {
    println!("🧪 Testing coordination patterns for stability...");
    
    let telemetry = Arc::new(TelemetryManager::new().await.unwrap());
    let work_queue = Arc::new(WorkQueue::new(None).await.unwrap());
    let coordinator = Arc::new(
        AgentCoordinator::new(telemetry, work_queue).await.unwrap()
    );
    
    let patterns = vec![
        CoordinationPattern::Atomic,
        CoordinationPattern::Realtime,
        CoordinationPattern::ScrumAtScale,
        CoordinationPattern::RobertsRules,
    ];
    
    let mut handles = Vec::new();
    
    // Test each coordination pattern under concurrent load
    for pattern in patterns {
        let coord = coordinator.clone();
        let handle = tokio::spawn(async move {
            let pattern_name = format!("{:?}", pattern);
            
            // Run coordination pattern multiple times
            for i in 0..10 {
                match coord.coordinate(pattern.clone()).await {
                    Ok(()) => (),
                    Err(e) => {
                        println!("❌ Pattern {} failed on iteration {}: {}", pattern_name, i, e);
                        return false;
                    }
                }
                
                // Small delay between runs
                sleep(Duration::from_millis(10)).await;
            }
            
            println!("✅ Pattern {} completed successfully", pattern_name);
            true
        });
        handles.push(handle);
    }
    
    // Wait for all patterns to complete
    let results = join_all(handles).await;
    
    let mut success_count = 0;
    for result in results {
        match result {
            Ok(true) => success_count += 1,
            Ok(false) => (),
            Err(e) => println!("❌ Pattern test task failed: {}", e),
        }
    }
    
    println!("✅ Coordination pattern stability: {}/4 patterns passed", success_count);
    assert_eq!(success_count, 4, "All coordination patterns should work without errors");
}

/// Test SwarmSystem initialization and basic functionality
#[tokio::test] 
async fn test_swarm_system_integration() {
    println!("🧪 Testing SwarmSystem integration...");
    
    // Test system initialization
    let system = match SwarmSystem::new().await {
        Ok(sys) => sys,
        Err(e) => {
            println!("❌ SwarmSystem initialization failed: {}", e);
            panic!("System should initialize successfully");
        }
    };
    
    // Test system startup
    match system.start().await {
        Ok(()) => println!("✅ SwarmSystem started successfully"),
        Err(e) => {
            println!("❌ SwarmSystem startup failed: {}", e);
            panic!("System should start successfully");
        }
    }
    
    // Test basic agent registration through system
    let agent = AgentSpec {
        id: "integration_test_agent".to_string(),
        role: "integration_tester".to_string(),
        capacity: 1.0,
        specializations: vec!["integration".to_string()],
        work_capacity: Some(5),
    };
    
    match system.coordinator.register_agent(agent).await {
        Ok(()) => println!("✅ Agent registration through system succeeded"),
        Err(e) => {
            println!("❌ Agent registration failed: {}", e);
            panic!("Agent registration should work");
        }
    }
    
    // Test system shutdown
    match system.stop().await {
        Ok(()) => println!("✅ SwarmSystem stopped successfully"),
        Err(e) => {
            println!("❌ SwarmSystem shutdown failed: {}", e);
            panic!("System should stop gracefully");
        }
    }
    
    println!("✅ SwarmSystem integration test passed");
}

/// Performance benchmark for coordination overhead
#[tokio::test]
async fn test_coordination_performance_benchmark() {
    println!("🧪 Running coordination performance benchmark...");
    
    let telemetry = Arc::new(TelemetryManager::new().await.unwrap());
    let work_queue = Arc::new(WorkQueue::new(None).await.unwrap());
    let coordinator = Arc::new(
        AgentCoordinator::new(telemetry, work_queue).await.unwrap()
    );
    
    let num_operations = 1000;
    let start_time = Instant::now();
    
    // Benchmark agent registrations
    for i in 0..num_operations {
        let spec = AgentSpec {
            id: format!("bench_agent_{}", i),
            role: "benchmark".to_string(),
            capacity: 1.0,
            specializations: vec!["bench".to_string()],
            work_capacity: Some(1),
        };
        
        coordinator.register_agent(spec).await.unwrap();
    }
    
    let total_duration = start_time.elapsed();
    let ops_per_second = (num_operations as f64) / total_duration.as_secs_f64();
    let avg_latency_us = total_duration.as_micros() / num_operations as u128;
    
    println!("📊 Performance Benchmark Results:");
    println!("   Operations: {}", num_operations);
    println!("   Total Duration: {:.2}s", total_duration.as_secs_f64());
    println!("   Ops/Second: {:.2}", ops_per_second);
    println!("   Avg Latency: {}μs", avg_latency_us);
    
    // Performance assertions
    assert!(ops_per_second > 100.0, "Should achieve >100 operations/second");
    assert!(avg_latency_us < 10_000, "Average latency should be <10ms");
    
    println!("✅ Performance benchmark passed");
}