use std::{collections::HashSet, sync::Arc, time::SystemTime};

use swarmsh_v2::{
    coordination::{AgentCoordinator, AgentSpec, WorkItem, WorkQueue},
    telemetry::{TelemetryConfig, TelemetryManager, TelemetryMode},
    SwarmError,
};

fn agent(id: usize) -> AgentSpec {
    AgentSpec {
        id: format!("acceptance-agent-{id}"),
        role: "acceptance".to_string(),
        capacity: 1.0,
        specializations: vec![],
        work_capacity: Some(64),
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_work_claims_are_exactly_once() {
    const WORK_ITEMS: usize = 256;
    const AGENTS: usize = 32;

    let queue = Arc::new(WorkQueue::new(None).await.expect("work queue must initialize"));

    for id in 0..WORK_ITEMS {
        queue
            .add_work(WorkItem {
                id: format!("acceptance-work-{id}"),
                priority: (WORK_ITEMS - id) as f64,
                requirements: vec![],
                estimated_duration_ms: 1,
                created_at: SystemTime::now(),
            })
            .await
            .expect("work insertion must succeed");
    }

    let mut claimers = Vec::with_capacity(AGENTS);
    for id in 0..AGENTS {
        let queue = Arc::clone(&queue);
        claimers.push(tokio::spawn(async move {
            let spec = agent(id);
            let mut claimed = Vec::new();

            loop {
                match queue
                    .get_work_for_agent(&spec)
                    .await
                    .expect("work claim must not fail")
                {
                    Some(work) => claimed.push(work.id),
                    None => break,
                }
            }

            claimed
        }));
    }

    let mut claimed = Vec::with_capacity(WORK_ITEMS);
    for claimer in claimers {
        claimed.extend(claimer.await.expect("claimer task must join"));
    }

    assert_eq!(
        claimed.len(),
        WORK_ITEMS,
        "every admitted work item must be claimed exactly once"
    );

    let unique: HashSet<_> = claimed.iter().collect();
    assert_eq!(
        unique.len(),
        WORK_ITEMS,
        "no two concurrent agents may receive the same work item"
    );

    assert!(
        queue
            .get_work_for_agent(&agent(AGENTS + 1))
            .await
            .expect("empty queue lookup must succeed")
            .is_none(),
        "the queue must be empty after all claims"
    );
}

#[tokio::test]
async fn incompatible_work_is_not_actuated() {
    let queue = WorkQueue::new(None).await.expect("work queue must initialize");

    queue
        .add_work(WorkItem {
            id: "requires-rust".to_string(),
            priority: 100.0,
            requirements: vec!["rust".to_string()],
            estimated_duration_ms: 1,
            created_at: SystemTime::now(),
        })
        .await
        .expect("work insertion must succeed");

    let incompatible = AgentSpec {
        id: "typescript-only".to_string(),
        role: "worker".to_string(),
        capacity: 1.0,
        specializations: vec!["typescript".to_string()],
        work_capacity: Some(1),
    };

    assert!(
        queue
            .get_work_for_agent(&incompatible)
            .await
            .expect("capability check must succeed")
            .is_none(),
        "work outside the agent capability set must remain unclaimed"
    );

    let compatible = AgentSpec {
        id: "rust-worker".to_string(),
        role: "worker".to_string(),
        capacity: 1.0,
        specializations: vec!["rust".to_string()],
        work_capacity: Some(1),
    };

    let admitted = queue
        .get_work_for_agent(&compatible)
        .await
        .expect("compatible claim must succeed")
        .expect("compatible work must be claimable");

    assert_eq!(admitted.id, "requires-rust");
}

#[tokio::test]
async fn duplicate_registration_is_refused_by_the_real_coordinator() {
    let telemetry = Arc::new(
        TelemetryManager::with_config(TelemetryConfig {
            mode: TelemetryMode::Disabled,
            enable_timing: false,
            ..TelemetryConfig::default()
        })
        .await
        .expect("disabled telemetry must initialize"),
    );
    let queue = Arc::new(WorkQueue::new(None).await.expect("work queue must initialize"));
    let coordinator = AgentCoordinator::new(telemetry, queue)
        .await
        .expect("coordinator must initialize");

    let spec = agent(10_000);
    coordinator
        .register_agent(spec.clone())
        .await
        .expect("first registration must be admitted");

    let duplicate = coordinator.register_agent(spec).await;
    assert!(
        matches!(duplicate, Err(SwarmError::AlreadyExists(_))),
        "the second registration of the same identity must be a typed refusal"
    );
}
