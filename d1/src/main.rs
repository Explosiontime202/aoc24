use std::{collections::HashMap, fs, iter::zip};

fn solve_a(mut a_list: Vec<u64>, mut b_list: Vec<u64>) -> u64 {
    a_list.sort();
    b_list.sort();
    zip(a_list.iter(), b_list.iter())
        .map(|(&a, &b)| a.abs_diff(b))
        .sum()
}

fn solve_b(a_list: Vec<u64>, b_list: Vec<u64>) -> u64 {
    let mut b_counts = HashMap::new();
    for b in b_list {
        *b_counts.entry(b).or_insert(0) += 1;
    }

    let mut sum = 0;
    for a in a_list {
        if b_counts.contains_key(&a) {
            sum += *b_counts.get(&a).unwrap() * a;
        }
    }

    sum
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut a_list = Vec::new();
    let mut b_list = Vec::new();

    for line in input.lines() {
        let line = line.split_ascii_whitespace().collect::<Vec<_>>();
        assert!(line.len() == 2);
        let a: u64 = line[0].parse().unwrap();
        let b: u64 = line[1].parse().unwrap();

        a_list.push(a);
        b_list.push(b);
    }

    let output_a = solve_a(a_list.clone(), b_list.clone());
    let output_b = solve_b(a_list, b_list);

    println!("Task1 = {output_a}");
    println!("Task2 = {output_b}");
}
