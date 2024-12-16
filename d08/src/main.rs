use std::{
    collections::{HashMap, HashSet},
    iter::{Repeat, RepeatN},
};

use nalgebra::Vector2;

fn print_antennas_antinodes(
    antennas: &HashMap<char, Vec<Vector2<isize>>>,
    antinodes: &HashSet<Vector2<isize>>,
    num_rows: usize,
    num_cols: usize,
) {
    let mut buf = Vec::new();

    for i in 0..num_rows {
        buf.push(Vec::from_iter(std::iter::repeat_n('.', num_cols)));
    }

    for (c, positions) in antennas {
        for pos in positions {
            buf[pos.x as usize][pos.y as usize] = *c;
        }
    }

    for antinode in antinodes {
        buf[antinode.x as usize][antinode.y as usize] = '#';
    }

    for i in 0..num_rows {
        for j in 0..num_cols {
            print!("{}", buf[i][j]);
        }
        println!("");
    }
}

fn solve(
    antennas: &HashMap<char, Vec<Vector2<isize>>>,
    num_rows: usize,
    num_cols: usize,
    restrict_distance: bool,
) -> u64 {
    let is_pos_in_bounds = |pos: Vector2<isize>| {
        pos.x >= 0 && pos.x < num_rows as isize && pos.y >= 0 && pos.y < num_cols as isize
    };

    let mut antinodes: HashSet<Vector2<isize>> = HashSet::new();
    for (c, positions) in antennas {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let a = positions[i];
                let b = positions[j];
                let diff = b - a;

                if restrict_distance {
                    let loc_a = a - diff;
                    let loc_b = b + diff;

                    if is_pos_in_bounds(loc_a) {
                        antinodes.insert(loc_a);
                    }

                    if is_pos_in_bounds(loc_b) {
                        antinodes.insert(loc_b);
                    }
                } else {
                    let mut cur_pos = a;
                    loop {
                        if !is_pos_in_bounds(cur_pos) {
                            break;
                        }

                        antinodes.insert(cur_pos);
                        cur_pos += diff;
                    }

                    let mut cur_pos = a;
                    loop {
                        if !is_pos_in_bounds(cur_pos) {
                            break;
                        }

                        antinodes.insert(cur_pos);
                        cur_pos -= diff;
                    }
                }
            }
        }
    }

    // print_antennas_antinodes(antennas, &antinodes, num_rows, num_cols);

    antinodes.len() as u64
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    // let input = std::fs::read_to_string("example.txt").unwrap();

    let mut antennas = HashMap::new();

    let num_rows = input.lines().count();
    let num_cols = input.lines().next().unwrap().len();

    for (row_idx, row) in input.lines().enumerate() {
        for (col_idx, c) in row.chars().enumerate() {
            if c.is_alphanumeric() {
                antennas
                    .entry(c)
                    .or_insert(Vec::new())
                    .push(Vector2::new(row_idx as isize, col_idx as isize));
            }
        }
    }

    println!("{:?}", antennas);

    let output_a = solve(&antennas, num_rows, num_cols, true);
    let output_b = solve(&antennas, num_rows, num_cols, false);

    println!("Task1 = {output_a}");
    println!("Task2 = {output_b}");
}
