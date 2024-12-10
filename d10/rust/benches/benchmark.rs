use criterion::{criterion_group, criterion_main, Criterion};
use day10::{parse, read_input, solve};
use std::hint::black_box;


fn day10_benches(c: &mut Criterion) {
    c.bench_function("read_input", |b| {
        b.iter(|| black_box(read_input(black_box("../input.txt"))))
    });
    let input = read_input("../input.txt");
    c.bench_function("parse", |b| b.iter(|| black_box(parse(black_box(&input)))));
    let (map, trailheads) = parse(&input);
    c.bench_function("solve", |b| {
        b.iter(|| black_box(solve(black_box(&map), black_box(&trailheads))))
    });
}

criterion_group!(benches, day10_benches);
criterion_main!(benches);
