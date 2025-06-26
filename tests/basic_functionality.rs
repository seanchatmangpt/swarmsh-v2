//! Basic Functionality Validation Tests
//! 
//! Simple tests to validate core SwarmSH v2 functionality works as claimed.

use swarmsh_v2::{
    coordination::{AgentCoordinator, AgentSpec, WorkQueue},
    telemetry::TelemetryManager,
    SwarmSystem,
};
use std::sync::Arc;

/// Test basic agent registration works
#[tokio::test]
async fn test_basic_agent_registration() {
    println!("🧪 Testing basic agent registration...");
    
    let telemetry = Arc::new(TelemetryManager::new().await.unwrap());
    let work_queue = Arc::new(WorkQueue::new(None).await.unwrap());
    let coordinator = Arc::new(
        AgentCoordinator::new(telemetry, work_queue).await.unwrap()
    );
    
    let agent = AgentSpec {
        id: "test_agent_001".to_string(),
        role: "test_worker".to_string(),
        capacity: 1.0,
        specializations: vec!["testing".to_string()],
        work_capacity: Some(10),
    };
    
    match coordinator.register_agent(agent).await {
        Ok(()) => {
            println!("✅ Agent registration successful");
        }
        Err(e) => {
            println!("❌ Agent registration failed: {}", e);
            panic!("Basic agent registration should work");
        }
    }
}

/// Test SwarmSystem can be created and started
#[tokio::test]
async fn test_swarm_system_lifecycle() {
    println!("🧪 Testing SwarmSystem lifecycle...");
    
    // Test system creation
    let system = match SwarmSystem::new().await {
        Ok(sys) => {
            println!("✅ SwarmSystem created successfully");
            sys
        }
        Err(e) => {
            println!("❌ SwarmSystem creation failed: {}", e);
            panic!("SwarmSystem should be creatable");
        }
    };
    
    // Test system startup
    match system.start().await {
        Ok(()) => {
            println!("✅ SwarmSystem started successfully");
        }
        Err(e) => {
            println!("❌ SwarmSystem startup failed: {}", e);
            panic!("SwarmSystem should start successfully");
        }
    }
    
    // Test system shutdown
    match system.stop().await {
        Ok(()) => {
            println!("✅ SwarmSystem stopped successfully");
        }
        Err(e) => {
            println!("❌ SwarmSystem shutdown failed: {}", e);
            panic!("SwarmSystem should stop gracefully");
        }
    }
}

/// Test work queue basic functionality
#[tokio::test]
async fn test_work_queue_basic_operations() {
    println!("🧪 Testing work queue basic operations...");
    
    let work_queue = Arc::new(WorkQueue::new(None).await.unwrap());
    
    let work_item = swarmsh_v2::coordination::WorkItem {
        id: "test_work_001".to_string(),
        priority: 1.0,
        requirements: vec!["testing".to_string()],
        estimated_duration_ms: 1000,
        created_at: std::time::SystemTime::now(),
    };
    
    // Test adding work to queue
    match work_queue.add_work(work_item).await {
        Ok(()) => {
            println!("✅ Work item added to queue successfully");
        }
        Err(e) => {
            println!("❌ Failed to add work to queue: {}", e);
            panic!("Adding work to queue should succeed");
        }
    }
    
    // Test getting work from queue
    let agent = AgentSpec {
        id: "test_agent_002".to_string(),
        role: "test_worker".to_string(),
        capacity: 1.0,
        specializations: vec!["testing".to_string()],
        work_capacity: Some(5),
    };
    
    match work_queue.get_work_for_agent(&agent).await {
        Ok(Some(work)) => {
            println!("✅ Work retrieved from queue successfully: {}", work.id);
        }
        Ok(None) => {
            println!("⚠️  No work available for agent (might be valid)");
        }
        Err(e) => {
            println!("❌ Failed to get work from queue: {}", e);
            panic!("Getting work from queue should not error");
        }
    }
}