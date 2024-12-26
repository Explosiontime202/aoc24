fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    // let input = std::fs::read_to_string("example.txt").unwrap();

    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for schematic in input.split("\n\n") {
        let lines: Vec<Vec<char>> = schematic
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        assert_eq!(lines[0].len(), 5);

        let search_char = lines[0][0];
        let is_lock = search_char == '#';

        let heights: [usize; 5] = std::array::from_fn(|col| {
            let mut height = 5;
            for row in 1..6 {
                if lines[row][col] != search_char {
                    height = row - 1;
                    break;
                }
            }

            if is_lock {
                height
            } else {
                5 - height
            }
        });

        if is_lock {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }

    // println!("keys = {keys:?}");
    // println!("locks = {locks:?}");
    println!("keys.len() = {}", keys.len());
    println!("locks.len() = {}", locks.len());

    let mut valid_pairs = 0;

    for key in keys {
        for lock in &locks {
            let mut is_valid = true;
            for i in 0..5 {
                if key[i] + lock[i] > 5 {
                    is_valid = false;
                    break;
                }
            }

            if is_valid {
                valid_pairs += 1;
            }
        }
    }

    println!("Task1: {valid_pairs}");
}
