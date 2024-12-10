use rustc_hash::FxHashSet as HashSet;
use std::path::Path;

pub fn solve(map: &Vec<Vec<u32>>, trailheads: &HashSet<(isize, isize)>) -> (u32, u32) {
    let num_rows = map.len() as isize;
    let num_cols = map[0].len() as isize;

    let mut reached_nines_sum = 0;
    let mut rating_sum = 0;
    for &(row, col) in trailheads {
        let mut reached_nines = HashSet::default();
        let mut rating = 0;

        let mut paths = Vec::new();

        paths.push((row, col));

        while let Some((row, col)) = paths.pop() {
            const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

            let height = map[row as usize][col as usize];

            if height == 9 {
                reached_nines.insert((row, col));
                rating += 1;
                continue;
            }

            let target_height = height + 1;

            for (row_off, col_off) in DIRECTIONS {
                let n_row = row + row_off;
                let n_col = col + col_off;

                if n_row < 0 || n_row >= num_rows || n_col < 0 || n_col >= num_cols {
                    continue;
                }

                if map[n_row as usize][n_col as usize] == target_height {
                    paths.push((n_row, n_col));
                }
            }
        }

        reached_nines_sum += reached_nines.len() as u32;
        rating_sum += rating;
    }
    (reached_nines_sum, rating_sum)
}

pub fn read_input<P: AsRef<Path>>(path: P) -> String {
    std::fs::read_to_string(path).unwrap()
}

pub fn parse(input: &String) -> (Vec<Vec<u32>>, HashSet<(isize, isize)>) {
    let mut trailheads = HashSet::default();
    let map = input
        .lines()
        .enumerate()
        .map(|(row_idx, row)| {
            row.chars()
                .enumerate()
                .map(|(col_idx, c)| match c {
                    '0' => {
                        trailheads.insert((row_idx as isize, col_idx as isize));
                        0
                    }
                    '1' => 1,
                    '2' => 2,
                    '3' => 3,
                    '4' => 4,
                    '5' => 5,
                    '6' => 6,
                    '7' => 7,
                    '8' => 8,
                    '9' => 9,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    (map, trailheads)
}
