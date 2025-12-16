use gridflow_contracts::TaskResult;
use std::collections::HashMap;
use tokio::sync::Mutex;

pub struct ResultAggregator {
    results: Mutex<HashMap<String, TaskResult>>,
}

impl ResultAggregator {
    pub fn new() -> Self {
        Self {
            results: Mutex::new(HashMap::new()),
        }
    }

    pub async fn add_result(&self, result: TaskResult) {
        let mut results = self.results.lock().await;
        results.insert(result.task_id.clone(), result);
    }
}
