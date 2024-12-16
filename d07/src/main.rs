use std::io::Write;

use regex::Regex;

fn solve_a_helper(value_left: u64, inputs_left: &[u64]) -> bool {
    if inputs_left.len() == 0 {
        println!("Finished recursion with inputs_left.len() == 0!");
        return value_left == 0;
    }

    if inputs_left.len() == 1 {
        return value_left == inputs_left[0];
    }

    let (last_input, rest_inputs) = inputs_left.split_last().unwrap();

    if value_left % last_input == 0 {
        if solve_a_helper(value_left / last_input, rest_inputs) {
            return true;
        }
    }

    if value_left <= *last_input {
        return false;
    }

    solve_a_helper(value_left - last_input, rest_inputs)
}

fn find_next_power_10(mut num: u64) -> u64 {
    assert!(num != 0);

    let mut power = 1u64;
    while num > 0 {
        num /= 10;
        power *= 10;
    }

    power
}

const CLEAR_LINE: &str =
    "\r                                                                                ";

fn solve_b_helper(value_left: u64, inputs_left: &[u64]) -> bool {
    if inputs_left.len() == 0 {
        println!("Finished recursion with inputs_left.len() == 0!");
        return value_left == 0;
    }

    if inputs_left.len() == 1 {
        print!("\r{} ", inputs_left[0]);
        if value_left == inputs_left[0] {
            print!("\r{} ", inputs_left[0]);
        }
        return value_left == inputs_left[0];
    }

    let (last_input, rest_inputs) = inputs_left.split_last().unwrap();

    if value_left % last_input == 0 {
        if solve_b_helper(value_left / last_input, rest_inputs) {
            print!("* {last_input} ");
            return true;
        }

        print!("{}", CLEAR_LINE);
    }

    if value_left > *last_input {
        if solve_b_helper(value_left - last_input, rest_inputs) {
            print!("+ {last_input} ");
            return true;
        }
        print!("{}", CLEAR_LINE);
    }

    let next_power_10_last_input = find_next_power_10(*last_input);
    if value_left % next_power_10_last_input == *last_input {
        if solve_b_helper(value_left / next_power_10_last_input, rest_inputs) {
            print!("|| {last_input} ");
            return true;
        }
    }

    false
}

fn solve_a(calibrations: &Vec<Calibration>) -> u64 {
    calibrations
        .iter()
        .filter(|calibration| solve_a_helper(calibration.test_value, &calibration.inputs))
        .map(|calibration| calibration.test_value)
        .sum()
}

fn solve_b(calibrations: &Vec<Calibration>) -> u64 {
    let res = calibrations
        .iter()
        .filter(|calibration| {
            let res = solve_b_helper(calibration.test_value, &calibration.inputs);
            if res {
                println!("= {}", calibration.test_value);
            }
            res
        })
        .map(|calibration| calibration.test_value)
        .sum();

    res
}

#[derive(Clone)]
struct Calibration {
    test_value: u64,
    inputs: Vec<u64>,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    // let input = std::fs::read_to_string("example.txt").unwrap();

    let num_re = Regex::new(r"(\d+)").unwrap();

    let mut calibrations: Vec<Calibration> = input
        .lines()
        .map(|line| {
            let mut nums = num_re.find_iter(line);

            let test_value = nums.next().unwrap().as_str().parse().unwrap();
            let inputs = nums.map(|num| num.as_str().parse().unwrap()).collect();

            Calibration { test_value, inputs }
        })
        .collect();

    // calibrations.retain(|calibration| calibration.test_value == 156);

    let output_a = solve_a(&calibrations);
    let output_b = solve_b(&calibrations);

    println!("Task1: {output_a}");
    println!("Task2: {output_b}");
}
