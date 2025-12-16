use gridflow_scheduler::WorkerRegistry;
use gridflow_contracts::WorkerInfo;

#[tokio::test]
async fn test_worker_registry_register_and_get() {
    let registry = WorkerRegistry::new();
    let worker = WorkerInfo {
        worker_id: "worker-1".to_string(),
        capabilities: vec!["compute".to_string()],
        last_seen: 0,
    };
    
    registry.register(worker).await;
    let available = registry.get_available().await;
    assert_eq!(available.len(), 1);
}
