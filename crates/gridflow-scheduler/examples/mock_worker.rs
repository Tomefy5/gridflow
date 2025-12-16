use gridflow_contracts::WorkerInfo;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Mock worker starting...");
    
    let worker_info = WorkerInfo {
        worker_id: "mock-worker-1".to_string(),
        capabilities: vec!["compute".to_string(), "io".to_string()],
        last_seen: 0,
    };
    
    loop {
        sleep(Duration::from_secs(5)).await;
        println!("Worker heartbeat: {}", worker_info.worker_id);
    }
}
