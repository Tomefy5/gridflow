use criterion::{criterion_group, criterion_main, Criterion};
use gridflow_scheduler::{TaskDistributor, TaskQueue, WorkerRegistry};
use std::sync::Arc;

fn distributor_benchmark(c: &mut Criterion) {
    c.bench_function("distributor_creation", |b| {
        b.iter(|| {
            let queue = Arc::new(TaskQueue::new());
            let registry = Arc::new(WorkerRegistry::new());
            TaskDistributor::new(queue, registry)
        })
    });
}

criterion_group!(benches, distributor_benchmark);
criterion_main!(benches);
