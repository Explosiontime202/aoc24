use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq, Eq)]
enum GateInput<'a> {
    Wire(&'a str),
    Value(bool),
}

impl<'a> GateInput<'a> {
    fn get_write_unchecked(&self) -> &'a str {
        match self {
            GateInput::Wire(wire) => wire,
            GateInput::Value(_) => panic!("get_write_unchecked on GateInput::Value"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn apply(self, a: bool, b: bool) -> bool {
        match self {
            Self::And => a && b,
            Self::Or => a || b,
            Self::Xor => a != b,
        }
    }
}

impl TryFrom<&str> for Op {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "AND" => Ok(Self::And),
            "OR" => Ok(Self::Or),
            "XOR" => Ok(Self::Xor),
            _ => Err(()),
        }
    }
}

fn run_circuit<'a>(
    mut gates: Vec<(GateInput<'a>, GateInput<'a>, Op, &'a str)>,
    initial_values: &Vec<(&'a str, bool)>,
    wire_usages: &HashMap<&'a str, HashSet<(usize, u8)>>,
    outputs: &Vec<&'a str>,
) -> u64 {
    let mut work_queue = initial_values.clone();

    let mut wires: HashMap<&str, bool> = HashMap::new();

    while let Some((wire, value)) = work_queue.pop() {
        wires.insert(wire, value);

        if let Some(usages) = wire_usages.get(wire) {
            for &(usage_idx, input_idx) in usages {
                let gate = gates.get_mut(usage_idx).unwrap();
                match input_idx {
                    0 => gate.0 = GateInput::Value(value),
                    1 => gate.1 = GateInput::Value(value),
                    _ => unreachable!(),
                }

                match (&gate.0, &gate.1) {
                    (&GateInput::Value(a), &GateInput::Value(b)) => {
                        let result = gate.2.apply(a, b);
                        wires.insert(gate.3, result);
                        work_queue.push((gate.3, result));
                    }
                    _ => (),
                }
            }
        }
    }

    let mut output_val = 0u64;
    for output in outputs {
        output_val = (output_val << 1) | (wires[output] as u64);
    }

    output_val
}

fn find_replacement<'a>(
    known_input: &'a str,
    replaced_input: &'a str,
    op: Op,
    gates: &Vec<(GateInput<'a>, GateInput<'a>, Op, &'a str)>,
) -> Option<(&'a str, usize)> {
    for (gate_idx, (input_a, input_b, gate_op, _)) in gates.iter().enumerate() {
        if input_a == &GateInput::Wire(replaced_input)
            || input_b == &GateInput::Wire(replaced_input)
        {
            continue;
        }
        let match_a = input_a == &GateInput::Wire(&known_input);
        let match_b = input_b == &GateInput::Wire(&known_input);
        if *gate_op == op && (match_a || match_b) {
            if match_a {
                return Some((input_b.get_write_unchecked(), gate_idx));
            }
            if match_b {
                return Some((input_a.get_write_unchecked(), gate_idx));
            }
        }
    }

    None
}

fn swap<'a>(
    gates: &mut Vec<(GateInput<'a>, GateInput<'a>, Op, &'a str)>,
    wire_usages: &HashMap<&'a str, HashSet<(usize, u8)>>,
    origin_map: &HashMap<&'a str, usize>,
    a: &'a str,
    b: &'a str,
) {
    println!("swap: {a} <-> {b}");
    if let Some(&origin) = origin_map.get(a) {
        gates[origin].3 = b;
    }
    if let Some(&origin) = origin_map.get(b) {
        gates[origin].3 = a;
    }

    if let Some(a_usages) = wire_usages.get(a) {
        for &(gate_idx, input_idx) in a_usages {
            let gate = &mut gates[gate_idx];
            match input_idx {
                0 => gate.0 = GateInput::Wire(b),
                1 => gate.1 = GateInput::Wire(b),
                _ => unreachable!(),
            }
        }
    }
    if let Some(b_usages) = wire_usages.get(b) {
        for &(gate_idx, input_idx) in b_usages {
            let gate = &mut gates[gate_idx];
            match input_idx {
                0 => gate.0 = GateInput::Wire(a),
                1 => gate.1 = GateInput::Wire(a),
                _ => unreachable!(),
            }
        }
    }
}

fn find_actual_gate<'a>(
    input_a: &'a str,
    input_b: &'a str,
    op: Op,
    gates: &mut Vec<(GateInput<'a>, GateInput<'a>, Op, &'a str)>,
    swaps: &mut Vec<&'a str>,
    wire_usages: &HashMap<&'a str, HashSet<(usize, u8)>>,
    origin_map: &HashMap<&'a str, usize>,
) -> usize {
    let mut gate_idx_opt = None;

    if let Some(usages_a) = wire_usages.get(input_a) {
        if let Some(usages_b) = wire_usages.get(input_b) {
            for &(gate_idx, input_idx) in usages_a {
                if usages_b.contains(&(gate_idx, 1 - input_idx)) {
                    let gate = &gates[gate_idx];

                    if gate.2 == op {
                        gate_idx_opt = Some(gate_idx);
                        break;
                    }
                }
            }
        }
    }

    match gate_idx_opt {
        None => {
            let replacement_b = find_replacement(input_a, input_b, op, gates);
            let replacement_a = find_replacement(input_b, input_a, op, gates);

            match (replacement_a, replacement_b) {
                (Some(r1), Some(r2)) => unreachable!("{r1:?} {r2:?}"),
                (Some((replacement, gate_idx)), None) => {
                    swaps.push(input_a);
                    swaps.push(replacement);
                    swap(gates, wire_usages, origin_map, input_a, replacement);
                    gate_idx
                }
                (None, Some((replacement, gate_idx))) => {
                    swaps.push(input_b);
                    swaps.push(replacement);
                    swap(gates, wire_usages, origin_map, input_b, replacement);

                    gate_idx
                }
                (None, None) => unreachable!("both inputs have been swapped :("),
            }
        }
        Some(gate_idx) => gate_idx,
    }
}

fn find_swaps<'a>(
    mut gates: Vec<(GateInput<'a>, GateInput<'a>, Op, &'a str)>,
    mut initial_values: Vec<(&'a str, bool)>,
    wire_usages: &HashMap<&'a str, HashSet<(usize, u8)>>,
    origin_map: &HashMap<&'a str, usize>,
) -> String {
    // circuit must form an adder
    // roll up circuit from the start

    let mut swaps = Vec::new();

    initial_values.sort_by(|(wire_a, _), (wire_b, _)| {
        let num_a: u8 = wire_a[1..].parse().unwrap();
        let num_b: u8 = wire_b[1..].parse().unwrap();
        num_a.cmp(&num_b).then_with(|| {
            wire_a
                .chars()
                .next()
                .unwrap()
                .cmp(&wire_b.chars().next().unwrap())
        })
    });

    let mut n = 0;

    let mut carry_wire_opt: Option<&str> = None;

    loop {
        if 2 * n >= initial_values.len() {
            break;
        }
        let x_input = initial_values[2 * n].0;
        let y_input = initial_values[2 * n + 1].0;

        let first_xor_gate = find_actual_gate(
            x_input,
            y_input,
            Op::Xor,
            &mut gates,
            &mut swaps,
            wire_usages,
            origin_map,
        );
        let first_and_gate = find_actual_gate(
            x_input,
            y_input,
            Op::And,
            &mut gates,
            &mut swaps,
            wire_usages,
            origin_map,
        );

        let first_xor_output = gates[first_xor_gate].3;

        match carry_wire_opt {
            Some(carry_wire) => {
                let second_xor_gate = find_actual_gate(
                    carry_wire,
                    first_xor_output,
                    Op::Xor,
                    &mut gates,
                    &mut swaps,
                    wire_usages,
                    origin_map,
                );

                let second_and_gate = find_actual_gate(
                    carry_wire,
                    gates[first_xor_gate].3,
                    Op::And,
                    &mut gates,
                    &mut swaps,
                    wire_usages,
                    origin_map,
                );

                let actual_output = gates[second_xor_gate].3;

                let expected_output = format!("z{n:02}");
                if actual_output != expected_output {
                    let expected_output_with_correct_lifetime = origin_map
                        .get_key_value(expected_output.as_str())
                        .unwrap()
                        .0;
                    swap(
                        &mut gates,
                        wire_usages,
                        origin_map,
                        actual_output,
                        expected_output_with_correct_lifetime,
                    );
                    swaps.push(actual_output);
                    swaps.push(expected_output_with_correct_lifetime);
                }

                let second_and_output = gates[second_and_gate].3;
                let first_and_output = gates[first_and_gate].3;

                let or_gate = find_actual_gate(
                    first_and_output,
                    second_and_output,
                    Op::Or,
                    &mut gates,
                    &mut swaps,
                    wire_usages,
                    origin_map,
                );

                carry_wire_opt = Some(gates[or_gate].3);
            }
            None => {
                let first_and_output = gates[first_and_gate].3;
                assert_eq!(first_xor_output, "z00");
                carry_wire_opt = Some(first_and_output);
            }
        }

        n += 1;
    }

    swaps.sort();
    swaps.join(",")
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    // let input = std::fs::read_to_string("example.txt").unwrap();
    // let input = std::fs::read_to_string("larger_example.txt").unwrap();

    let mut outputs = Vec::new();
    let mut wire_usages: HashMap<&str, HashSet<(usize, u8)>> = HashMap::new();
    let mut origin_map: HashMap<&str, usize> = HashMap::new();
    let mut gates = Vec::new();

    let (initials_str, gates_str) = input.split_once("\n\n").unwrap();

    let initials: Vec<(&str, bool)> = initials_str
        .lines()
        .map(|line| {
            let (wire, value) = line.split_once(": ").unwrap();
            let value: u8 = value.parse().unwrap();
            let value = match value {
                0 => false,
                1 => true,
                _ => unreachable!(),
            };
            (wire, value)
        })
        .collect();

    for line in gates_str.lines() {
        let mut splits = line.split_ascii_whitespace();
        let a = splits.next().unwrap();
        let op: Op = splits.next().unwrap().try_into().unwrap();
        let b = splits.next().unwrap();
        assert_eq!(splits.next().unwrap(), "->");
        let output = splits.next().unwrap();
        assert!(splits.next().is_none());

        wire_usages.entry(a).or_default().insert((gates.len(), 0));
        wire_usages.entry(b).or_default().insert((gates.len(), 1));

        origin_map.insert(output, gates.len());

        gates.push((GateInput::Wire(a), GateInput::Wire(b), op, output));

        if output.starts_with('z') {
            outputs.push(output);
        }
    }

    outputs.sort();
    outputs.reverse();

    let task1 = run_circuit(gates.clone(), &initials, &wire_usages, &outputs);

    let swaps = find_swaps(gates.clone(), initials.clone(), &wire_usages, &origin_map);

    println!("Task1: {task1}");
    println!("Task2: {swaps}");
}
