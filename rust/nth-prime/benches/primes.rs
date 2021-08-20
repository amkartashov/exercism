use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nth_prime as np;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("prime 10th", |b| b.iter(|| np::nth(black_box(10))));
    c.bench_function("prime 100th", |b| b.iter(|| np::nth(black_box(100))));
    c.bench_function("prime 1000th", |b| b.iter(|| np::nth(black_box(1000))));
    c.bench_function("prime 10_000th", |b| b.iter(|| np::nth(black_box(10_000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
