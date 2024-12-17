use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn apply(self, row: usize, col: usize) -> (usize, usize) {
        match self {
            Self::Left => (row, col - 1),
            Self::Right => (row, col + 1),
            Self::Up => (row - 1, col),
            Self::Down => (row + 1, col),
        }
    }
    fn turn_right(self) -> Self {
        match self {
            Self::Left => Self::Up,
            Self::Right => Self::Down,
            Self::Up => Self::Right,
            Self::Down => Self::Left,
        }
    }
    fn turn_left(self) -> Self {
        match self {
            Self::Left => Self::Down,
            Self::Right => Self::Up,
            Self::Up => Self::Left,
            Self::Down => Self::Right,
        }
    }
}

const COST_TURN: u64 = 1000;
const COST_STEP: u64 = 1;

fn solve(
    walls: &Vec<Vec<bool>>,
    start_row: usize,
    start_col: usize,
    end_row: usize,
    end_col: usize,
) -> (u64, u64) {
    let mut queue = vec![(start_row, start_col, Direction::Right, 0, Vec::new())];

    let mut visited = HashMap::new();
    let mut min_cost_tiles = HashSet::new();
    let mut min_cost = u64::MAX;

    while let Some((check_row, check_col, check_dir, acc_cost, history)) = queue.pop() {
        if min_cost < acc_cost {
            continue;
        }
        if check_row == end_row && check_col == end_col {
            if acc_cost < min_cost {
                min_cost_tiles = HashSet::new();
                min_cost = acc_cost;
            }
            if acc_cost == min_cost {
                for history_tile in history {
                    min_cost_tiles.insert(history_tile);
                }
            }
            continue;
        }

        if let Some(&old_cost) = visited.get(&(check_row, check_col)) {
            if old_cost < acc_cost {
                continue;
            }
        }

        visited
            .entry((check_row, check_col))
            .and_modify(|cost| *cost = acc_cost)
            .or_insert(acc_cost);

        let mut check_direction = |dir: Direction, extra_cost| {
            let (n_row, n_col) = dir.apply(check_row, check_col);
            if !walls[n_row][n_col] {
                let mut history_clone = history.clone();
                history_clone.push((n_row, n_col));
                queue.push((n_row, n_col, dir, acc_cost + extra_cost, history_clone));
            }
        };

        check_direction(check_dir, COST_STEP);
        check_direction(check_dir.turn_right(), COST_TURN + COST_STEP);
        check_direction(check_dir.turn_left(), COST_TURN + COST_STEP);
    }

    min_cost_tiles.insert((start_row, start_col));

    (min_cost, min_cost_tiles.len() as u64)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    // let input = std::fs::read_to_string("example.txt").unwrap();
    // let input = std::fs::read_to_string("example2.txt").unwrap();

    let mut start_row = None;
    let mut start_col = None;
    let mut end_row = None;
    let mut end_col = None;

    let walls: Vec<Vec<bool>> = input
        .lines()
        .enumerate()
        .map(|(row_idx, row)| {
            row.chars()
                .enumerate()
                .map(|(col_idx, c)| match c {
                    '#' => true,
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
                    '.' => false,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let (output_a, output_b) = solve(
        &walls,
        start_row.unwrap(),
        start_col.unwrap(),
        end_row.unwrap(),
        end_col.unwrap(),
    );

    println!("Task1: {output_a}");
    println!("Task2: {output_b}");
}
