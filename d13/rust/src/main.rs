use std::time::Instant;

use regex::Regex;
use z3::ast::{Ast, Int};

struct Machine {
    button_a: [u64; 2],
    button_b: [u64; 2],
    prize: [u64; 2],
}

fn solve_a(machines: &Vec<Machine>) -> u64 {
    let mut sum = 0;
    for machine in machines {
        let mut solution: Option<u64> = None;
        for num_b in 0..100 {
            let b_x = num_b * machine.button_b[0];
            let b_y = num_b * machine.button_b[1];
            if b_x > machine.prize[0] || b_y > machine.prize[1] {
                break;
            }
            for num_a in 0..100 {
                let sum_x = b_x + num_a * machine.button_a[0];
                let sum_y = b_y + num_a * machine.button_a[1];

                if sum_x > machine.prize[0] || sum_y > machine.prize[1] {
                    break;
                }
                if sum_x == machine.prize[0] && sum_y == machine.prize[1] {
                    let presses = num_a * 3 + num_b;
                    if let Some(sol_presses) = solution {
                        if sol_presses > presses {
                            solution = Some(presses);
                        }
                    } else {
                        solution = Some(presses);
                    }
                    break;
                }
            }
        }

        if let Some(presses) = solution {
            sum += presses;
        }
    }

    sum
}

fn solve_b(mut machines: Vec<Machine>) -> u64 {
    let mut total_token_num = 0;
    // adjust for measurement error
    for machine in &mut machines {
        machine.prize[0] += 10000000000000;
        machine.prize[1] += 10000000000000;

        let ctx = z3::Context::new(&z3::Config::new());
        let o = z3::Optimize::new(&ctx);
        let num_a = Int::new_const(&ctx, "num_a");
        let num_b = Int::new_const(&ctx, "num_b");
        let prize_x_var = machine.button_a[0] * num_a.clone() + machine.button_b[0] * num_b.clone();
        let prize_y_var = machine.button_a[1] * num_a.clone() + machine.button_b[1] * num_b.clone();
        //  == machine.prize[0];
        o.assert(&prize_x_var._eq(&Int::from_u64(&ctx, machine.prize[0])));
        o.assert(&prize_y_var._eq(&Int::from_u64(&ctx, machine.prize[1])));
        o.assert(&(num_a.ge(&Int::from_u64(&ctx, 0))));
        o.assert(&(num_b.ge(&Int::from_u64(&ctx, 0))));
        let token_num = (3u64 * num_a) + num_b;
        o.minimize(&token_num);
        total_token_num += match o.check(&[]) {
            z3::SatResult::Unsat => continue,
            z3::SatResult::Unknown => unreachable!(),
            z3::SatResult::Sat => o
                .get_model()
                .unwrap()
                .eval(&token_num, false)
                .unwrap()
                .as_u64()
                .unwrap(),
        };
    }

    total_token_num
}

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    // let input = std::fs::read_to_string("../example.txt").unwrap();

    let machine_re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    let machines: Vec<Machine> = input
        .split("\n\n")
        .map(|machine| {
            if let Some(caps) = machine_re.captures(machine) {
                let button_a_x = caps[1].parse().unwrap();
                let button_a_y = caps[2].parse().unwrap();
                let button_b_x = caps[3].parse().unwrap();
                let button_b_y = caps[4].parse().unwrap();
                let prize_x = caps[5].parse().unwrap();
                let prize_y = caps[6].parse().unwrap();
                Machine {
                    button_a: [button_a_x, button_a_y],
                    button_b: [button_b_x, button_b_y],
                    prize: [prize_x, prize_y],
                }
            } else {
                unreachable!()
            }
        })
        .collect();

    let start_a = Instant::now();
    let output_a = solve_a(&machines);
    let elapsed_a = start_a.elapsed();
    let start_b = Instant::now();
    let output_b = solve_b(machines);
    let elapsed_b = start_b.elapsed();
    println!("Task1: {output_a}");
    println!("Task1: {output_b}");

    println!("Task 1 too: {}ms", elapsed_a.as_millis());
    println!("Task 2 too: {}ms", elapsed_b.as_millis());
}
