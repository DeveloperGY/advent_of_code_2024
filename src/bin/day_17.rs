use std::fs;

fn main() {
    let input = fs::read_to_string("Input_Day_17").unwrap();
    let (register_input, instruction_input) = parse_input(&input);

    // let program = Program::new(&instruction_input);
    let out = Cpu::run(register_input[0]);
    println!("Part 1 Output: {:?}", out);

    let part_2_guess = find_part_2(&instruction_input).unwrap();
    println!("Part 2 Guess: {}", part_2_guess);
    let part_2_out = Cpu::run(part_2_guess);
    println!("{:?}", part_2_out);
    println!("{:?}", instruction_input);
}

fn parse_input(input: &str) -> ([u64; 3], Vec<u8>) {
    let (registers_str, program_str) = input.split_once("\n\n").unwrap();
    let mut registers = [0; 3];
    for (i, line) in registers_str.lines().enumerate() {
        let (_, num_str) = line.split_once(":").unwrap();
        let val = num_str.trim().parse::<u64>().unwrap();
        registers[i] = val;
    }

    let instructions = program_str
        .trim_start_matches("Program:")
        .trim()
        .split(',')
        .map(|v| v.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    (registers, instructions)
}

struct Cpu;

impl Cpu {
    pub fn run(mut a: u64) -> Vec<u8> {
        let mut output = vec![];

        if a == 0 {
            return vec![5];
        }

        while a != 0 {
            let mut b = a % 8;
            b = b ^ 1;
            let c = a >> b;
            b = b ^ c;
            b = b ^ 4;
            output.push((b % 8) as u8);
            a >>= 3;
        }

        output
    }
}

fn recurse_find_part_2(program: &[u8], depth: usize, current_a: u64) -> Option<u64> {
    if depth == program.len() {
        return Some(current_a);
    }

    let target_instruction_value = *program.iter().rev().nth(depth).unwrap() as u64;
    println!("Target: {}", target_instruction_value);

    let mut possible_a_list = vec![];
    for possible_a in 0..8 {
        let theoretical_a = (current_a << 3) | possible_a;
        let mut b = theoretical_a % 8;
        b ^= 1;
        let c = theoretical_a >> b;
        b ^= c;

        if (b ^ 4) % 8 == target_instruction_value {
            possible_a_list.push(theoretical_a);
        }
    }

    if possible_a_list.is_empty() {
        return None;
    }

    let continuations = possible_a_list
        .iter()
        .map(|a| recurse_find_part_2(program, depth + 1, *a))
        .filter(|res| res.is_some())
        .map(|res| res.unwrap())
        .collect::<Vec<_>>();

    if continuations.is_empty() {
        None
    } else {
        println!("# of possibilities: {}", continuations.len());
        Some(continuations[0])
    }
}

fn find_part_2(input: &[u8]) -> Option<u64> {
    recurse_find_part_2(input, 0, 0)
}
