use gridflow_scheduler::Scheduler;

#[tokio::test]
async fn test_scheduler_initialization() {
    let scheduler = Scheduler::new();
    assert_eq!(scheduler.task_queue.len().await, 0);
}
