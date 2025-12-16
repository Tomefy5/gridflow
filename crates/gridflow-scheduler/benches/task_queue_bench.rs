use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gridflow_scheduler::TaskQueue;
use gridflow_contracts::Task;

fn task_queue_benchmark(c: &mut Criterion) {
    let queue = TaskQueue::new();
    
    c.bench_function("task_queue_push", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap()).iter(|| async {
            let task = Task {
                task_id: "bench".to_string(),
                task_type: "compute".to_string(),
                input_data: vec![1, 2, 3],
                priority: 1,
            };
            queue.push(black_box(task)).await;
        })
    });
}

criterion_group!(benches, task_queue_benchmark);
criterion_main!(benches);
