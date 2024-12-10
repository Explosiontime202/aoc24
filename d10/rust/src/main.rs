use day10::{parse, read_input, solve};
fn main() {
    let input = read_input("../input.txt");
    // let input = std::fs::read_to_string("../example.txt").unwrap();

    let (map, trailheads) = parse(&input);

    let (output_a, output_b) = solve(&map, &trailheads);

    println!("Task1: {}", output_a);
    println!("Task2: {}", output_b);
}
