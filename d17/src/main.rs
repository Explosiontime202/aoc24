use regex::Regex;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Instruction {
    Adv(ComboOperand),
    Bxl(u8),
    Bst(ComboOperand),
    Jnz(u8),
    Bxc,
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ComboOperand {
    Literal(u8),
    A,
    B,
    C,
}

#[derive(Clone, Copy, Debug)]
#[allow(non_snake_case)]
struct State {
    A: u64,
    B: u64,
    C: u64,
}

fn simulate(mut state: State, instrs: &Vec<Instruction>) -> Vec<u8> {
    let mut pc = 0;
    let mut outputs = Vec::new();
    loop {
        let get_combo_val = |state: &State, combo: ComboOperand| match combo {
            ComboOperand::Literal(literal) => literal as u64,
            ComboOperand::A => state.A,
            ComboOperand::B => state.B,
            ComboOperand::C => state.C,
        };
        if let Some(&instr) = instrs.get(pc) {
            // println!("{state:?}");
            // println!("{pc}: {instr:?}");
            match instr {
                Instruction::Adv(combo_operand) => {
                    state.A /= 2u64.pow(get_combo_val(&state, combo_operand).try_into().unwrap())
                }
                Instruction::Bxl(literal) => state.B ^= literal as u64,
                Instruction::Bst(combo_operand) => {
                    state.B = get_combo_val(&state, combo_operand) % 8
                }
                Instruction::Jnz(literal) => {
                    if state.A != 0 {
                        pc = literal as usize;
                        continue;
                    }
                }
                Instruction::Bxc => state.B ^= state.C,
                Instruction::Out(combo_operand) => {
                    outputs.push((get_combo_val(&state, combo_operand) % 8) as u8);
                }
                Instruction::Bdv(combo_operand) => {
                    state.B = state.A
                        / (2u64.pow(get_combo_val(&state, combo_operand).try_into().unwrap()))
                }
                Instruction::Cdv(combo_operand) => {
                    state.C = state.A
                        / (2u64.pow(get_combo_val(&state, combo_operand).try_into().unwrap()))
                }
            }
            pc += 1;
        } else {
            break outputs;
        }
    }
}

fn find_initial_brute_force(instrs: &Vec<Instruction>, input_prog: &Vec<u8>) -> State {
    let mut a = 0;
    loop {
        if a % 1000 == 0 {
            print!("\r{}", a);
        }
        let state = State { A: a, B: 0, C: 0 };
        let output = simulate(state, instrs);
        if &output == input_prog {
            println!("");
            break state;
        }
        a += 1;
    }
}

fn find_initial_smart(input_prog: &Vec<u8>) -> u64 {
    let mut queue = vec![(0, 0)];
    let mut min_initial = None;
    while let Some((a, skip)) = queue.pop() {
        println!("{}", queue.len());
        if let Some(min_initial) = min_initial {
            if a >= min_initial {
                continue;
            }
        }
        if skip == input_prog.len() {
            if let Some(min_initial) = min_initial.as_mut() {
                *min_initial = a;
            } else {
                min_initial = Some(a);
            }
            continue;
        }
        let input_byte = input_prog.iter().rev().skip(skip).next().unwrap();
        for b in 0..8 {
            let b_prime = b ^ 6;
            // println!("{b:#x}, {}, {}", b_prime ^ input_byte ^ 7, ((((a << 3) | b as u64) >> b_prime) % 8));
            let new_a = (a << 3) | b as u64;
            if (b_prime ^ input_byte ^ 7) == ((new_a >> b_prime) % 8) as u8 {
                queue.push((new_a, skip + 1));
            }
        }
    }
    min_initial.unwrap()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    // let input = std::fs::read_to_string("example.txt").unwrap();
    // let input = std::fs::read_to_string("example2.txt").unwrap();

    let mut input = input.split("\n\n");

    let register_re =
        Regex::new(r"Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)").unwrap();
    let instr_re = Regex::new(r"((\d+),(\d+))").unwrap();

    let mut state = {
        let cap = register_re.captures(input.next().unwrap()).unwrap();
        State {
            A: cap[1].parse().unwrap(),
            B: cap[2].parse().unwrap(),
            C: cap[3].parse().unwrap(),
        }
    };

    let instr = input.next().unwrap();
    assert!(instr.starts_with("Program: "));
    let mut input_prog = Vec::new();
    let instr: Vec<Instruction> = instr_re
        .captures_iter(instr)
        .map(|cap| {
            let opcode: u8 = cap[2].parse().unwrap();
            let operand: u8 = cap[3].parse().unwrap();

            input_prog.push(opcode);
            input_prog.push(operand);

            assert!(opcode < 8);
            assert!(operand < 8);

            let make_combo_op = || match operand {
                0..=3 => ComboOperand::Literal(operand),
                4 => ComboOperand::A,
                5 => ComboOperand::B,
                6 => ComboOperand::C,
                _ => unreachable!(),
            };

            match opcode {
                0 => Instruction::Adv(make_combo_op()),
                1 => Instruction::Bxl(operand),
                2 => Instruction::Bst(make_combo_op()),
                3 => Instruction::Jnz(operand / 2),
                4 => Instruction::Bxc,
                5 => Instruction::Out(make_combo_op()),
                6 => Instruction::Bdv(make_combo_op()),
                7 => Instruction::Cdv(make_combo_op()),
                _ => unreachable!(),
            }
        })
        .collect();

    println!("{instr:?}");

    let output_a = simulate(state, &instr);
    // let output_b_brute_force = find_initial_brute_force(&instr, &input_prog).A;
    let output_b_smart = find_initial_smart(&input_prog);

    let output_a = output_a
        .into_iter()
        .map(|o| o.to_string())
        .collect::<Vec<_>>();

    println!("Task1: {}", output_a.join(","));
    // println!("Task2: {output_b_brute_force}");
    println!("Task2: {output_b_smart}");
}
