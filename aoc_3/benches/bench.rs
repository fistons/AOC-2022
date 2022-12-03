use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc_3::{part1, part1_hashset};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("loop version", |b| b.iter(|| part1(black_box("input.txt"))));
    c.bench_function("hashset version", |b| {
        b.iter(|| part1_hashset(black_box("input.txt")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
