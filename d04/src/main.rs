fn solve_a(input: &Vec<Vec<char>>) -> u64 {
    let num_cols = input[0].len() as i64;
    let num_rows = input.len() as i64;

    let is_valid_pos = |r, c| r >= 0 && r < num_rows && c >= 0 && c < num_cols;

    const DIRECTIONS: [(i64, i64); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    const REFERENCE: [char; 4] = ['X', 'M', 'A', 'S'];

    let mut num_matches = 0;

    for base_row in 0..num_rows {
        for base_col in 0..num_cols {
            for (row_off, col_off) in DIRECTIONS {
                let mut is_valid = true;
                for k in 0..(REFERENCE.len() as i64) {
                    let row = base_row + row_off * k;
                    let col = base_col + col_off * k;
                    if !is_valid_pos(row, col) {
                        is_valid = false;
                        break;
                    }

                    if input[row as usize][col as usize] != REFERENCE[k as usize] {
                        is_valid = false;
                        break;
                    }
                }

                if is_valid {
                    num_matches += 1;
                }
            }
        }
    }

    num_matches
}

fn solve_b(input: &Vec<Vec<char>>) -> u64 {
    let num_cols = input[0].len() as i64;
    let num_rows = input.len() as i64;

    let is_valid_pos = |r, c| r >= 0 && r < num_rows && c >= 0 && c < num_cols;

    const DIRECTIONS: [(i64, i64); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut num_matches = 0;

    for base_row in 1..(num_rows - 1) {
        for base_col in 1..num_cols {
            if input[base_row as usize][base_col as usize] != 'A' {
                continue;
            }

            if !is_valid_pos(base_row - 1, base_col - 1)
                || !is_valid_pos(base_row - 1, base_col + 1)
                || !is_valid_pos(base_row + 1, base_col - 1)
                || !is_valid_pos(base_row + 1, base_col + 1)
            {
                continue;
            }

            let is_diag1_mas = match (
                input[(base_row - 1) as usize][(base_col - 1) as usize],
                input[(base_row + 1) as usize][(base_col + 1) as usize],
            ) {
                ('M', 'S') | ('S', 'M') => true,
                _ => false,
            };

            if !is_diag1_mas {
                continue;
            }

            let is_diag2_mas = match (
                input[(base_row - 1) as usize][(base_col + 1) as usize],
                input[(base_row + 1) as usize][(base_col - 1) as usize],
            ) {
                ('M', 'S') | ('S', 'M') => true,
                _ => false,
            };

            if !is_diag2_mas {
                continue;
            }

            num_matches += 1;
        }
    }

    num_matches
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let output_a = solve_a(&input);
    let output_b = solve_b(&input);

    println!("Task1: {output_a}");
    println!("Task2: {output_b}");
}
