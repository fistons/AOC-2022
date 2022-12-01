use aoc_1::{iterator_version, loop_version};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("loop version", |b| {
        b.iter(|| loop_version(black_box("input.txt".to_owned()), 1))
    });
    c.bench_function("iter version", |b| {
        b.iter(|| iterator_version(black_box("input.txt".to_owned()), 1))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
