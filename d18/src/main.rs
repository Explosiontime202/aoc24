use std::time::Instant;

use regex::Regex;

fn find_path(map: &Vec<Vec<bool>>, map_size: usize, shortest: bool) -> Option<u64> {
    let mut queue = vec![(0, 0, 0u64)];
    let mut visited = vec![vec![None; map_size]; map_size];
    while let Some((row, col, steps)) = queue.pop() {
        if let Some(old_steps) = &mut visited[row][col] {
            if *old_steps <= steps {
                continue;
            }
            *old_steps = steps;
        } else {
            visited[row][col] = Some(steps);
        }

        if row == map_size - 1 && col == map_size - 1 {
            if !shortest {
                break;
            } else {
                continue;
            }
        }

        if row > 0 && !map[row - 1][col] {
            queue.push((row - 1, col, steps + 1));
        }

        if col > 0 && !map[row][col - 1] {
            queue.push((row, col - 1, steps + 1));
        }

        if row + 1 < map_size && !map[row + 1][col] {
            queue.push((row + 1, col, steps + 1));
        }

        if col + 1 < map_size && !map[row][col + 1] {
            queue.push((row, col + 1, steps + 1));
        }
    }

    visited[map_size - 1][map_size - 1]
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    // let input = std::fs::read_to_string("example.txt").unwrap();

    let num_bytes = 1024;
    // let num_bytes = 12;

    let map_size = 71;
    // let map_size = 7;

    let coord_re = Regex::new(r"^(\d+),(\d+)$").unwrap();

    let mut falling_bytes = Vec::new();

    for line in input.lines() {
        let cap = coord_re.captures(line).unwrap();
        let row: usize = cap[1].parse().unwrap();
        let col: usize = cap[2].parse().unwrap();
        falling_bytes.push((row, col));
    }

    let mut map = vec![vec![false; map_size]; map_size];

    for &(row, col) in falling_bytes.iter().take(num_bytes) {
        map[row][col] = true;
    }

    let start_a = Instant::now();
    let output_a = find_path(&map, map_size, true).unwrap();
    let elapsed_a = start_a.elapsed();

    let start_b = Instant::now();
    let mut next_byte = num_bytes;
    let output_b = loop {
        if let Some(&(row, col)) = falling_bytes.get(next_byte) {
            map[row][col] = true;
        } else {
            unreachable!();
        }

        if find_path(&map, map_size, false).is_none() {
            break falling_bytes[next_byte];
        }

        next_byte += 1;
    };
    let elapsed_b = start_b.elapsed();

    println!("Task1: {output_a}");
    println!("Task2: {},{}", output_b.0, output_b.1);

    println!("Task 1 took {}ms", elapsed_a.as_millis());
    println!("Task 2 took {}ms", elapsed_b.as_millis());
}
