use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NumKey {
    Num(u8),
    Activate,
}

impl NumKey {
    fn to_keypad_pos(self) -> (isize, isize) {
        const TARGET_POS: [(isize, isize); 10] = [
            (3, 1),
            (2, 0),
            (2, 1),
            (2, 2),
            (1, 0),
            (1, 1),
            (1, 2),
            (0, 0),
            (0, 1),
            (0, 2),
        ];

        match self {
            Self::Num(n) => TARGET_POS[n as usize],
            Self::Activate => (3, 2),
        }
    }
}

impl Display for NumKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumKey::Num(n) => write!(f, "{n}"),
            NumKey::Activate => write!(f, "A"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DirKey {
    Up,
    Down,
    Left,
    Right,
    Activate,
}

impl DirKey {
    fn to_keypad_pos(self) -> (isize, isize) {
        match self {
            DirKey::Up => (0, 1),
            DirKey::Down => (1, 1),
            DirKey::Left => (1, 0),
            DirKey::Right => (1, 2),
            DirKey::Activate => (0, 2),
        }
    }
}

impl Display for DirKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirKey::Up => write!(f, "^"),
            DirKey::Down => write!(f, "v"),
            DirKey::Left => write!(f, "<"),
            DirKey::Right => write!(f, ">"),
            DirKey::Activate => write!(f, "A"),
        }
    }
}

fn print_dir_keys(dir_keys: &Vec<DirKey>) {
    for dir_key in dir_keys {
        print!("{dir_key}");
    }
    println!("");
}

trait KeyPad {
    const ROWS: usize;
    const COLS: usize;

    const START_ROW: usize;
    const START_COL: usize;

    type KeyT;

    fn map_pos_to_key(row: usize, col: usize) -> Option<Self::KeyT>;
}

struct NumKeyPad {}

impl KeyPad for NumKeyPad {
    const ROWS: usize = 4;
    const COLS: usize = 3;

    const START_ROW: usize = Self::ROWS - 1;
    const START_COL: usize = Self::COLS - 1;

    type KeyT = NumKey;

    fn map_pos_to_key(row: usize, col: usize) -> Option<Self::KeyT> {
        const KEYS: [Option<NumKey>; 4 * 3] = [
            Some(NumKey::Num(7)),
            Some(NumKey::Num(8)),
            Some(NumKey::Num(9)),
            Some(NumKey::Num(4)),
            Some(NumKey::Num(5)),
            Some(NumKey::Num(6)),
            Some(NumKey::Num(1)),
            Some(NumKey::Num(2)),
            Some(NumKey::Num(3)),
            None,
            Some(NumKey::Num(0)),
            Some(NumKey::Activate),
        ];

        KEYS[3 * row + col]
    }
}

struct DirKeyPad {}

impl KeyPad for DirKeyPad {
    const ROWS: usize = 2;
    const COLS: usize = 3;

    const START_ROW: usize = 0;
    const START_COL: usize = 2;

    type KeyT = DirKey;

    fn map_pos_to_key(row: usize, col: usize) -> Option<Self::KeyT> {
        const KEYS: [Option<DirKey>; 2 * 3] = [
            None,
            Some(DirKey::Up),
            Some(DirKey::Activate),
            Some(DirKey::Left),
            Some(DirKey::Down),
            Some(DirKey::Right),
        ];

        KEYS[3 * row + col]
    }
}

fn simulate_dirkey_pad<KP: KeyPad>(dir_keys: &Vec<DirKey>) -> Vec<KP::KeyT> {
    let mut cur_row = KP::START_ROW;
    let mut cur_col = KP::START_COL;

    let mut activated_keys = Vec::new();

    for (idx, dir_key) in dir_keys.iter().enumerate() {
        match dir_key {
            DirKey::Up => {
                cur_row = cur_row.checked_sub(1).expect("OUT OF BOUNDS: UP");
            }
            DirKey::Down => {
                cur_row += 1;
                if cur_row >= KP::ROWS {
                    panic!("OUT OF BOUNDS: DOWN")
                }
            }
            DirKey::Left => {
                cur_col = match cur_col.checked_sub(1) {
                    Some(c) => c,
                    None => panic!("OUT OF BOUNDS: LEFT, idx = {idx}"),
                };
            }
            DirKey::Right => {
                cur_col += 1;
                if cur_col >= KP::COLS {
                    panic!("OUT OF BOUNDS: RIGHT")
                }
            }
            DirKey::Activate => activated_keys.push(KP::map_pos_to_key(cur_row, cur_col).unwrap()),
        }

        if KP::map_pos_to_key(cur_row, cur_col).is_none() {
            panic!("Over GAP: idx = {idx}");
        }
    }

    activated_keys
}

fn find_shortest_pattern_stage1(combination: &Vec<NumKey>) -> Vec<DirKey> {
    let mut cur_row = 3;
    let mut cur_col = 2;

    let mut dir_keys = Vec::new();

    for num_key in combination {
        let (t_row, t_col) = num_key.to_keypad_pos();

        let mut row_off = t_row - cur_row;
        let mut col_off = t_col - cur_col;

        let vert_dir = if row_off < 0 {
            row_off = -row_off;
            DirKey::Up
        } else {
            DirKey::Down
        };
        let hort_dir = if col_off < 0 {
            col_off = -col_off;
            DirKey::Left
        } else {
            DirKey::Right
        };

        // preferable
        // RIGHT + UP
        // UP + LEFT (<- but not overall??)
        // LEFT + DOWN
        // DOWN + RIGHT

        // avoid gaps
        if cur_row == 3 && t_col == 0
            // || (vert_dir == DirKey::Up && hort_dir == DirKey::Left)
            || (vert_dir == DirKey::Down && hort_dir == DirKey::Right)
        {
            dir_keys.extend(std::iter::repeat_n(vert_dir, row_off as usize));
            dir_keys.extend(std::iter::repeat_n(hort_dir, col_off as usize));
        } else {
            dir_keys.extend(std::iter::repeat_n(hort_dir, col_off as usize));
            dir_keys.extend(std::iter::repeat_n(vert_dir, row_off as usize));
        }

        dir_keys.push(DirKey::Activate);

        cur_row = t_row;
        cur_col = t_col;
    }

    dir_keys
}

fn find_shortest_pattern_stage2(combination: &Vec<DirKey>) -> Vec<DirKey> {
    let mut cur_row = 0;
    let mut cur_col = 2;

    let mut dir_keys = Vec::new();

    for dir_key in combination {
        let (t_row, t_col) = dir_key.to_keypad_pos();

        let mut row_off = t_row - cur_row;
        let mut col_off = t_col - cur_col;

        let vert_dir = if row_off < 0 {
            row_off = -row_off;
            DirKey::Up
        } else {
            DirKey::Down
        };
        let hort_dir = if col_off < 0 {
            col_off = -col_off;
            DirKey::Left
        } else {
            DirKey::Right
        };

        // preferable
        // RIGHT + UP
        // UP + LEFT (<- but not overall??)
        // LEFT + DOWN
        // DOWN + RIGHT

        // avoiding gaps
        if cur_row == 0 && t_col == 0
            // || (vert_dir == DirKey::Up && hort_dir == DirKey::Left)
            || (vert_dir == DirKey::Down && hort_dir == DirKey::Right)
        {
            dir_keys.extend(std::iter::repeat_n(vert_dir, row_off as usize));
            dir_keys.extend(std::iter::repeat_n(hort_dir, col_off as usize));
        } else {
            dir_keys.extend(std::iter::repeat_n(hort_dir, col_off as usize));
            dir_keys.extend(std::iter::repeat_n(vert_dir, row_off as usize));
        }

        dir_keys.push(DirKey::Activate);

        cur_row = t_row;
        cur_col = t_col;
    }

    dir_keys
}

fn find_shortest_pattern(combination: &Vec<NumKey>, num_stage_2s: usize) -> usize {
    let stage1 = find_shortest_pattern_stage1(combination);
    let stage1_rev = simulate_dirkey_pad::<NumKeyPad>(&stage1);
    assert_eq!(stage1_rev, *combination);
    println!("Stage1: checked!");

    let mut stage2 = stage1;
    for i in 0..num_stage_2s {
        println!("Checking Stage 2.{i}: len = {}!", stage2.len());
        let new_stage = find_shortest_pattern_stage2(&stage2);
        let new_stage_rev = simulate_dirkey_pad::<DirKeyPad>(&new_stage);
        assert_eq!(stage2, new_stage_rev);
        stage2 = new_stage;
        println!("Checked Stage2.{i}!");
    }

    stage2.len()
}

fn calc_complexities(combinations: &Vec<Vec<NumKey>>, num_stage_2s: usize) -> usize {
    combinations
        .iter()
        .map(|combination| {
            let shortest_pattern_len = find_shortest_pattern(combination, num_stage_2s);
            let num = combination
                .iter()
                .filter_map(|num_key| match num_key {
                    NumKey::Num(n) => Some(*n),
                    NumKey::Activate => None,
                })
                .fold(0usize, |acc, elem| acc * 10 + elem as usize);

            num * shortest_pattern_len
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    // let input = std::fs::read_to_string("example.txt").unwrap();

    let inputs: Vec<Vec<NumKey>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '0' => NumKey::Num(0),
                    '1' => NumKey::Num(1),
                    '2' => NumKey::Num(2),
                    '3' => NumKey::Num(3),
                    '4' => NumKey::Num(4),
                    '5' => NumKey::Num(5),
                    '6' => NumKey::Num(6),
                    '7' => NumKey::Num(7),
                    '8' => NumKey::Num(8),
                    '9' => NumKey::Num(9),
                    'A' => NumKey::Activate,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    println!("{inputs:?}");

    println!("Task1: {}", calc_complexities(&inputs, 2));
    println!("Task2: {}", calc_complexities(&inputs, 25));
}
