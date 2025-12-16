use gridflow_scheduler::{TaskDistributor, TaskQueue, WorkerRegistry};
use std::sync::Arc;

#[tokio::test]
async fn test_distributor_creation() {
    let queue = Arc::new(TaskQueue::new());
    let registry = Arc::new(WorkerRegistry::new());
    let _distributor = TaskDistributor::new(queue, registry);
}
