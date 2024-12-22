use std::collections::{HashMap, HashSet};

fn next_secret_number(mut secret: u64) -> u64 {
    secret ^= (secret.wrapping_mul(64)) % 0x1000000;
    secret ^= secret / 32;
    (secret ^ secret.wrapping_mul(2048)) % 0x1000000
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    // let input = std::fs::read_to_string("example.txt").unwrap();

    let secret_nums: Vec<u64> = input.lines().map(|line| line.parse().unwrap()).collect();

    let task1 = secret_nums
        .iter()
        .map(|&initial_secret| {
            let mut secret = initial_secret;
            for _ in 0..2000 {
                secret = next_secret_number(secret);
            }
            secret
        })
        .sum::<u64>();

    let mut max_seq = None;
    let mut max_seq_val = u64::MIN;
    let mut sequence_sums = HashMap::new();

    for initial_secret in secret_nums {
        let mut visited = HashSet::new();
        let mut first;
        let mut second = initial_secret;
        let mut third = next_secret_number(second);
        let mut fourth = next_secret_number(third);
        let mut fifth = next_secret_number(fourth);

        for _ in 0..(2000 - 4) {
            first = second;
            second = third;
            third = fourth;
            fourth = fifth;
            fifth = next_secret_number(fifth);
            let change_a = ((second % 10) as i8) - ((first % 10) as i8);
            let change_b = ((third % 10) as i8) - ((second % 10) as i8);
            let change_c = ((fourth % 10) as i8) - ((third % 10) as i8);
            let change_d = ((fifth % 10) as i8) - ((fourth % 10) as i8);

            let seq = [change_a, change_b, change_c, change_d];

            if visited.contains(&seq) {
                continue;
            }

            visited.insert(seq);

            let seq_val = sequence_sums.entry(seq).or_insert(0);

            *seq_val += fifth % 10;

            if max_seq_val < *seq_val {
                max_seq_val = *seq_val;
                max_seq = Some(seq);
            }
        }
    }

    println!("Task1: {task1}");
    println!("Task2: {max_seq_val}, {max_seq:?}");
}
