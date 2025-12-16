use gridflow_scheduler::ResultAggregator;
use gridflow_contracts::TaskResult;

#[tokio::test]
async fn test_aggregator_add_result() {
    let aggregator = ResultAggregator::new();
    let result = TaskResult {
        task_id: "task-1".to_string(),
        worker_id: "worker-1".to_string(),
        output_data: vec![1, 2, 3],
        execution_time_ms: 100,
    };
    
    aggregator.add_result(result).await;
}
