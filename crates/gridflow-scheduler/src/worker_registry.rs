use gridflow_contracts::WorkerInfo;
use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct WorkerRegistry {
    workers: RwLock<HashMap<String, WorkerInfo>>,
}

impl WorkerRegistry {
    pub fn new() -> Self {
        Self {
            workers: RwLock::new(HashMap::new()),
        }
    }

    pub async fn register(&self, worker: WorkerInfo) {
        let mut workers = self.workers.write().await;
        workers.insert(worker.worker_id.clone(), worker);
    }

    pub async fn get_available(&self) -> Vec<WorkerInfo> {
        let workers = self.workers.read().await;
        workers.values().cloned().collect()
    }
}
