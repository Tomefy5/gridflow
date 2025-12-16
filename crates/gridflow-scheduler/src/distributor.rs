use crate::TaskQueue;
use crate::WorkerRegistry;
use std::sync::Arc;

pub struct TaskDistributor {
    task_queue: Arc<TaskQueue>,
    worker_registry: Arc<WorkerRegistry>,
}

impl TaskDistributor {
    pub fn new(task_queue: Arc<TaskQueue>, worker_registry: Arc<WorkerRegistry>) -> Self {
        Self {
            task_queue,
            worker_registry,
        }
    }

    pub async fn distribute_tasks(&self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: impl vraie logique de distribution
        Ok(())
    }
}
