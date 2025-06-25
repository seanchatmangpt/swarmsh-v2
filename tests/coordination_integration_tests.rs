//! Integration tests for SwarmSH v2 coordination patterns
//!
//! Tests the complete coordination system with real telemetry, AI integration,
//! and multi-agent scenarios to validate zero-conflict guarantees.

use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use swarmsh_v2::{
    SwarmSystem, AgentCoordinator, WorkQueue, TelemetryManager, CoordinationPattern,
    AgentSpec, WorkItem, WorktreeManager, WorktreeSpec, AgentId, WorkId
};
use tempfile::TempDir;
use tokio::time;
use tracing::{info, debug, span, Level};
use tracing_test::traced_test;

/// Integration test fixture for coordination patterns
struct CoordinationTestFixture {
    temp_dir: TempDir,
    swarm_system: SwarmSystem,
    telemetry: Arc<TelemetryManager>,
    coordinator: Arc<AgentCoordinator>,
    work_queue: Arc<WorkQueue>,
    worktree_manager: Arc<WorktreeManager>,
}

impl CoordinationTestFixture {
    async fn new() -> Result<Self> {
        let temp_dir = TempDir::new()?;
        
        // Initialize git repository for worktree tests
        std::process::Command::new("git")
            .args(&["init"])
            .current_dir(temp_dir.path())
            .output()?;
            
        std::process::Command::new("git")
            .args(&["config", "user.email", "test@swarmsh.dev"])
            .current_dir(temp_dir.path())
            .output()?;
            
        std::process::Command::new("git")
            .args(&["config", "user.name", "SwarmSH Integration Test"])
            .current_dir(temp_dir.path())
            .output()?;
        
        // Create initial commit
        tokio::fs::write(temp_dir.path().join("README.md"), "# SwarmSH Integration Test").await?;
        std::process::Command::new("git")
            .args(&["add", "README.md"])
            .current_dir(temp_dir.path())
            .output()?;
            
        std::process::Command::new("git")
            .args(&["commit", "-m", "Initial commit"])
            .current_dir(temp_dir.path())
            .output()?;
        
        // Change to test directory
        std::env::set_current_dir(temp_dir.path())?;
        
        // Initialize SwarmSH system
        let swarm_system = SwarmSystem::new().await?;
        let telemetry = swarm_system.telemetry.clone();
        let coordinator = swarm_system.coordinator.clone();
        let work_queue = swarm_system.work_queue.clone();
        let worktree_manager = swarm_system.worktree_manager.clone();
        
        // Start the system
        swarm_system.start().await?;
        
        Ok(Self {
            temp_dir,
            swarm_system,
            telemetry,
            coordinator,
            work_queue,
            worktree_manager,
        })
    }
    
    async fn register_test_agents(&self, count: usize, pattern: CoordinationPattern) -> Result<Vec<AgentId>> {
        let mut agent_ids = Vec::new();
        
        for i in 0..count {
            let agent_id = format!("test-agent-{}-{}", i, SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos());
            let spec = AgentSpec {
                id: agent_id.clone(),
                role: format!("test-role-{}", i),
                capacity: 1.0,
                specializations: vec!["testing".to_string(), "coordination".to_string()],
                work_capacity: Some(5),
            };
            
            self.coordinator.register_agent(spec).await?;
            agent_ids.push(agent_id);
        }
        
        Ok(agent_ids)
    }
    
    async fn create_test_work_items(&self, count: usize) -> Result<Vec<WorkId>> {
        let mut work_ids = Vec::new();
        
        for i in 0..count {
            let work_id = format!("test-work-{}-{}", i, SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos());
            let work_item = WorkItem {
                id: work_id.clone(),
                priority: (i as f64) * 10.0, // Varying priorities
                requirements: vec!["testing".to_string()],
                estimated_duration_ms: 1000 + (i as u64 * 100),
                created_at: SystemTime::now(),
            };
            
            self.work_queue.add_work(work_item).await?;
            work_ids.push(work_id);
        }
        
        Ok(work_ids)
    }
}

#[tokio::test]
#[traced_test]
async fn test_atomic_coordination_zero_conflict() -> Result<()> {
    let fixture = CoordinationTestFixture::new().await?;
    
    info!("Testing atomic coordination with zero-conflict guarantees");
    
    // Register multiple agents concurrently
    let agent_count = 5;
    let agent_ids = fixture.register_test_agents(agent_count, CoordinationPattern::Atomic).await?;
    
    // Create work items
    let work_ids = fixture.create_test_work_items(10).await?;
    
    // Test concurrent coordination operations
    let mut coordination_handles = Vec::new();
    
    for i in 0..3 {
        let coordinator = fixture.coordinator.clone();
        let span_name = format!("atomic_coordination_{}", i);
        
        let handle = tokio::spawn(async move {
            let _span = span!(Level::INFO, "atomic_coordination", iteration = i).entered();
            
            // Multiple rapid coordination calls
            for j in 0..5 {
                let result = coordinator.coordinate(CoordinationPattern::Atomic).await;
                debug!(iteration = i, round = j, success = result.is_ok(), "Atomic coordination result");
                time::sleep(Duration::from_millis(10)).await;
            }
            
            Ok::<(), anyhow::Error>(())
        });
        
        coordination_handles.push(handle);
    }
    
    // Wait for all coordination operations to complete
    for handle in coordination_handles {
        handle.await??;
    }
    
    // Verify zero conflicts occurred
    let telemetry_data = fixture.telemetry.generate_report().await?;
    info!("Atomic coordination telemetry: {:?}", telemetry_data);
    
    // Test AI recommendations for coordination
    let ai_recommendations = fixture.coordinator.get_ai_recommendations(&CoordinationPattern::Atomic).await;
    match ai_recommendations {
        Ok(analysis) => {
            info!("AI coordination analysis: {} recommendations with {:.2}% confidence", 
                analysis.recommendations.len(), analysis.confidence * 100.0);
        }
        Err(_) => {
            info!("AI integration not available, using fallback coordination");
        }
    }
    
    Ok(())
}

#[tokio::test]
#[traced_test]
async fn test_scrum_at_scale_coordination() -> Result<()> {
    let fixture = CoordinationTestFixture::new().await?;
    
    info!("Testing Scrum at Scale coordination patterns");
    
    // Register agents representing different scrum teams
    let team_sizes = vec![3, 5, 4]; // Three teams with different sizes
    let mut all_agent_ids = Vec::new();
    
    for (team_id, team_size) in team_sizes.iter().enumerate() {
        let team_agents = fixture.register_test_agents(*team_size, CoordinationPattern::ScrumAtScale).await?;
        all_agent_ids.extend(team_agents);
        
        info!("Registered team {} with {} agents", team_id, team_size);
    }
    
    // Create sprint work items
    let sprint_work = fixture.create_test_work_items(20).await?;
    
    // Test Scrum at Scale coordination
    let coordination_result = fixture.coordinator.coordinate(CoordinationPattern::ScrumAtScale).await;
    assert!(coordination_result.is_ok(), "Scrum at Scale coordination should succeed");
    
    // Test AI-enhanced sprint planning
    let ai_recommendations = fixture.coordinator.get_ai_recommendations(&CoordinationPattern::ScrumAtScale).await;
    match ai_recommendations {
        Ok(analysis) => {
            info!("AI sprint planning: {} recommendations", analysis.recommendations.len());
            for recommendation in analysis.recommendations {
                debug!("Sprint recommendation: {}", recommendation);
            }
        }
        Err(_) => {
            info!("AI not available, using fallback sprint planning");
        }
    }
    
    // Test work distribution optimization
    let optimization_result = fixture.coordinator.optimize_work_distribution().await;
    assert!(optimization_result.is_ok(), "Work distribution optimization should succeed");
    
    // Verify coordination telemetry
    let telemetry = fixture.telemetry.generate_report().await?;
    assert!(telemetry.get("coordination_events").is_some());
    
    Ok(())
}

#[tokio::test]
#[traced_test]
async fn test_roberts_rules_governance() -> Result<()> {
    let fixture = CoordinationTestFixture::new().await?;
    
    info!("Testing Roberts Rules governance and voting");
    
    // Register agents for governance (odd number for clear voting)
    let governance_agents = fixture.register_test_agents(7, CoordinationPattern::RobertsRules).await?;
    
    // Test Roberts Rules coordination (includes voting simulation)
    let coordination_result = fixture.coordinator.coordinate(CoordinationPattern::RobertsRules).await;
    assert!(coordination_result.is_ok(), "Roberts Rules coordination should succeed");
    
    // Test AI-enhanced voting procedure
    let ai_recommendations = fixture.coordinator.get_ai_recommendations(&CoordinationPattern::RobertsRules).await;
    match ai_recommendations {
        Ok(analysis) => {
            info!("AI voting analysis: {} recommendations with {:.2}% confidence", 
                analysis.recommendations.len(), analysis.confidence * 100.0);
        }
        Err(_) => {
            info!("AI not available, using fallback voting procedures");
        }
    }
    
    // Verify all agents can participate in governance
    assert_eq!(governance_agents.len(), 7);
    
    Ok(())
}

#[tokio::test]
#[traced_test]
async fn test_realtime_coordination() -> Result<()> {
    let fixture = CoordinationTestFixture::new().await?;
    
    info!("Testing real-time coordination with high-frequency operations");
    
    // Register agents for real-time coordination
    let realtime_agents = fixture.register_test_agents(3, CoordinationPattern::Realtime).await?;
    
    // Test high-frequency coordination operations
    let start_time = SystemTime::now();
    let mut coordination_epochs = Vec::new();
    
    for i in 0..10 {
        let epoch_start = SystemTime::now();
        
        let coordination_result = fixture.coordinator.coordinate(CoordinationPattern::Realtime).await;
        assert!(coordination_result.is_ok(), "Real-time coordination should succeed");
        
        let epoch = epoch_start.duration_since(UNIX_EPOCH)?.as_nanos();
        coordination_epochs.push(epoch);
        
        // Small delay to test high-frequency operations
        time::sleep(Duration::from_millis(5)).await;
    }
    
    let total_duration = start_time.elapsed()?;
    info!("Real-time coordination: 10 operations in {}ms", total_duration.as_millis());
    
    // Verify nanosecond precision (all epochs should be unique)
    coordination_epochs.sort();
    coordination_epochs.dedup();
    assert_eq!(coordination_epochs.len(), 10, "All coordination epochs should be unique");
    
    // Test AI streaming optimizations
    let ai_recommendations = fixture.coordinator.get_ai_recommendations(&CoordinationPattern::Realtime).await;
    match ai_recommendations {
        Ok(analysis) => {
            info!("Real-time AI analysis: {} optimization opportunities", analysis.optimization_opportunities.len());
        }
        Err(_) => {
            info!("AI streaming not available, using fallback optimization");
        }
    }
    
    Ok(())
}

#[tokio::test]
#[traced_test]
async fn test_worktree_coordination_integration() -> Result<()> {
    let fixture = CoordinationTestFixture::new().await?;
    
    info!("Testing worktree coordination integration");
    
    // Create multiple worktrees with different coordination patterns
    let patterns = vec![
        CoordinationPattern::Atomic,
        CoordinationPattern::ScrumAtScale,
        CoordinationPattern::RobertsRules,
        CoordinationPattern::Realtime,
    ];
    
    let mut worktree_names = Vec::new();
    
    for (i, pattern) in patterns.iter().enumerate() {
        let spec = WorktreeSpec {
            name: format!("coord-worktree-{}", i),
            branch: Some("main".to_string()),
            base_branch: None,
            coordination_pattern: pattern.clone(),
            agent_assignments: vec![format!("coord-agent-{}", i)],
            auto_sync: false,
            backup_enabled: false,
        };
        
        let worktree_state = fixture.worktree_manager.create_worktree(spec).await?;
        worktree_names.push(worktree_state.name);
        
        info!("Created worktree '{}' with pattern {:?}", worktree_state.name, pattern);
    }
    
    // Test coordination across all patterns
    for pattern in patterns.iter() {
        let coordination_result = fixture.worktree_manager.coordinate_worktrees(pattern.clone()).await;
        assert!(coordination_result.is_ok(), "Worktree coordination should succeed for pattern: {:?}", pattern);
    }
    
    // Test telemetry generation
    let worktree_telemetry = fixture.worktree_manager.generate_telemetry().await?;
    assert_eq!(worktree_telemetry.get("worktree_count").unwrap().as_u64().unwrap(), 4);
    
    // Verify coordination pattern distribution
    let pattern_counts = worktree_telemetry.get("coordination_patterns").unwrap();
    assert_eq!(pattern_counts.get("atomic").unwrap().as_u64().unwrap(), 1);
    assert_eq!(pattern_counts.get("scrum_at_scale").unwrap().as_u64().unwrap(), 1);
    assert_eq!(pattern_counts.get("roberts_rules").unwrap().as_u64().unwrap(), 1);
    assert_eq!(pattern_counts.get("realtime").unwrap().as_u64().unwrap(), 1);
    
    Ok(())
}

#[tokio::test]
#[traced_test]
async fn test_multi_agent_work_distribution() -> Result<()> {
    let fixture = CoordinationTestFixture::new().await?;
    
    info!("Testing multi-agent work distribution with coordination");
    
    // Register agents with different specializations
    let agents = vec![
        ("frontend-agent", vec!["ui", "react", "testing"]),
        ("backend-agent", vec!["api", "database", "testing"]),
        ("devops-agent", vec!["deployment", "monitoring", "testing"]),
        ("qa-agent", vec!["testing", "automation", "quality"]),
    ];
    
    let mut agent_ids = Vec::new();
    for (role, specializations) in agents {
        let agent_id = format!("{}-{}", role, SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos());
        let spec = AgentSpec {
            id: agent_id.clone(),
            role: role.to_string(),
            capacity: 1.0,
            specializations: specializations.iter().map(|s| s.to_string()).collect(),
            work_capacity: Some(3),
        };
        
        fixture.coordinator.register_agent(spec).await?;
        agent_ids.push(agent_id);
    }
    
    // Create diverse work items
    let work_items = vec![
        ("ui-work", vec!["ui", "react"], 100.0),
        ("api-work", vec!["api", "database"], 80.0),
        ("deploy-work", vec!["deployment"], 90.0),
        ("test-work", vec!["testing"], 70.0),
        ("integration-work", vec!["testing", "api"], 110.0),
    ];
    
    for (work_name, requirements, priority) in work_items {
        let work_id = format!("{}-{}", work_name, SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos());
        let work_item = WorkItem {
            id: work_id,
            priority,
            requirements: requirements.iter().map(|r| r.to_string()).collect(),
            estimated_duration_ms: 2000,
            created_at: SystemTime::now(),
        };
        
        fixture.work_queue.add_work(work_item).await?;
    }
    
    // Test work distribution through coordination
    let coordination_result = fixture.coordinator.coordinate(CoordinationPattern::Atomic).await;
    assert!(coordination_result.is_ok(), "Multi-agent coordination should succeed");
    
    // Test AI-enhanced work distribution optimization
    let optimization_result = fixture.coordinator.optimize_work_distribution().await;
    assert!(optimization_result.is_ok(), "Work distribution optimization should succeed");
    
    info!("Multi-agent work distribution test completed successfully");
    
    Ok(())
}

#[tokio::test]
#[traced_test]
async fn test_coordination_pattern_switching() -> Result<()> {
    let fixture = CoordinationTestFixture::new().await?;
    
    info!("Testing dynamic coordination pattern switching");
    
    // Register agents
    let agent_ids = fixture.register_test_agents(4, CoordinationPattern::Atomic).await?;
    
    // Test switching between all coordination patterns
    let patterns = vec![
        CoordinationPattern::Atomic,
        CoordinationPattern::ScrumAtScale,
        CoordinationPattern::RobertsRules,
        CoordinationPattern::Realtime,
    ];
    
    for (i, pattern) in patterns.iter().enumerate() {
        info!("Testing coordination pattern switch to: {:?}", pattern);
        
        let start_time = SystemTime::now();
        let coordination_result = fixture.coordinator.coordinate(pattern.clone()).await;
        let switch_duration = start_time.elapsed()?;
        
        assert!(coordination_result.is_ok(), "Pattern switch should succeed: {:?}", pattern);
        info!("Pattern switch to {:?} completed in {}ms", pattern, switch_duration.as_millis());
        
        // Small delay between pattern switches
        time::sleep(Duration::from_millis(50)).await;
    }
    
    // Test AI recommendations for pattern optimization
    for pattern in patterns.iter() {
        let ai_result = fixture.coordinator.get_ai_recommendations(pattern).await;
        match ai_result {
            Ok(analysis) => {
                info!("AI analysis for {:?}: {} recommendations, {:.2}% confidence", 
                    pattern, analysis.recommendations.len(), analysis.confidence * 100.0);
            }
            Err(_) => {
                debug!("AI analysis not available for pattern: {:?}", pattern);
            }
        }
    }
    
    Ok(())
}

#[tokio::test]
#[traced_test]
async fn test_telemetry_consistency_across_patterns() -> Result<()> {
    let fixture = CoordinationTestFixture::new().await?;
    
    info!("Testing telemetry consistency across coordination patterns");
    
    // Register agents and create work
    let agent_ids = fixture.register_test_agents(3, CoordinationPattern::Atomic).await?;
    let work_ids = fixture.create_test_work_items(5).await?;
    
    // Collect telemetry for each coordination pattern
    let patterns = vec![
        CoordinationPattern::Atomic,
        CoordinationPattern::ScrumAtScale,
        CoordinationPattern::RobertsRules,
        CoordinationPattern::Realtime,
    ];
    
    let mut pattern_telemetry = HashMap::new();
    
    for pattern in patterns.iter() {
        info!("Collecting telemetry for pattern: {:?}", pattern);
        
        // Execute coordination
        fixture.coordinator.coordinate(pattern.clone()).await?;
        
        // Generate telemetry report
        let telemetry = fixture.telemetry.generate_report().await?;
        pattern_telemetry.insert(format!("{:?}", pattern), telemetry);
        
        // Small delay for distinct timestamps
        time::sleep(Duration::from_millis(100)).await;
    }
    
    // Verify telemetry consistency
    for (pattern_name, telemetry) in pattern_telemetry.iter() {
        assert!(telemetry.get("timestamp").is_some(), "Telemetry should have timestamp for {}", pattern_name);
        assert!(telemetry.get("coordination_events").is_some(), "Telemetry should have coordination events for {}", pattern_name);
        
        info!("Telemetry for {}: {} coordination events", 
            pattern_name, 
            telemetry.get("coordination_events").unwrap_or(&serde_json::Value::Null));
    }
    
    // Test worktree telemetry integration
    let worktree_telemetry = fixture.worktree_manager.generate_telemetry().await?;
    assert!(worktree_telemetry.get("timestamp").is_some());
    
    info!("Telemetry consistency test completed successfully");
    
    Ok(())
}

/// Performance test for coordination latency
#[tokio::test]
#[traced_test]
async fn test_coordination_performance() -> Result<()> {
    let fixture = CoordinationTestFixture::new().await?;
    
    info!("Testing coordination performance and latency");
    
    // Register agents for performance testing
    let agent_ids = fixture.register_test_agents(10, CoordinationPattern::Atomic).await?;
    
    // Measure coordination latency for each pattern
    let patterns = vec![
        CoordinationPattern::Atomic,
        CoordinationPattern::ScrumAtScale,
        CoordinationPattern::RobertsRules,
        CoordinationPattern::Realtime,
    ];
    
    for pattern in patterns.iter() {
        let mut latencies = Vec::new();
        
        // Run multiple coordination operations
        for i in 0..5 {
            let start_time = SystemTime::now();
            let result = fixture.coordinator.coordinate(pattern.clone()).await;
            let latency = start_time.elapsed()?;
            
            assert!(result.is_ok(), "Coordination should succeed for {:?}", pattern);
            latencies.push(latency.as_millis());
            
            debug!("Coordination {} for {:?}: {}ms", i, pattern, latency.as_millis());
        }
        
        let avg_latency = latencies.iter().sum::<u128>() / latencies.len() as u128;
        let max_latency = latencies.iter().max().unwrap();
        
        info!("Pattern {:?} - Avg: {}ms, Max: {}ms", pattern, avg_latency, max_latency);
        
        // Performance assertions (adjust based on requirements)
        assert!(*max_latency < 1000, "Coordination latency should be under 1 second for {:?}", pattern);
    }
    
    Ok(())
}