use criterion::{criterion_group, criterion_main, Criterion};
use day11::{
    calc_lookup_tables, parse_stones, solve_lookup, solve_memoization, solve_smart, solve_smart_fast, solve_stupid
};
use std::hint::black_box;

fn day11_benches(c: &mut Criterion) {
    let input_stones = parse_stones("../input.txt");

    let lookup_tables_25_10 = calc_lookup_tables::<10>(25);
    let lookup_tables_75_10 = calc_lookup_tables::<10>(75);
    let lookup_tables_75_16 = calc_lookup_tables::<16>(75);
    let lookup_tables_75_32 = calc_lookup_tables::<32>(75);
    let lookup_tables_75_64 = calc_lookup_tables::<64>(75);

    c.bench_function("task1_stupid", |b| {
        b.iter(|| black_box(solve_stupid::<25>(black_box(input_stones.clone()))))
    });
    c.bench_function("task1_smart", |b| {
        b.iter(|| black_box(solve_smart::<25>(black_box(input_stones.clone()))))
    });
    c.bench_function("task1_smart_fast", |b| {
        b.iter(|| black_box(solve_smart_fast::<25>(black_box(input_stones.clone()))))
    });
    c.bench_function("task1_lookup", |b| {
        b.iter(|| {
            black_box(solve_lookup(
                black_box(input_stones.clone()),
                black_box(25),
                black_box(&lookup_tables_25_10),
            ))
        })
    });
    c.bench_function("task1_memoization", |b| {
        b.iter(|| {
            black_box(solve_memoization(
                black_box(input_stones.clone()),
                black_box(25),
            ))
        })
    });
    c.bench_function("task2_lookup_10", |b| {
        b.iter(|| {
            black_box(solve_lookup(
                black_box(input_stones.clone()),
                black_box(75),
                black_box(&lookup_tables_75_10),
            ))
        })
    });
    c.bench_function("task2_lookup_16", |b| {
        b.iter(|| {
            black_box(solve_lookup(
                black_box(input_stones.clone()),
                black_box(75),
                black_box(&lookup_tables_75_16),
            ))
        })
    });
    c.bench_function("task2_lookup_32", |b| {
        b.iter(|| {
            black_box(solve_lookup(
                black_box(input_stones.clone()),
                black_box(75),
                black_box(&lookup_tables_75_32),
            ))
        })
    });
    c.bench_function("task2_lookup_64", |b| {
        b.iter(|| {
            black_box(solve_lookup(
                black_box(input_stones.clone()),
                black_box(75),
                black_box(&lookup_tables_75_64),
            ))
        })
    });
    c.bench_function("task2_memoization", |b| {
        b.iter(|| {
            black_box(solve_memoization(
                black_box(input_stones.clone()),
                black_box(75),
            ))
        })
    });
}

fn lookup_tables_benches(c: &mut Criterion) {
    c.bench_function("lookup_gen_25_10", |b| {
        b.iter(|| black_box(calc_lookup_tables::<10>(black_box(25))))
    });
    c.bench_function("lookup_gen_75_10", |b| {
        b.iter(|| black_box(calc_lookup_tables::<10>(black_box(75))))
    });
    c.bench_function("lookup_gen_75_16", |b| {
        b.iter(|| black_box(calc_lookup_tables::<16>(black_box(75))))
    });
    c.bench_function("lookup_gen_75_32", |b| {
        b.iter(|| black_box(calc_lookup_tables::<32>(black_box(75))))
    });
    c.bench_function("lookup_gen_75_64", |b| {
        b.iter(|| black_box(calc_lookup_tables::<64>(black_box(75))))
    });
}

criterion_group!(benches, day11_benches, lookup_tables_benches);
criterion_main!(benches);
