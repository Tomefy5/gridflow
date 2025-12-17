use gridflow_scheduler::task_queue;
pub fn main() {
    let task = task_queue::Task::new(
        "job-123".to_string(),
        0,
        vec![0x00, 0x01], // wasm_module
        vec![1, 2, 3],
    );

    println!("{:?}", task.wasm_module);
    for byte in &task.wasm_module {
        print!("{:02x}", byte);
    }
}
