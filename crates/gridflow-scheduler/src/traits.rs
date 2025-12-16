use async_trait::async_trait;
use gridflow_contracts::{ProtocolMessage, WorkerInfo, Task, TaskResult};

#[async_trait]
pub trait NetworkTransport: Send + Sync {
    async fn send_message(&self, msg: ProtocolMessage) -> Result<(), Box<dyn std::error::Error>>;
    async fn receive_message(&self) -> Result<ProtocolMessage, Box<dyn std::error::Error>>;
}

#[async_trait]
pub trait WASMInfo: Send + Sync {
    async fn get_capabilities(&self) -> Vec<String>;
    async fn validate_task(&self, task: &Task) -> bool;
}
