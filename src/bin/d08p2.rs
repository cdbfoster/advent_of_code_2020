extern crate advent_of_code;
extern crate regex;

use std::str::FromStr;

use regex::Regex;

use advent_of_code::read_input_list;

fn main() {
    let mut input: Vec<String> = read_input_list().unwrap();

    for i in 0..input.len() {
        let final_value = if input[i].starts_with("nop") {
            swap_run(&mut input, i, "nop", "jmp")
        } else if input[i].starts_with("jmp") {
            swap_run(&mut input, i, "jmp", "nop")
        } else {
            None
        };

        if let Some(acc) = final_value {
            println!("Fixed acc: {}", acc);
            return;
        }
    }
}

fn swap_run(program: &mut [String], i: usize, a: &str, b: &str) -> Option<isize> {
    program[i] = program[i].replace(a, b);
    let result = completes(program);
    program[i] = program[i].replace(b, a);
    result
}

fn completes(program: &[String]) -> Option<isize> {
    let instruction = Regex::new(r"^(acc|jmp|nop) ([+-]\d+)$").unwrap();

    let mut pc = 0;
    let mut acc = 0;
    let mut executed = vec![false; program.len()];

    while pc < program.len() as isize {
        if executed[pc as usize] {
            return None;
        } else {
            executed[pc as usize] = true;
        }

        let c = instruction.captures(&program[pc as usize]).unwrap();

        let operation = c.get(1).unwrap().as_str();
        let operand = isize::from_str(c.get(2).unwrap().as_str()).unwrap();

        match operation {
            "acc" => {
                acc += operand;
                pc += 1;
            },
            "jmp" => pc += operand,
            _ => pc += 1,
        }
    }

    Some(acc)
}