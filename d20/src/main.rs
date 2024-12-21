use std::time::Instant;

fn find_shortest_path(
    walls: &Vec<Vec<bool>>,
    start_row: usize,
    start_col: usize,
    end_row: usize,
    end_col: usize,
) -> Vec<Vec<Option<u64>>> {
    let num_rows = walls.len();
    let num_cols = walls[0].len();
    let mut queue = vec![(start_row, start_col, 0u64)];

    let mut visited = vec![vec![None; num_rows]; num_cols];

    while let Some((row, col, time)) = queue.pop() {
        if walls[row][col] {
            unreachable!("wall at {row} {col}")
        }
        if let Some(old_time) = visited[row][col].as_mut() {
            if *old_time < time {
                continue;
            }
            *old_time = time;
        } else {
            visited[row][col] = Some(time);
        }

        if row == end_row && col == end_col {
            continue;
        }

        let mut push_queue = |q_row: usize, q_col: usize| {
            if !walls[q_row][q_col] {
                queue.push((q_row, q_col, time + 1));
            }
        };

        if row > 0 {
            // go UP
            push_queue(row - 1, col);
        }

        if row + 1 < num_rows {
            // go DOWN
            push_queue(row + 1, col);
        }

        if col > 0 {
            // go LEFT
            push_queue(row, col - 1);
        }

        if col + 1 < num_cols {
            // go RIGHT
            push_queue(row, col + 1);
        }
    }

    visited
}

fn find_all_cheats(
    walls: &Vec<Vec<bool>>,
    start_row: usize,
    start_col: usize,
    end_row: usize,
    end_col: usize,
    max_cheat_time: usize,
) -> u64 {
    let num_rows = walls.len();
    let num_cols = walls[0].len();
    let time_taken = find_shortest_path(walls, start_row, start_col, end_row, end_col);

    let regular_time = time_taken[end_row][end_col].unwrap();

    let mut counted_cheats = 0;

    for s_row in 0..num_rows {
        for s_col in 0..num_cols {
            if walls[s_row][s_col] {
                continue;
            }
            let start_time = time_taken[s_row][s_col].unwrap();
            for row_off in (-(max_cheat_time as isize))..=(max_cheat_time as isize) {
                for col_off in (-(max_cheat_time as isize - row_off.abs()))
                    ..=(max_cheat_time as isize - row_off.abs())
                {
                    if row_off == 0 && col_off == 0 {
                        continue;
                    }
                    let target_row = s_row as isize + row_off;
                    let target_col = s_col as isize + col_off;
                    if target_row < 0
                        || target_row >= num_rows as isize
                        || target_col < 0
                        || target_col >= num_cols as isize
                    {
                        continue;
                    }

                    let target_row = target_row as usize;
                    let target_col = target_col as usize;

                    if walls[target_row][target_col] {
                        continue;
                    }

                    let cheat_time = row_off.abs() + col_off.abs();
                    let end_time = time_taken[end_row][end_col].unwrap()
                        - time_taken[target_row][target_col].unwrap();

                    let total_time = start_time + cheat_time as u64 + end_time;

                    if total_time + 100 <= regular_time {
                        counted_cheats += 1;
                    }
                }
            }
        }
    }

    counted_cheats
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    // let input = std::fs::read_to_string("example.txt").unwrap();

    let mut start_row = None;
    let mut start_col = None;
    let mut end_row = None;
    let mut end_col = None;
    let walls: Vec<Vec<bool>> = input
        .lines()
        .enumerate()
        .map(|(row_idx, line)| {
            line.chars()
                .enumerate()
                .map(|(col_idx, c)| match c {
                    '#' => true,
                    '.' => false,
                    'S' => {
                        start_row = Some(row_idx);
                        start_col = Some(col_idx);
                        false
                    }
                    'E' => {
                        end_row = Some(row_idx);
                        end_col = Some(col_idx);
                        false
                    }
                    _ => unreachable!("{c}"),
                })
                .collect()
        })
        .collect();

    let start_a = Instant::now();
    let output_a = find_all_cheats(
        &walls,
        start_row.unwrap(),
        start_col.unwrap(),
        end_row.unwrap(),
        end_col.unwrap(),
        2,
    );
    let elapsed_a = start_a.elapsed();

    let start_b = Instant::now();
    let output_b = find_all_cheats(
        &walls,
        start_row.unwrap(),
        start_col.unwrap(),
        end_row.unwrap(),
        end_col.unwrap(),
        20,
    );
    let elapsed_b = start_b.elapsed();

    println!("Task1: {output_a}");
    println!("Task1: {output_b}");

    println!("Task1 took: {}µs", elapsed_a.as_micros());
    println!("Task2 took: {}ms", elapsed_b.as_millis());
}
