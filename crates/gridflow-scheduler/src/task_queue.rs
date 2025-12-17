use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, VecDeque},
    sync::{Arc, Mutex}, // Mutex pour la sécurité thread
    time::SystemTime,
};
use uuid::Uuid;

pub type TaskId = Uuid;
pub type JobId = String;
pub type WorkerId = String;

// Global const for Task
pub const DEFAULT_TIMEOUT_MS: u32 = 60_000;
pub const DEFAULT_PRIORITY: u32 = 50;
pub const DEFAULT_ESTIMATED_DURATION_MS: u32 = 1000;
pub const DEFAULT_MIN_RAM_GB: f32 = 0.5;

// Global cons for TaskQueue
pub const DEFAULT_MAX_QUEUE_SIZE: usize = 10_000;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum TaskStatus {
    Pending,
    Assigned(WorkerId),
    Running,
    Completed,
    Failed(String),
    Timeout,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Task {
    // Identity
    pub id: TaskId,
    pub job_id: JobId,
    pub sequence: u32,
    // payload
    pub wasm_module: Arc<Vec<u8>>,
    pub input_data: Arc<Vec<u8>>,
    // scheduling metedata
    pub timeout_ms: u32,
    pub priority: u32,
    pub retry_count: u32,
    pub estimated_duration: u32,
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
    pub fn new(
        job_id: JobId,
        sequence: u32,
        wasm_module: Arc<Vec<u8>>,
        input_data: Arc<Vec<u8>>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            job_id,
            sequence,
            wasm_module,
            input_data,
            timeout_ms: DEFAULT_TIMEOUT_MS,
            priority: DEFAULT_PRIORITY,
            retry_count: 0,
            estimated_duration: DEFAULT_ESTIMATED_DURATION_MS,
            min_ram_gb: DEFAULT_MIN_RAM_GB,
            preferred_worker_id: None,
            task_status: TaskStatus::Pending,
            created_at: SystemTime::now(),
            assigned_at: None,
            completed_at: None,
        }
    }
}

#[derive(Debug)]
pub struct TaskTicket {
    pub id: TaskId,
    pub priority: u32,
    pub estimated_duration: u32,
}

impl Ord for TaskTicket {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_score = (self.priority as u64) * (other.estimated_duration as u64);
        let other_score = (self.estimated_duration as u64) * (other.priority as u64);
        self_score.cmp(&other_score)
    }
}

impl PartialOrd for TaskTicket {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // CORRECTION 4 : Syntaxe correcte (pas de &other)
        Some(self.cmp(other))
    }
}

impl PartialEq for TaskTicket {
    fn eq(&self, other: &Self) -> bool {
        (self.priority as u64) * (other.estimated_duration as u64)
            == (self.estimated_duration as u64) * (other.priority as u64)
    }
}

impl Eq for TaskTicket {}

#[derive(Debug)]
pub struct TaskQueue {
    tasks: DashMap<TaskId, Task>,
    queue: Mutex<BinaryHeap<TaskTicket>>,
    job_index: DashMap<JobId, Vec<TaskId>>,
    worker_index: DashMap<WorkerId, Vec<TaskId>>,
    failed_index: Mutex<VecDeque<TaskId>>,
    max_queue_size: usize,
}

impl TaskQueue {
    pub fn new() -> Self {
        Self {
            tasks: DashMap::new(),
            queue: Mutex::new(BinaryHeap::new()),
            job_index: DashMap::new(),
            worker_index: DashMap::new(),
            failed_index: Mutex::new(VecDeque::new()),
            max_queue_size: DEFAULT_MAX_QUEUE_SIZE,
        }
    }
    pub fn push(&self, task: Task) {
        let ticket = TaskTicket {
            id: task.id,
            priority: task.priority,
            estimated_duration: task.estimated_duration,
        };
        self.tasks.insert(task.id, task);
        self.queue.lock().unwrap().push(ticket);
    }

    pub fn pop(&self) -> Option<Task> {
        let ticket = self.queue.lock().unwrap().pop();
        if let Some(ticket) = ticket {
            if let Some((_, task)) = self.tasks.remove(&ticket.id) {
                return Some(task);
            }
        }
        None
    }
}
