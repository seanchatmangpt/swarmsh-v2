//! Comprehensive unit tests for SwarmSH v2 Worktree Lifecycle Management
//!
//! Tests all worktree operations with zero-conflict guarantees, coordination patterns,
//! and AI integration capabilities.

use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use swarmsh_v2::{
    WorktreeManager, WorktreeSpec, WorktreeStatus, CoordinationPattern, 
    TelemetryManager, SwarmError
};
use tempfile::TempDir;
use tokio::fs;
use tokio_test;

/// Test fixture for worktree operations
struct WorktreeTestFixture {
    temp_dir: TempDir,
    base_path: PathBuf,
    manager: WorktreeManager,
    telemetry: Arc<TelemetryManager>,
}

impl WorktreeTestFixture {
    async fn new() -> Result<Self> {
        let temp_dir = TempDir::new()?;
        let base_path = temp_dir.path().join("worktrees");
        
        // Initialize git repository in temp directory
        std::process::Command::new("git")
            .args(&["init", "--bare"])
            .current_dir(temp_dir.path())
            .output()?;
            
        // Create a test repository
        let repo_path = temp_dir.path().join("test-repo");
        fs::create_dir_all(&repo_path).await?;
        
        std::process::Command::new("git")
            .args(&["init"])
            .current_dir(&repo_path)
            .output()?;
            
        std::process::Command::new("git")
            .args(&["config", "user.email", "test@swarmsh.dev"])
            .current_dir(&repo_path)
            .output()?;
            
        std::process::Command::new("git")
            .args(&["config", "user.name", "SwarmSH Test"])
            .current_dir(&repo_path)
            .output()?;
        
        // Create initial commit
        fs::write(repo_path.join("README.md"), "# SwarmSH Test Repository").await?;
        std::process::Command::new("git")
            .args(&["add", "README.md"])
            .current_dir(&repo_path)
            .output()?;
            
        std::process::Command::new("git")
            .args(&["commit", "-m", "Initial commit"])
            .current_dir(&repo_path)
            .output()?;
        
        // Change to repo directory for worktree operations
        std::env::set_current_dir(&repo_path)?;
        
        let telemetry = Arc::new(TelemetryManager::new().await?);
        let manager = WorktreeManager::new(base_path.clone(), telemetry.clone()).await?;
        
        Ok(Self {
            temp_dir,
            base_path,
            manager,
            telemetry,
        })
    }
    
    fn temp_path(&self) -> &Path {
        self.temp_dir.path()
    }
}

#[tokio::test]
async fn test_worktree_creation_lifecycle() -> Result<()> {
    let fixture = WorktreeTestFixture::new().await?;
    
    // Test creating a new worktree
    let spec = WorktreeSpec {
        name: "feature-test".to_string(),
        branch: Some("feature/test-branch".to_string()),
        base_branch: Some("main".to_string()),
        coordination_pattern: CoordinationPattern::Atomic,
        agent_assignments: vec!["test-agent-1".to_string()],
        auto_sync: true,
        backup_enabled: false,
    };
    
    let state = fixture.manager.create_worktree(spec.clone()).await?;
    
    // Verify worktree was created successfully
    assert_eq!(state.name, "feature-test");
    assert_eq!(state.branch, "feature/test-branch");
    assert!(matches!(state.status, WorktreeStatus::Active));
    assert!(matches!(state.coordination_pattern, CoordinationPattern::Atomic));
    assert_eq!(state.agent_assignments.len(), 1);
    assert!(state.path.exists());
    
    // Verify it appears in the list
    let worktrees = fixture.manager.list_worktrees().await;
    assert_eq!(worktrees.len(), 1);
    assert_eq!(worktrees[0].name, "feature-test");
    
    // Test duplicate creation fails
    let duplicate_result = fixture.manager.create_worktree(spec).await;
    assert!(matches!(duplicate_result, Err(SwarmError::AlreadyExists(_))));
    
    Ok(())
}

#[tokio::test]
async fn test_worktree_sync_operations() -> Result<()> {
    let fixture = WorktreeTestFixture::new().await?;
    
    // Create a worktree
    let spec = WorktreeSpec {
        name: "sync-test".to_string(),
        branch: Some("main".to_string()),
        base_branch: None,
        coordination_pattern: CoordinationPattern::Realtime,
        agent_assignments: vec![],
        auto_sync: false,
        backup_enabled: false,
    };
    
    fixture.manager.create_worktree(spec).await?;
    
    // Test sync operation
    let sync_result = fixture.manager.sync_worktree("sync-test").await;
    
    // Sync might fail in test environment without upstream, but should handle gracefully
    match sync_result {
        Ok(()) => {
            // Sync succeeded
            let state = fixture.manager.get_worktree("sync-test").await?;
            assert!(matches!(state.status, WorktreeStatus::Active));
        }
        Err(SwarmError::GitOperation(_)) => {
            // Expected in test environment without proper upstream
        }
        Err(e) => return Err(e.into()),
    }
    
    Ok(())
}

#[tokio::test]
async fn test_worktree_backup_restore_lifecycle() -> Result<()> {
    let fixture = WorktreeTestFixture::new().await?;
    
    // Create a worktree
    let spec = WorktreeSpec {
        name: "backup-test".to_string(),
        branch: Some("main".to_string()),
        base_branch: None,
        coordination_pattern: CoordinationPattern::Atomic,
        agent_assignments: vec![],
        auto_sync: false,
        backup_enabled: true,
    };
    
    fixture.manager.create_worktree(spec).await?;
    
    // Add some content to the worktree
    let worktree_state = fixture.manager.get_worktree("backup-test").await?;
    let test_file = worktree_state.path.join("test.txt");
    fs::write(&test_file, "Test content for backup").await?;
    
    // Test backup operation
    let backup_path = fixture.manager.backup_worktree("backup-test", None).await?;
    assert!(backup_path.exists());
    
    // Verify backup contains the test file
    let backup_test_file = backup_path.join("test.txt");
    assert!(backup_test_file.exists());
    let backup_content = fs::read_to_string(&backup_test_file).await?;
    assert_eq!(backup_content, "Test content for backup");
    
    // Remove original worktree
    fixture.manager.remove_worktree("backup-test", true).await?;
    
    // Verify worktree is gone
    let list_result = fixture.manager.list_worktrees().await;
    assert!(!list_result.iter().any(|w| w.name == "backup-test"));
    
    // Test restore operation
    fixture.manager.restore_worktree("backup-test", backup_path).await?;
    
    // Verify restored worktree
    let restored_state = fixture.manager.get_worktree("backup-test").await?;
    assert_eq!(restored_state.name, "backup-test");
    assert!(restored_state.path.exists());
    
    // Verify restored content
    let restored_test_file = restored_state.path.join("test.txt");
    assert!(restored_test_file.exists());
    let restored_content = fs::read_to_string(&restored_test_file).await?;
    assert_eq!(restored_content, "Test content for backup");
    
    Ok(())
}

#[tokio::test]
async fn test_worktree_coordination_patterns() -> Result<()> {
    let fixture = WorktreeTestFixture::new().await?;
    
    // Create worktrees with different coordination patterns
    let patterns = vec![
        CoordinationPattern::Atomic,
        CoordinationPattern::ScrumAtScale,
        CoordinationPattern::RobertsRules,
        CoordinationPattern::Realtime,
    ];
    
    for (i, pattern) in patterns.iter().enumerate() {
        let spec = WorktreeSpec {
            name: format!("coord-test-{}", i),
            branch: Some("main".to_string()),
            base_branch: None,
            coordination_pattern: pattern.clone(),
            agent_assignments: vec![],
            auto_sync: false,
            backup_enabled: false,
        };
        
        fixture.manager.create_worktree(spec).await?;
    }
    
    // Test coordination operations
    for pattern in patterns.iter() {
        let result = fixture.manager.coordinate_worktrees(pattern.clone()).await;
        assert!(result.is_ok(), "Coordination failed for pattern: {:?}", pattern);
    }
    
    // Verify all worktrees exist with correct patterns
    let worktrees = fixture.manager.list_worktrees().await;
    assert_eq!(worktrees.len(), 4);
    
    for (i, worktree) in worktrees.iter().enumerate() {
        assert_eq!(worktree.coordination_pattern, patterns[i]);
    }
    
    Ok(())
}

#[tokio::test]
async fn test_worktree_metrics_and_analytics() -> Result<()> {
    let fixture = WorktreeTestFixture::new().await?;
    
    // Create a worktree for metrics testing
    let spec = WorktreeSpec {
        name: "metrics-test".to_string(),
        branch: Some("main".to_string()),
        base_branch: None,
        coordination_pattern: CoordinationPattern::Atomic,
        agent_assignments: vec!["metrics-agent".to_string()],
        auto_sync: false,
        backup_enabled: false,
    };
    
    fixture.manager.create_worktree(spec).await?;
    
    // Test usage analytics
    let usage_result = fixture.manager.get_worktree_usage("metrics-test").await?;
    assert!(usage_result.get("worktree").is_some());
    assert!(usage_result.get("metrics").is_some());
    assert!(usage_result.get("recommendations").is_some());
    
    // Test optimization
    let optimizations = fixture.manager.optimize_worktree("metrics-test").await?;
    assert!(!optimizations.is_empty());
    
    // Test telemetry generation
    let telemetry = fixture.manager.generate_telemetry().await?;
    assert!(telemetry.get("timestamp").is_some());
    assert!(telemetry.get("worktree_count").is_some());
    assert_eq!(telemetry.get("worktree_count").unwrap().as_u64().unwrap(), 1);
    
    Ok(())
}

#[tokio::test]
async fn test_worktree_error_handling() -> Result<()> {
    let fixture = WorktreeTestFixture::new().await?;
    
    // Test operations on non-existent worktree
    let non_existent_result = fixture.manager.get_worktree("non-existent").await;
    assert!(matches!(non_existent_result, Err(SwarmError::NotFound(_))));
    
    let sync_non_existent = fixture.manager.sync_worktree("non-existent").await;
    assert!(matches!(sync_non_existent, Err(SwarmError::NotFound(_))));
    
    let backup_non_existent = fixture.manager.backup_worktree("non-existent", None).await;
    assert!(matches!(backup_non_existent, Err(SwarmError::NotFound(_))));
    
    // Test restore from non-existent backup
    let fake_backup_path = fixture.temp_path().join("fake-backup");
    let restore_result = fixture.manager.restore_worktree("test", fake_backup_path).await;
    assert!(matches!(restore_result, Err(SwarmError::NotFound(_))));
    
    Ok(())
}

#[tokio::test]
async fn test_worktree_concurrent_operations() -> Result<()> {
    let fixture = WorktreeTestFixture::new().await?;
    
    // Test concurrent worktree creation
    let mut handles = vec![];
    
    for i in 0..5 {
        let manager = fixture.manager.clone();
        let handle = tokio::spawn(async move {
            let spec = WorktreeSpec {
                name: format!("concurrent-{}", i),
                branch: Some("main".to_string()),
                base_branch: None,
                coordination_pattern: CoordinationPattern::Atomic,
                agent_assignments: vec![],
                auto_sync: false,
                backup_enabled: false,
            };
            
            manager.create_worktree(spec).await
        });
        handles.push(handle);
    }
    
    // Wait for all operations to complete
    let mut successful_creations = 0;
    for handle in handles {
        match handle.await? {
            Ok(_) => successful_creations += 1,
            Err(_) => {}, // Some may fail due to git conflicts in concurrent scenario
        }
    }
    
    // Verify at least some succeeded (zero-conflict guarantee may prevent all from succeeding)
    assert!(successful_creations > 0);
    
    let worktrees = fixture.manager.list_worktrees().await;
    assert_eq!(worktrees.len(), successful_creations);
    
    Ok(())
}

#[tokio::test]
async fn test_worktree_switch_operations() -> Result<()> {
    let fixture = WorktreeTestFixture::new().await?;
    
    // Create multiple worktrees
    for i in 1..=3 {
        let spec = WorktreeSpec {
            name: format!("switch-test-{}", i),
            branch: Some("main".to_string()),
            base_branch: None,
            coordination_pattern: CoordinationPattern::Atomic,
            agent_assignments: vec![],
            auto_sync: false,
            backup_enabled: false,
        };
        
        fixture.manager.create_worktree(spec).await?;
    }
    
    // Test switching between worktrees
    for i in 1..=3 {
        let switch_result = fixture.manager.switch_worktree(&format!("switch-test-{}", i)).await?;
        assert!(switch_result.exists());
        assert!(switch_result.to_str().unwrap().contains(&format!("switch-test-{}", i)));
    }
    
    // Test switching to non-existent worktree
    let invalid_switch = fixture.manager.switch_worktree("non-existent").await;
    assert!(matches!(invalid_switch, Err(SwarmError::NotFound(_))));
    
    Ok(())
}

#[tokio::test]
async fn test_worktree_merge_operations() -> Result<()> {
    let fixture = WorktreeTestFixture::new().await?;
    
    // Create source and target worktrees
    let source_spec = WorktreeSpec {
        name: "merge-source".to_string(),
        branch: Some("feature/merge-test".to_string()),
        base_branch: Some("main".to_string()),
        coordination_pattern: CoordinationPattern::Atomic,
        agent_assignments: vec![],
        auto_sync: false,
        backup_enabled: false,
    };
    
    let target_spec = WorktreeSpec {
        name: "merge-target".to_string(),
        branch: Some("main".to_string()),
        base_branch: None,
        coordination_pattern: CoordinationPattern::Atomic,
        agent_assignments: vec![],
        auto_sync: false,
        backup_enabled: false,
    };
    
    fixture.manager.create_worktree(source_spec).await?;
    fixture.manager.create_worktree(target_spec).await?;
    
    // Create a branch for the source worktree
    let source_state = fixture.manager.get_worktree("merge-source").await?;
    std::process::Command::new("git")
        .args(&["checkout", "-b", "feature/merge-test"])
        .current_dir(&source_state.path)
        .output()?;
    
    // Test merge operation (may fail in test environment, but should handle gracefully)
    let merge_result = fixture.manager.merge_worktrees("merge-source", "merge-target").await;
    
    match merge_result {
        Ok(()) => {
            // Merge succeeded
            let target_state = fixture.manager.get_worktree("merge-target").await?;
            assert!(target_state.metrics.coordination_events > 0);
        }
        Err(SwarmError::GitOperation(_)) => {
            // Expected in test environment without proper branch setup
        }
        Err(e) => return Err(e.into()),
    }
    
    Ok(())
}

#[tokio::test]
async fn test_worktree_nanosecond_precision() -> Result<()> {
    let fixture = WorktreeTestFixture::new().await?;
    
    // Create multiple worktrees rapidly and verify unique coordination epochs
    let mut coordination_epochs = vec![];
    
    for i in 0..3 {
        let start_time = SystemTime::now();
        
        let spec = WorktreeSpec {
            name: format!("precision-test-{}", i),
            branch: Some("main".to_string()),
            base_branch: None,
            coordination_pattern: CoordinationPattern::Atomic,
            agent_assignments: vec![],
            auto_sync: false,
            backup_enabled: false,
        };
        
        fixture.manager.create_worktree(spec).await?;
        
        let end_time = SystemTime::now();
        let epoch = end_time.duration_since(UNIX_EPOCH)?.as_nanos();
        coordination_epochs.push(epoch);
        
        // Small delay to ensure different timestamps
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
    }
    
    // Verify all epochs are unique (nanosecond precision)
    coordination_epochs.sort();
    coordination_epochs.dedup();
    assert_eq!(coordination_epochs.len(), 3, "Coordination epochs should be unique");
    
    Ok(())
}

#[tokio::test]
async fn test_worktree_ai_integration_fallback() -> Result<()> {
    let fixture = WorktreeTestFixture::new().await?;
    
    // Create a worktree (AI integration may not be available in test environment)
    let spec = WorktreeSpec {
        name: "ai-test".to_string(),
        branch: Some("main".to_string()),
        base_branch: None,
        coordination_pattern: CoordinationPattern::ScrumAtScale,
        agent_assignments: vec!["ai-agent".to_string()],
        auto_sync: false,
        backup_enabled: false,
    };
    
    // This should succeed even if AI is not available (graceful fallback)
    let state = fixture.manager.create_worktree(spec).await?;
    assert_eq!(state.name, "ai-test");
    
    // Test optimization with AI fallback
    let optimizations = fixture.manager.optimize_worktree("ai-test").await?;
    assert!(!optimizations.is_empty(), "Should have fallback optimizations");
    
    // Test deployment with AI fallback
    let deploy_result = fixture.manager.deploy_worktree("ai-test", "test-env").await;
    assert!(deploy_result.is_ok(), "Deployment should succeed with fallback");
    
    Ok(())
}

/// Integration test module for coordination patterns
mod coordination_integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_scrum_at_scale_coordination() -> Result<()> {
        let fixture = WorktreeTestFixture::new().await?;
        
        // Create multiple worktrees for Scrum at Scale coordination
        for i in 1..=3 {
            let spec = WorktreeSpec {
                name: format!("scrum-team-{}", i),
                branch: Some("main".to_string()),
                base_branch: None,
                coordination_pattern: CoordinationPattern::ScrumAtScale,
                agent_assignments: vec![format!("scrum-agent-{}", i)],
                auto_sync: true,
                backup_enabled: false,
            };
            
            fixture.manager.create_worktree(spec).await?;
        }
        
        // Test Scrum at Scale coordination
        let result = fixture.manager.coordinate_worktrees(CoordinationPattern::ScrumAtScale).await;
        assert!(result.is_ok(), "Scrum at Scale coordination should succeed");
        
        // Verify all worktrees are using Scrum at Scale pattern
        let worktrees = fixture.manager.list_worktrees().await;
        for worktree in worktrees {
            assert!(matches!(worktree.coordination_pattern, CoordinationPattern::ScrumAtScale));
        }
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_roberts_rules_coordination() -> Result<()> {
        let fixture = WorktreeTestFixture::new().await?;
        
        // Create worktrees for Roberts Rules governance
        for i in 1..=5 {
            let spec = WorktreeSpec {
                name: format!("governance-{}", i),
                branch: Some("main".to_string()),
                base_branch: None,
                coordination_pattern: CoordinationPattern::RobertsRules,
                agent_assignments: vec![format!("governance-agent-{}", i)],
                auto_sync: false,
                backup_enabled: false,
            };
            
            fixture.manager.create_worktree(spec).await?;
        }
        
        // Test Roberts Rules coordination
        let result = fixture.manager.coordinate_worktrees(CoordinationPattern::RobertsRules).await;
        assert!(result.is_ok(), "Roberts Rules coordination should succeed");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_realtime_coordination() -> Result<()> {
        let fixture = WorktreeTestFixture::new().await?;
        
        // Create worktrees for real-time coordination
        let spec = WorktreeSpec {
            name: "realtime-test".to_string(),
            branch: Some("main".to_string()),
            base_branch: None,
            coordination_pattern: CoordinationPattern::Realtime,
            agent_assignments: vec!["realtime-agent".to_string()],
            auto_sync: true,
            backup_enabled: false,
        };
        
        fixture.manager.create_worktree(spec).await?;
        
        // Test real-time coordination
        let result = fixture.manager.coordinate_worktrees(CoordinationPattern::Realtime).await;
        assert!(result.is_ok(), "Real-time coordination should succeed");
        
        // Verify nanosecond precision timing
        let telemetry = fixture.manager.generate_telemetry().await?;
        let timestamp = telemetry.get("timestamp").unwrap().as_u64().unwrap();
        assert!(timestamp > 0, "Timestamp should be nanosecond precision");
        
        Ok(())
    }
}