use std::{collections::{HashMap, HashSet}, time::Instant};

fn find_stats_a(
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    row: usize,
    col: usize,
    c: char,
) -> (usize, usize) {
    if map[row][col] != c {
        return (0, 0);
    }

    visited[row][col] = true;

    const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut region_area = 1;
    let mut region_perimeter = 0;

    for (row_off, col_off) in DIRECTIONS {
        let n_row = row as isize + row_off;
        let n_col = col as isize + col_off;

        if n_row < 0 || n_row >= map.len() as isize || n_col < 0 || n_col >= map[0].len() as isize {
            region_perimeter += 1;
        } else if map[n_row as usize][n_col as usize] != c {
            region_perimeter += 1;
        } else if !visited[n_row as usize][n_col as usize] {
            let (sub_area, sub_perimeter) =
                find_stats_a(map, visited, n_row as usize, n_col as usize, c);
            region_area += sub_area;
            region_perimeter += sub_perimeter;
        }
    }

    (region_area, region_perimeter)
}

fn solve_a(map: &Vec<Vec<char>>) -> usize {
    let num_cols = map[0].len();
    let mut visited = vec![vec![false; num_cols]; map.len()];

    let mut sum = 0;

    for row in 0..map.len() {
        for col in 0..num_cols {
            if visited[row][col] {
                continue;
            }

            let (region_area, region_perimeter) =
                find_stats_a(&map, &mut visited, row, col, map[row][col]);
            sum += region_area * region_perimeter;
        }
    }

    sum
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Vertical,
    Horizontal,
}

impl Direction {
    fn inv(self) -> Self {
        match self {
            Self::Vertical => Self::Horizontal,
            Self::Horizontal => Self::Vertical,
        }
    }
}

fn find_stats_b(
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    perimeter: &mut HashMap<Direction, Vec<(usize, usize)>>,
    row: usize,
    col: usize,
    c: char,
) -> usize {
    if map[row][col] != c {
        return 0;
    }

    visited[row][col] = true;

    const DIRECTIONS: [(isize, isize, Direction, bool); 4] = [
        (-1, 0, Direction::Vertical, false),
        (1, 0, Direction::Vertical, true),
        (0, -1, Direction::Horizontal, false),
        (0, 1, Direction::Horizontal, true),
    ];

    let mut region_area = 1;

    for (row_off, col_off, dir, use_new) in DIRECTIONS {
        let n_row = row as isize + row_off;
        let n_col = col as isize + col_off;

        if n_row < 0 || n_row >= map.len() as isize || n_col < 0 || n_col >= map[0].len() as isize {
            if use_new {
                assert!(n_row >= 0 && n_col >= 0);
                perimeter
                    .entry(dir.inv())
                    .or_default()
                    .push((n_row as usize, n_col as usize));
            } else {
                perimeter.entry(dir.inv()).or_default().push((row, col));
            }
        } else if map[n_row as usize][n_col as usize] != c {
            if use_new {
                perimeter
                    .entry(dir.inv())
                    .or_default()
                    .push((n_row as usize, n_col as usize));
            } else {
                perimeter.entry(dir.inv()).or_default().push((row, col));
            }
        } else if !visited[n_row as usize][n_col as usize] {
            region_area += find_stats_b(map, visited, perimeter, n_row as usize, n_col as usize, c);
        }
    }

    region_area
}

fn find_region_sides(
    map: &Vec<Vec<char>>,
    perimeter: &mut HashMap<Direction, Vec<(usize, usize)>>,
    region_c: char,
) -> usize {
    let mut sides = 0;
    for (dir, perimeter_pieces) in perimeter.iter_mut() {
        perimeter_pieces.sort_by_key(|&(r, c)| match dir {
            Direction::Horizontal => -(c as isize),
            Direction::Vertical => -(r as isize),
        });
        // println!("{dir:?}, {perimeter_pieces:?}");
        while let Some((s_row, s_col)) = perimeter_pieces.pop() {
            // println!("[{s_row}, {s_col}], {perimeter_pieces:?}");
            let (side_anchor, mut side_e) = match dir {
                Direction::Vertical => (s_col, s_row),
                Direction::Horizontal => (s_row, s_col),
            };

            let off_a = match dir {
                Direction::Vertical => (0, -1),
                Direction::Horizontal => (-1, 0),
            };

            let char_a = {
                let n_row = (s_row as isize) + off_a.0;
                let n_col = (s_col as isize) + off_a.1;

                if n_row < 0
                    || n_row >= map.len() as isize
                    || n_col < 0
                    || n_col >= map[0].len() as isize
                {
                    false
                } else {
                    map[n_row as usize][n_col as usize] == region_c
                }
            };

            let char_b = map
                .get(s_row)
                .and_then(|r| r.get(s_col))
                .map_or(false, |&char_b| char_b == region_c);

            for i in (0..perimeter_pieces.len()).rev() {
                let (p_row, p_col) = perimeter_pieces[i];

                if char_a != {
                    let n_row = (p_row as isize) + off_a.0;
                    let n_col = (p_col as isize) + off_a.1;

                    if n_row < 0
                        || n_row >= map.len() as isize
                        || n_col < 0
                        || n_col >= map[0].len() as isize
                    {
                        false
                    } else {
                        map[n_row as usize][n_col as usize] == region_c
                    }
                } {
                    continue;
                }

                if char_b
                    != map
                        .get(p_row)
                        .and_then(|r| r.get(p_col))
                        .map_or(false, |&char_b| char_b == region_c)
                {
                    continue;
                }

                let (test_coord, match_coord) = match dir {
                    Direction::Horizontal => (p_col, p_row),
                    Direction::Vertical => (p_row, p_col),
                };

                if side_e + 1 == test_coord && side_anchor == match_coord {
                    side_e = test_coord;
                    perimeter_pieces.remove(i);
                }
            }

            // println!(
            //     "Side: {dir:?}, {side_anchor}, [{}, {side_e}]",
            //     match dir {
            //         Direction::Vertical => s_row,
            //         Direction::Horizontal => s_col,
            //     }
            // );

            // match dir {
            //     Direction::Vertical => {
            //         println!("Side: {dir:?}, [{s_row}, {side_anchor}] - [{side_e}, {side_anchor}]")
            //     }
            //     Direction::Horizontal => {
            //         println!("Side: {dir:?}, [{side_anchor}, {s_col}] - [{side_anchor}, {side_e}]")
            //     }
            // }

            sides += 1;
        }
    }

    sides
}

fn solve_b(map: &Vec<Vec<char>>) -> usize {
    let num_cols = map[0].len();
    let mut visited = vec![vec![false; num_cols]; map.len()];

    let mut sum = 0;

    for row in 0..map.len() {
        for col in 0..num_cols {
            if visited[row][col] {
                continue;
            }

            // println!("Region: {}", map[row][col]);

            let mut perimeter = HashMap::new();

            let region_area =
                find_stats_b(&map, &mut visited, &mut perimeter, row, col, map[row][col]);

            let region_sides = find_region_sides(&map, &mut perimeter, map[row][col]);

            // println!("Region: {}: {region_sides}", map[row][col]);

            sum += region_area * region_sides;
        }
    }

    sum
}

fn main() {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    // let input = std::fs::read_to_string("../example.txt").unwrap();
    // let input = std::fs::read_to_string("../example2.txt").unwrap();
    // let input = std::fs::read_to_string("../example3.txt").unwrap();

    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let start_a = Instant::now();
    let output_a = solve_a(&map);
    let elapsed_a = start_a.elapsed();
    let start_b = Instant::now();
    let output_b = solve_b(&map);
    let elapsed_b = start_b.elapsed();

    println!("Task 1: {output_a}");
    println!("Task 2: {output_b}");

    println!("Task 1 took: {}µs", elapsed_a.as_micros());
    println!("Task 2 took: {}µs", elapsed_b.as_micros());
}
