use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

pub type TaskId = String;
pub type JobId = String;
pub type WorkerId = String;

// Global const for Task
pub const DEFAULT_TIMEOUT_MS: u32 = 60_000;
pub const DEFAULT_PRIORITY: u32 = 50;
pub const DEFAULT_ESTIMATED_DURATION_MS: u32 = 1000;
pub const DEFAULT_MIN_RAM_GB: f32 = 0.5;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum TaskStatus {
    Pending,
    Assigned(WorkerId),
    Running,
    Completed,
    Failed(String), // contained error message
    Timeout,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    // Identity
    pub id: TaskId,
    pub job_id: JobId,
    pub sequence: u32,
    // payload
    pub wasm_module: Vec<u8>,
    pub input_data: Vec<u8>,
    // scheduling metedata
    pub timeout_ms: u32,
    pub priority: u32, // brut priority (0 = low, 50 = medium, 100 = high, 1000 = critical)
    pub retry_count: u32,
    // placement metadata
    pub min_ram_gb: f32,
    pub preferred_worker_id: Option<WorkerId>,
    // task's state
    pub task_status: TaskStatus,
    pub created_at: SystemTime,
    pub assigned_at: Option<SystemTime>,
    pub completed_at: Option<SystemTime>,
}

impl Task {
    // Create task with reasonable default value
    pub fn new(job_id: JobId, sequence: u32, wasm_module: Vec<u8>, input_data: Vec<u8>) -> Self {
        Self {
            id: format!("task-{}", Uuid::new_v4()),
            job_id,
            sequence,
            wasm_module,
            input_data,
            timeout_ms: DEFAULT_TIMEOUT_MS,
            priority: DEFAULT_PRIORITY,
            retry_count: 0,
            min_ram_gb: DEFAULT_MIN_RAM_GB,
            preferred_worker_id: None,
            task_status: TaskStatus::Pending,
            created_at: SystemTime::now(),
            assigned_at: None,
            completed_at: None,
        }
    }
}
