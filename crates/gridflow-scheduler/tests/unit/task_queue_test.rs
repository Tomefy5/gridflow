use gridflow_scheduler::TaskQueue;
use gridflow_contracts::Task;

#[tokio::test]
async fn test_task_queue_push_pop() {
    let queue = TaskQueue::new();
    let task = Task {
        task_id: "test-1".to_string(),
        task_type: "compute".to_string(),
        input_data: vec![1, 2, 3],
        priority: 1,
    };
    
    queue.push(task.clone()).await;
    assert_eq!(queue.len().await, 1);
    
    let popped = queue.pop().await;
    assert!(popped.is_some());
    assert_eq!(queue.len().await, 0);
}
