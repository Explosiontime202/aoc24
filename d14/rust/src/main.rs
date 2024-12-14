use std::io::stdin;

use nalgebra::Vector2;
use regex::Regex;

#[derive(Debug, Clone)]
struct Robot {
    pos: Vector2<i64>,
    v: Vector2<i64>,
}

fn solve_a(robots: Vec<Robot>, width: u64, height: u64) -> u64 {
    let x_middle = width / 2;
    let y_middle = height / 2;
    let mut quadrants = [0; 4];
    for robot in &robots {
        // println!("{robot:?}");
        let mut end_pos = robot.pos + robot.v * 100;
        end_pos.x %= width as i64;
        end_pos.y %= height as i64;
        let end_x = (end_pos.x + width as i64) as u64 % (width as u64);
        let end_y = (end_pos.y + height as i64) as u64 % (height as u64);
        // println!("[{end_x}, {end_y}]");
        let x_quad = if end_x < x_middle {
            0
        } else if end_x > x_middle {
            2
        } else {
            continue;
        };

        let y_quad = if end_y < y_middle {
            0
        } else if end_y > y_middle {
            1
        } else {
            continue;
        };

        quadrants[x_quad + y_quad] += 1;
    }

    println!("{quadrants:?}");

    quadrants
        .into_iter()
        .reduce(|acc, elem| acc * elem)
        .unwrap()
}

// fn iterate_pics(mut robots: Vec<Robot>, width: u64, height: u64) -> u64 {
//     let mut input_buf = String::new();
//     let mut iteration = 0;
//     loop {
//         input_buf.clear();
//         stdin().read_line(&mut input_buf).unwrap();
//         if input_buf.starts_with("exit") {
//             return iteration;
//         }

//         iteration += 1;

//         let mut map = vec![vec![0; width as usize]; height as usize];
//         for robot in &mut robots {
//             robot.pos[0] = (robot.pos[0] + robot.v[0] + width as i64) % width as i64;
//             robot.pos[1] = (robot.pos[1] + robot.v[1] + width as i64) % width as i64;
//             map[robot.pos[1] as usize][robot.pos[0] as usize] += 1;
//         }

//         for r in 0..(height as usize) {
//             for c in 0..(width as usize) {
//                 if map[r][c] == 0 {
//                     print!(".");
//                 } else  {
//                     print!("{}", map[r][c]);
//                 }
//             }
//             println!("");
//         }

//     }
// }

fn find_chirstmas_tree(mut robots: Vec<Robot>, width: u64, height: u64) -> u64 {
    let mut input_buf = String::new();
    let mut iteration = 0;
    loop {
        input_buf.clear();
        stdin().read_line(&mut input_buf).unwrap();
        if input_buf.starts_with("exit") {
            return iteration;
        }

        loop {
            iteration += 1;

            let mut map = vec![vec![0; width as usize]; height as usize];
            for robot in &mut robots {
                robot.pos[0] = (robot.pos[0] + robot.v[0] + width as i64) % width as i64;
                robot.pos[1] = (robot.pos[1] + robot.v[1] + height as i64) % height as i64;
                map[robot.pos[1] as usize][robot.pos[0] as usize] += 1;
            }

            let mut num_vert_lines = 0;
            for r in 0..(height as usize) {
                if r + 4 >= height as usize {
                    break;
                }
                for c in 0..(width as usize) {
                    let mut is_line = true;
                    for r_dash in r..(r + 4) {
                        if map[r_dash][c] < 1 {
                            is_line = false;
                            break;
                        }
                    }
                    if is_line {
                        num_vert_lines += 1;
                    }
                }

                if num_vert_lines >= 2 {
                    break;
                }
            }

            if num_vert_lines >= 2 {
                for r in 0..(height as usize) {
                    for c in 0..(width as usize) {
                        if map[r][c] == 0 {
                            print!(".");
                        } else {
                            print!("{}", map[r][c]);
                        }
                    }
                    println!("");
                }
                break;
            }
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    // let input = std::fs::read_to_string("../example.txt").unwrap();

    let (width, height) = (101, 103);
    // let (width, height) = (11, 7);

    let robot_re = Regex::new(r"^p=(\d+),(\d+) v=(-?\d+),(-?\d+)$").unwrap();

    let robots: Vec<Robot> = input
        .lines()
        .filter_map(|line| {
            robot_re.captures(line).map(|cap| Robot {
                pos: Vector2::new(cap[1].parse().unwrap(), cap[2].parse().unwrap()),
                v: Vector2::new(cap[3].parse().unwrap(), cap[4].parse().unwrap()),
            })
        })
        .collect();

    // println!("{robots:?}");

    let output_a = solve_a(robots.clone(), width, height);
    let output_b = find_chirstmas_tree(robots, width, height);

    println!("Task1: {output_a}");
    println!("Task2: {output_b}");
}
