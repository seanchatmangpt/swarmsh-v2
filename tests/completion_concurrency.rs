use std::collections::HashSet;
use std::sync::Arc;
use std::time::SystemTime;

use swarmsh_v2::coordination::{AgentSpec, WorkItem, WorkQueue};
use tokio::sync::Barrier;

fn agent(id: usize) -> AgentSpec {
    AgentSpec {
        id: format!("completion-agent-{id}"),
        role: "completion-test-worker".to_string(),
        capacity: 1.0,
        specializations: Vec::new(),
        work_capacity: Some(16),
    }
}

fn work(id: &str, priority: f64, requirements: Vec<&str>) -> WorkItem {
    WorkItem {
        id: id.to_string(),
        priority,
        requirements: requirements.into_iter().map(str::to_string).collect(),
        estimated_duration_ms: 1,
        created_at: SystemTime::now(),
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn concurrent_claims_are_unique_and_exhaustive() {
    const WORK_ITEMS: usize = 256;
    const AGENTS: usize = 64;

    let queue = Arc::new(WorkQueue::new(None).await.expect("queue"));
    for id in 0..WORK_ITEMS {
        queue
            .add_work(work(
                &format!("completion-work-{id}"),
                (WORK_ITEMS - id) as f64,
                Vec::new(),
            ))
            .await
            .expect("add work");
    }

    let barrier = Arc::new(Barrier::new(AGENTS));
    let mut tasks = Vec::with_capacity(AGENTS);

    for id in 0..AGENTS {
        let queue = Arc::clone(&queue);
        let barrier = Arc::clone(&barrier);
        tasks.push(tokio::spawn(async move {
            let spec = agent(id);
            barrier.wait().await;
            let mut claimed = Vec::new();
            loop {
                match queue
                    .get_work_for_agent(&spec)
                    .await
                    .expect("claim must not fail")
                {
                    Some(work) => claimed.push(work.id),
                    None => break,
                }
            }
            claimed
        }));
    }

    let mut all_claims = Vec::with_capacity(WORK_ITEMS);
    for task in tasks {
        all_claims.extend(task.await.expect("agent task"));
    }

    let unique: HashSet<_> = all_claims.iter().cloned().collect();
    assert_eq!(
        all_claims.len(),
        WORK_ITEMS,
        "every work item is claimed once"
    );
    assert_eq!(unique.len(), WORK_ITEMS, "no work item is claimed twice");
}

#[tokio::test]
async fn capability_mismatch_does_not_consume_work() {
    let queue = WorkQueue::new(None).await.expect("queue");
    queue
        .add_work(work("specialized-work", 1.0, vec!["rust"]))
        .await
        .expect("add work");

    let mut incapable = agent(1);
    incapable.specializations = vec!["shell".to_string()];
    assert!(queue
        .get_work_for_agent(&incapable)
        .await
        .expect("claim")
        .is_none());

    let mut capable = agent(2);
    capable.specializations = vec!["rust".to_string()];
    let claimed = queue
        .get_work_for_agent(&capable)
        .await
        .expect("claim")
        .expect("work preserved for capable agent");
    assert_eq!(claimed.id, "specialized-work");
}

#[tokio::test]
async fn highest_priority_compatible_work_is_claimed_first() {
    let queue = WorkQueue::new(None).await.expect("queue");
    queue
        .add_work(work("low", 1.0, Vec::new()))
        .await
        .expect("low");
    queue
        .add_work(work("high", 100.0, Vec::new()))
        .await
        .expect("high");
    queue
        .add_work(work("middle", 10.0, Vec::new()))
        .await
        .expect("middle");

    let spec = agent(3);
    let first = queue
        .get_work_for_agent(&spec)
        .await
        .expect("claim")
        .expect("first work");
    let second = queue
        .get_work_for_agent(&spec)
        .await
        .expect("claim")
        .expect("second work");
    let third = queue
        .get_work_for_agent(&spec)
        .await
        .expect("claim")
        .expect("third work");

    assert_eq!([first.id, second.id, third.id], ["high", "middle", "low"]);
}

#[tokio::test]
async fn duplicate_work_ids_are_refused_at_admission() {
    let queue = WorkQueue::new(None).await.expect("queue");
    queue
        .add_work(work("duplicate", 1.0, Vec::new()))
        .await
        .expect("first admission");
    let error = queue
        .add_work(work("duplicate", 2.0, Vec::new()))
        .await
        .expect_err("duplicate id must be refused");
    assert!(error.to_string().contains("already exists"));
}

#[tokio::test]
async fn non_finite_priority_is_refused() {
    let queue = WorkQueue::new(None).await.expect("queue");
    for priority in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let error = queue
            .add_work(work("bad-priority", priority, Vec::new()))
            .await
            .expect_err("non-finite priority must be refused");
        assert!(error.to_string().contains("finite"));
    }
}
