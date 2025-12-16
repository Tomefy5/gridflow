use crate::TaskQueue;
use crate::WorkerRegistry;
use crate::TaskDistributor;
use crate::ResultAggregator;
use std::sync::Arc;

pub struct Scheduler {
    pub task_queue: Arc<TaskQueue>,
    pub worker_registry: Arc<WorkerRegistry>,
    pub distributor: Arc<TaskDistributor>,
    pub aggregator: Arc<ResultAggregator>,
}

impl Scheduler {
    pub fn new() -> Self {
        let task_queue = Arc::new(TaskQueue::new());
        let worker_registry = Arc::new(WorkerRegistry::new());
        let distributor = Arc::new(TaskDistributor::new(
            task_queue.clone(),
            worker_registry.clone(),
        ));
        let aggregator = Arc::new(ResultAggregator::new());

        Self {
            task_queue,
            worker_registry,
            distributor,
            aggregator,
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: event loop principal
        Ok(())
    }
}
