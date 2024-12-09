use regex::Regex;

const MUL_PATTERN: &str = r"mul\((\d+),(\d+)\)";
const START_MUL_PATTERN: &str = r"^mul\((\d+),(\d+)\)";

fn solve_a(input: &str) -> u64 {
    let re = Regex::new(MUL_PATTERN).unwrap();

    re
        .captures_iter(input)
        .map(|cap| {
            let n: u64 = cap[1].parse().unwrap();
            let m: u64 = cap[2].parse().unwrap();
            n * m
        })
        .sum()
}

fn solve_b(input: &str) -> u64 {
    let mut enabled = true;
    let mut idx = 0;

    let re = Regex::new(START_MUL_PATTERN).unwrap();

    let mut sum = 0;

    while idx < input.len() {
        if input[idx..].starts_with("don't") {
            enabled = false;
            idx += "don't".len();
            continue;   
        }
        if input[idx..].starts_with("do") {
            enabled = true;
            idx += "do".len();
            continue;
        }
        if enabled && input[idx..].starts_with("mul(") {
            if let Some(cap) = re.captures(&input[idx..]) {
                let n: u64 = cap[1].parse().unwrap();
                let m: u64 = cap[2].parse().unwrap();
                sum += n * m;
                idx += cap.len();
                continue;
            }
        }
        idx += 1;
    }

    sum
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    
    let output_a = solve_a(&input);
    let output_b = solve_b(&input);

    println!("Task1: {output_a}");
    println!("Task2: {output_b}");
}
