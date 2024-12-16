use std::collections::HashSet;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum DIR {
    UP,
    LEFT,
    RIGHT,
    DOWN,
}

impl DIR {
    fn rotate(self) -> Self {
        match self {
            Self::UP => Self::RIGHT,
            Self::RIGHT => Self::DOWN,
            Self::DOWN => Self::LEFT,
            Self::LEFT => Self::UP,
        }
    }

    fn to_offset(&self) -> (isize, isize) {
        match self {
            Self::UP => (-1, 0),
            Self::RIGHT => (0, 1),
            Self::DOWN => (1, 0),
            Self::LEFT => (0, -1),
        }
    }
}

fn solve_a(map: &Vec<Vec<char>>) -> u64 {
    let mut path = HashSet::new();

    let mut row = 0isize;
    let mut col = 0isize;
    for (line_idx, line) in map.iter().enumerate() {
        if line.contains(&'^') {
            row = line_idx as isize;
            col = line.iter().position(|c| *c == '^').unwrap() as isize;
        }
    }

    let mut dir = DIR::UP;

    loop {
        path.insert((row, col));
        let (r_off, c_off) = dir.to_offset();

        let n_row = row + r_off;
        let n_col = col + c_off;
        if n_row < 0 || n_row >= map.len() as isize || n_col < 0 || n_col >= map[0].len() as isize {
            break;
        }

        match map[n_row as usize][n_col as usize] {
            '^' | '.' => {
                row = n_row;
                col = n_col;
            },
            '#' => dir = dir.rotate(),
            _ => unreachable!(),
        }
    }

    path.len() as u64
}

fn solve_b(orig_map: &Vec<Vec<char>>) -> u64 {
    let mut options = 0;
    for m_row in 0..orig_map.len() {
        for m_col in 0..orig_map[0].len() {
            let mut map = orig_map.clone();
            map[m_row][m_col] = '#';
            let mut path = HashSet::new();

            let mut row = 0isize;
            let mut col = 0isize;
            for (line_idx, line) in map.iter().enumerate() {
                if line.contains(&'^') {
                    row = line_idx as isize;
                    col = line.iter().position(|c| *c == '^').unwrap() as isize;
                }
            }

            if row as usize == m_row && col as usize == m_col {
                continue;
            }

            let mut dir = DIR::UP;

            let is_option = loop {
                if path.contains(&(row, col, dir)) {
                    break true;
                }
                path.insert((row, col, dir));
                let (r_off, c_off) = dir.to_offset();

                let n_row = row + r_off;
                let n_col = col + c_off;
                if n_row < 0
                    || n_row >= map.len() as isize
                    || n_col < 0
                    || n_col >= map[0].len() as isize
                {
                    break false;
                }

                match map[n_row as usize][n_col as usize] {
                    '^' | '.' => {
                        row = n_row;
                        col = n_col;
                    }
                    '#' => dir = dir.rotate(),
                    _ => unreachable!(),
                }
            };

            if is_option {
                options += 1;
                // println!("{m_row}, {m_col}");
            }
        }
    }

    options
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    // let input = std::fs::read_to_string("sample.txt").unwrap();

    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    println!("{}, {}", map.len(), map[0].len());

    let output_a = solve_a(&map);
    let output_b = solve_b(&map);
    println!("Task1: {output_a}");
    println!("Task2: {output_b}");
}
