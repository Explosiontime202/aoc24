use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Object {
    Border,
    Box,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Object2 {
    Border,
    BoxStart,
    BoxEnd,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn apply(self, row: usize, col: usize) -> (usize, usize) {
        match self {
            Direction::Up => (row - 1, col),
            Direction::Right => (row, col + 1),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
        }
    }
}

fn print_map(map: &Vec<Vec<Option<Object2>>>, robot_row: usize, robot_col: usize) {
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            if row_idx == robot_row && col_idx == robot_col {
                print!("@");
            } else {
                print!(
                    "{}",
                    match col {
                        Some(obj) => match obj {
                            Object2::Border => '#',
                            Object2::BoxStart => '[',
                            Object2::BoxEnd => ']',
                        },
                        None => '.',
                    }
                );
            }
        }
        println!("");
    }
}

fn solve_a(input: &String) -> u64 {
    let mut robot_col = None;
    let mut robot_row = None;

    let mut input = input.split("\n\n");

    let mut map: Vec<Vec<Option<Object>>> = input
        .next()
        .unwrap()
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '#' => Some(Object::Border),
                    '.' => None,
                    'O' => Some(Object::Box),
                    '@' => {
                        robot_col = Some(col);
                        robot_row = Some(row);
                        None
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let mut robot_col = robot_col.unwrap();
    let mut robot_row = robot_row.unwrap();

    let instructions: Vec<Direction> = input
        .next()
        .unwrap()
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| match c {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => unreachable!(),
        })
        .collect();

    'outer: for dir in instructions {
        let (mut n_row, mut n_col) = dir.apply(robot_row, robot_col);
        let (next_robot_row, next_robot_col) = (n_row, n_col);

        loop {
            match map[n_row][n_col] {
                Some(obj) => match obj {
                    Object::Border => continue 'outer,
                    Object::Box => {
                        (n_row, n_col) = dir.apply(n_row, n_col);
                    }
                },
                None => break,
            }
        }

        map[n_row][n_col] = Some(Object::Box);
        map[next_robot_row][next_robot_col] = None;

        robot_row = next_robot_row;
        robot_col = next_robot_col;
    }

    map.into_iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.into_iter()
                .enumerate()
                .filter(|&(_, col)| col.is_some_and(|obj| obj == Object::Box))
                .map(|(col_idx, _)| 100 * row_idx + col_idx)
                .sum::<usize>()
        })
        .sum::<usize>() as u64
}

fn solve_b(input: &String) -> u64 {
    let mut robot_col = None;
    let mut robot_row = None;

    let mut input = input.split("\n\n");

    let mut map: Vec<Vec<Option<Object2>>> = input
        .next()
        .unwrap()
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '#' => [Some(Object2::Border), Some(Object2::Border)],
                    '.' => [None, None],
                    'O' => [Some(Object2::BoxStart), Some(Object2::BoxEnd)],
                    '@' => {
                        robot_row = Some(row);
                        robot_col = Some(2 * col);
                        [None, None]
                    }
                    _ => unreachable!(),
                })
                .flat_map(|objs| objs.into_iter())
                .collect()
        })
        .collect();

    let mut robot_col = robot_col.unwrap();
    let mut robot_row = robot_row.unwrap();

    let instructions: Vec<Direction> = input
        .next()
        .unwrap()
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| match c {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => unreachable!(),
        })
        .collect();

    'outer: for dir in instructions {
        // print_map(&map, robot_row, robot_col);
        // println!("{dir:?}");
        let (next_robot_row, next_robot_col) = dir.apply(robot_row, robot_col);
        let mut check_stack = vec![(next_robot_row, next_robot_col, true)];

        let mut visited = HashSet::new();
        let mut box_moves = HashSet::new();

        while let Some((n_row, n_col, check_other)) = check_stack.pop() {
            if visited.contains(&(n_row, n_col)) {
                continue;
            }
            visited.insert((n_row, n_col));
            match map[n_row][n_col] {
                Some(obj) => match obj {
                    Object2::Border => continue 'outer,
                    Object2::BoxStart => {
                        box_moves.insert((n_row, n_col));
                        let (next_row, next_col) = dir.apply(n_row, n_col);
                        check_stack.push((next_row, next_col, true));
                        if check_other {
                            check_stack.push((n_row, n_col + 1, false));
                        }
                    }
                    Object2::BoxEnd => {
                        let (next_row, next_col) = dir.apply(n_row, n_col);
                        check_stack.push((next_row, next_col, true));
                        if check_other {
                            check_stack.push((n_row, n_col - 1, false));
                        }
                    }
                },
                None => continue,
            }
        }

        assert!(check_stack.is_empty());

        for &(box_row, box_col) in &box_moves {
            map[box_row][box_col] = None;
            map[box_row][box_col + 1] = None;
        }
        for &(box_row, box_col) in &box_moves {
            let (moved_row, moved_col) = dir.apply(box_row, box_col);
            map[moved_row][moved_col] = Some(Object2::BoxStart);
            map[moved_row][moved_col + 1] = Some(Object2::BoxEnd);
        }
        map[next_robot_row][next_robot_col] = None;

        robot_row = next_robot_row;
        robot_col = next_robot_col;
    }

    map.into_iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.into_iter()
                .enumerate()
                .filter(|&(_, col)| col.is_some_and(|obj| obj == Object2::BoxStart))
                .map(|(col_idx, _)| 100 * row_idx + col_idx)
                .sum::<usize>()
        })
        .sum::<usize>() as u64
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    // let input = std::fs::read_to_string("example.txt").unwrap();
    // let input = std::fs::read_to_string("example2.txt").unwrap();

    let output_a = solve_a(&input);
    let output_b = solve_b(&input);

    println!("Task1: {output_a}");
    println!("Task1: {output_b}");
}
