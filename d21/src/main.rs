use std::{collections::HashMap, fmt::Display, time::Instant};

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

// fn print_dir_keys(dir_keys: &Vec<DirKey>) {
//     for dir_key in dir_keys {
//         print!("{dir_key}");
//     }
//     println!("");
// }

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

struct Simulator<const N: usize> {
    num_pad_state: (usize, usize),
    dir_pad_state: [(usize, usize); N],
}

impl<const N: usize> Simulator<N> {
    pub fn new() -> Self {
        Self {
            num_pad_state: (NumKeyPad::START_ROW, NumKeyPad::START_COL),
            dir_pad_state: [(DirKeyPad::START_ROW, DirKeyPad::START_COL); N],
        }
    }

    pub fn simulate(&mut self, dir_key: DirKey) {
        // print!("{dir_key}");
        self.press_dir_key(dir_key, N);
    }

    fn press_dir_key(&mut self, dir_key: DirKey, stage: usize) {
        let row;
        let col;

        let row_max;
        let col_max;

        let invalid_row;
        let invalid_col;

        if stage == 0 {
            row = &mut self.num_pad_state.0;
            col = &mut self.num_pad_state.1;
            row_max = NumKeyPad::ROWS;
            col_max = NumKeyPad::COLS;
            invalid_row = 3;
            invalid_col = 0;
        } else {
            row = &mut self.dir_pad_state[stage - 1].0;
            col = &mut self.dir_pad_state[stage - 1].1;
            row_max = DirKeyPad::ROWS;
            col_max = DirKeyPad::COLS;
            invalid_row = 0;
            invalid_col = 0;
        };

        match dir_key {
            DirKey::Up => {
                if *row <= 0 {
                    panic!("Row out of bounds on stage {stage} for UP.")
                }

                *row -= 1;

                if *row == invalid_row && *col == invalid_col {
                    panic!("Over GAP on stage {stage}")
                }
            }
            DirKey::Down => {
                if *row + 1 >= row_max {
                    panic!("Row out of bounds on stage {stage} for DOWN.")
                }

                *row += 1;

                if *row == invalid_row && *col == invalid_col {
                    panic!("Over GAP on stage {stage}")
                }
            }
            DirKey::Left => {
                if *col <= 0 {
                    panic!("Col out of bounds on stage {stage} for LEFT.")
                }

                *col -= 1;

                if *row == invalid_row && *col == invalid_col {
                    panic!("Over GAP on stage {stage}")
                }
            }
            DirKey::Right => {
                if *col >= col_max {
                    panic!("Col out of bounds on stage {stage} for RIGHT.")
                }

                *col += 1;

                if *row == invalid_row && *col == invalid_col {
                    panic!("Over GAP on stage {stage}")
                }
            }
            DirKey::Activate => {
                if stage == 0 {
                    Self::press_num_key(NumKeyPad::map_pos_to_key(*row, *col).unwrap());
                } else {
                    let r = *row;
                    let c = *col;
                    self.press_dir_key(DirKeyPad::map_pos_to_key(r, c).unwrap(), stage - 1);
                }
            }
        }
    }

    fn press_num_key(num_key: NumKey) {
        print!("{num_key}");
    }
}

// fn simulate_dirkey_pad<KP: KeyPad>(dir_keys: &Vec<DirKey>) -> Vec<KP::KeyT> {
//     let mut cur_row = KP::START_ROW;
//     let mut cur_col = KP::START_COL;

//     let mut activated_keys = Vec::new();

//     for (idx, dir_key) in dir_keys.iter().enumerate() {
//         match dir_key {
//             DirKey::Up => {
//                 cur_row = cur_row.checked_sub(1).expect("OUT OF BOUNDS: UP");
//             }
//             DirKey::Down => {
//                 cur_row += 1;
//                 if cur_row >= KP::ROWS {
//                     panic!("OUT OF BOUNDS: DOWN")
//                 }
//             }
//             DirKey::Left => {
//                 cur_col = match cur_col.checked_sub(1) {
//                     Some(c) => c,
//                     None => panic!("OUT OF BOUNDS: LEFT, idx = {idx}"),
//                 };
//             }
//             DirKey::Right => {
//                 cur_col += 1;
//                 if cur_col >= KP::COLS {
//                     panic!("OUT OF BOUNDS: RIGHT")
//                 }
//             }
//             DirKey::Activate => activated_keys.push(KP::map_pos_to_key(cur_row, cur_col).unwrap()),
//         }

//         if KP::map_pos_to_key(cur_row, cur_col).is_none() {
//             panic!("Over GAP: idx = {idx}");
//         }
//     }

//     activated_keys
// }

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

        // avoid gaps
        // prefer horizontal, then vertical for LEFT
        // prefer vertical, then horizontal for RIGHT
        if cur_row == 3 && t_col == 0 {
            dir_keys.extend(std::iter::repeat_n(vert_dir, row_off as usize));
            dir_keys.extend(std::iter::repeat_n(hort_dir, col_off as usize));
        } else if cur_col == 0 && t_row == 3 {
            dir_keys.extend(std::iter::repeat_n(hort_dir, col_off as usize));
            dir_keys.extend(std::iter::repeat_n(vert_dir, row_off as usize));
        } else if hort_dir == DirKey::Left {
            dir_keys.extend(std::iter::repeat_n(hort_dir, col_off as usize));
            dir_keys.extend(std::iter::repeat_n(vert_dir, row_off as usize));
        } else {
            assert!(hort_dir == DirKey::Right);
            dir_keys.extend(std::iter::repeat_n(vert_dir, row_off as usize));
            dir_keys.extend(std::iter::repeat_n(hort_dir, col_off as usize));
        }

        dir_keys.push(DirKey::Activate);

        cur_row = t_row;
        cur_col = t_col;
    }

    dir_keys
}

fn find_shortest_pattern_stage2_stupid(combination: &Vec<DirKey>) -> Vec<DirKey> {
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

        // avoid gaps
        // prefer horizontal, then vertical for LEFT
        // prefer vertical, then horizontal for RIGHT
        if cur_row == 0 && t_col == 0 {
            assert_eq!(hort_dir, DirKey::Left);
            assert_eq!(vert_dir, DirKey::Down);
            dir_keys.extend(std::iter::repeat_n(vert_dir, row_off as usize));
            dir_keys.extend(std::iter::repeat_n(hort_dir, col_off as usize));
        } else if cur_col == 0 && t_row == 0 {
            assert_eq!(hort_dir, DirKey::Right);
            assert_eq!(vert_dir, DirKey::Up);
            dir_keys.extend(std::iter::repeat_n(hort_dir, col_off as usize));
            dir_keys.extend(std::iter::repeat_n(vert_dir, row_off as usize));
        } else if hort_dir == DirKey::Left {
            dir_keys.extend(std::iter::repeat_n(hort_dir, col_off as usize));
            dir_keys.extend(std::iter::repeat_n(vert_dir, row_off as usize));
        } else if hort_dir == DirKey::Right {
            dir_keys.extend(std::iter::repeat_n(vert_dir, row_off as usize));
            dir_keys.extend(std::iter::repeat_n(hort_dir, col_off as usize));
        }

        dir_keys.push(DirKey::Activate);

        cur_row = t_row;
        cur_col = t_col;
    }

    dir_keys
}

fn find_shortest_pattern_stage2_smart_helper<const N: usize>(
    key: DirKey,
    row: &mut usize,
    col: &mut usize,
    stages_left: usize,
    cache: &mut HashMap<(DirKey, usize, usize, usize), usize>,
    sim: &mut Option<&mut Simulator<N>>,
) -> usize {
    if stages_left == 0 {
        if let Some(sim) = sim {
            sim.simulate(key);
        }
        return 1;
    }

    if let Some(generated_keys) = cache.get(&(key, stages_left, *row, *col)) {
        let (t_row, t_col) = key.to_keypad_pos();
        *row = t_row as usize;
        *col = t_col as usize;
        return *generated_keys;
    }

    let (t_row, t_col) = key.to_keypad_pos();

    let mut row_off = t_row - *row as isize;
    let mut col_off = t_col - *col as isize;

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

    let mut next_pad_row = DirKeyPad::START_ROW;
    let mut next_pad_col = DirKeyPad::START_COL;
    let mut num_generated_keys = 0;

    let mut process_next_keys = |next_key: DirKey, count: usize| {
        for _ in 0..count {
            num_generated_keys += find_shortest_pattern_stage2_smart_helper(
                next_key,
                &mut next_pad_row,
                &mut next_pad_col,
                stages_left - 1,
                cache,
                sim,
            );
        }
    };

    // avoid gaps
    // prefer horizontal, then vertical for LEFT
    // prefer vertical, then horizontal for RIGHT
    if *row == 0 && t_col == 0 {
        assert_eq!(hort_dir, DirKey::Left);
        assert_eq!(vert_dir, DirKey::Down);
        process_next_keys(vert_dir, row_off as usize);
        process_next_keys(hort_dir, col_off as usize);
    } else if *col == 0 && t_row == 0 {
        assert_eq!(hort_dir, DirKey::Right);
        assert_eq!(vert_dir, DirKey::Up);
        process_next_keys(hort_dir, col_off as usize);
        process_next_keys(vert_dir, row_off as usize);
    } else if hort_dir == DirKey::Left {
        process_next_keys(hort_dir, col_off as usize);
        process_next_keys(vert_dir, row_off as usize);
    } else if hort_dir == DirKey::Right {
        process_next_keys(vert_dir, row_off as usize);
        process_next_keys(hort_dir, col_off as usize);
    }

    process_next_keys(DirKey::Activate, 1);

    if sim.is_none() {
        cache.insert((key, stages_left, *row, *col), num_generated_keys);
    }

    *row = t_row as usize;
    *col = t_col as usize;

    num_generated_keys
}

fn find_shortest_pattern_stage2_smart<const N: usize>(
    combination: &Vec<DirKey>,
    num_stage_2s: usize,
    sim: &mut Option<&mut Simulator<N>>,
) -> usize {
    let mut cache = HashMap::new();

    let mut next_pad_row = DirKeyPad::START_ROW;
    let mut next_pad_col = DirKeyPad::START_COL;

    combination
        .iter()
        .map(|&key| {
            find_shortest_pattern_stage2_smart_helper(
                key,
                &mut next_pad_row,
                &mut next_pad_col,
                num_stage_2s,
                &mut cache,
                sim,
            )
        })
        .sum()
}

fn find_shortest_pattern_smart<const N: usize>(
    combination: &Vec<NumKey>,
    num_stage_2s: usize,
    sim: &mut Option<&mut Simulator<N>>,
) -> usize {
    let stage1 = find_shortest_pattern_stage1(combination);
    find_shortest_pattern_stage2_smart(&stage1, num_stage_2s, sim)
}

fn find_shortest_pattern_stupid<const N: usize>(
    combination: &Vec<NumKey>,
    num_stage_2s: usize,
    sim: &mut Option<&mut Simulator<N>>,
) -> usize {
    let stage1 = find_shortest_pattern_stage1(combination);

    let mut stage2 = stage1;
    for _ in 0..num_stage_2s {
        stage2 = find_shortest_pattern_stage2_stupid(&stage2);
    }

    if let Some(sim) = sim {
        for &key in &stage2 {
            sim.simulate(key);
        }
    }

    stage2.len()
}

fn calc_complexities<
    const N: usize,
    F: Fn(&Vec<NumKey>, usize, &mut Option<&mut Simulator<N>>) -> usize,
>(
    combinations: &Vec<Vec<NumKey>>,
    num_stage_2s: usize,
    find_shortest_pattern_fn: F,
    sim: &mut Option<&mut Simulator<N>>,
) -> usize {
    combinations
        .iter()
        .map(|combination| {
            let shortest_pattern_len = find_shortest_pattern_fn(combination, num_stage_2s, sim);
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

    let start_task1_stupid = Instant::now();
    let task1_stupid = calc_complexities(&inputs, 2, find_shortest_pattern_stupid::<0>, &mut None);
    let elapsed_task1_stupid = start_task1_stupid.elapsed();
    let start_task1_smart = Instant::now();
    let task1_smart = calc_complexities(&inputs, 2, find_shortest_pattern_smart::<0>, &mut None);
    let elapsed_task1_smart = start_task1_smart.elapsed();
    let start_task2_smart = Instant::now();
    let task2_smart = calc_complexities(&inputs, 25, find_shortest_pattern_smart::<0>, &mut None);
    let elapsed_task2_smart = start_task2_smart.elapsed();

    println!("Task1 (stupid): {}", task1_stupid);
    println!("Task1 (smart): {}", task1_smart);
    println!("Task2: {}", task2_smart);

    println!(
        "Task1 (stupid) took: {}µs",
        elapsed_task1_stupid.as_micros()
    );
    println!("Task1 (smart) took: {}µs", elapsed_task1_smart.as_micros());
    println!("Task2 (smart) took: {}µs", elapsed_task2_smart.as_micros());

    const EX_ROUNDS: usize = 4;
    let mut sim_smart = Simulator::<EX_ROUNDS>::new();
    let mut sim_stupid = Simulator::<EX_ROUNDS>::new();
    let stupid = calc_complexities(
        &inputs,
        EX_ROUNDS,
        find_shortest_pattern_stupid,
        &mut Some(&mut sim_stupid),
    );
    println!("\nEx stupid: {}", stupid);
    let smart = calc_complexities(
        &inputs,
        EX_ROUNDS,
        find_shortest_pattern_smart,
        &mut Some(&mut sim_smart),
    );
    println!("\nEx smart: {}", smart);
    println!("Ex diff: {}", (smart as isize) - (stupid as isize));
}
