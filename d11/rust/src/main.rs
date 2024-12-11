use std::time::Instant;

use day11::{
    calc_lookup_tables, parse_stones, solve_lookup, solve_smart, solve_smart_fast, solve_stupid, solve_memoization
};

fn main() {
    let stones = parse_stones("../input.txt");
    // let stones = parse_stones("../example.txt");

    println!("Initial: {stones:?}");

    // let start_a_stupid = Instant::now();
    // let output_a_stupid = solve_stupid::<25>(stones.clone());
    // let elapsed_stupid_a = start_a_stupid.elapsed();

    let start_a_smart = Instant::now();
    let output_a_smart = solve_smart::<25>(stones.clone());
    let elapsed_a_smart = start_a_smart.elapsed();

    let start_a_smart_fast = Instant::now();
    let output_a_smart_fast = solve_smart_fast::<25>(stones.clone());
    let elapsed_a_smart_fast = start_a_smart_fast.elapsed();

    let lookup_tables = calc_lookup_tables::<10>(75);
    let start_a_lookup = Instant::now();
    let output_a_lookup = solve_lookup(stones.clone(), 25, &lookup_tables);
    let elapsed_a_lookup = start_a_lookup.elapsed();

    let start_a_memoization = Instant::now();
    let output_a_memoization = solve_memoization(stones.clone(), 25);
    let elapsed_a_memoization = start_a_memoization.elapsed();

    let start_b_lookup = Instant::now();
    let output_b_lookup = solve_lookup(stones.clone(), 75, &lookup_tables);
    let elapsed_b_lookup = start_b_lookup.elapsed();

    let start_b_memoization = Instant::now();
    let output_b_memoization = solve_memoization(stones.clone(), 75);
    let elapsed_b_memoization = start_b_memoization.elapsed();

    // assert_eq!(output_a_stupid, output_a_smart);
    assert_eq!(output_a_smart, output_a_smart_fast);
    assert_eq!(output_a_smart, output_a_lookup);
    assert_eq!(output_a_smart, output_a_memoization);

    assert_eq!(output_b_lookup, output_b_memoization);

    println!("Task1: {output_a_smart}");
    println!("Task2: {output_b_lookup}");

    // println!("Task 1 (stupid): {}µs", elapsed_stupid_a.as_micros());
    println!("Task 1 (smart):\t\t{}µs", elapsed_a_smart.as_micros());
    println!(
        "Task 1 (smart-fast):\t{}µs",
        elapsed_a_smart_fast.as_micros()
    );
    println!("Task 1 (lookup):\t{}µs", elapsed_a_lookup.as_micros());
    println!("Task 1 (memoization):\t{}µs", elapsed_a_memoization.as_micros());
    println!("Task 2 (lookup):\t{}µs", elapsed_b_lookup.as_micros());
    println!("Task 2 (memoization):\t{}µs", elapsed_b_memoization.as_micros());
}
