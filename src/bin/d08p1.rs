extern crate advent_of_code;
extern crate regex;

use std::str::FromStr;

use regex::Regex;

use advent_of_code::read_input_list;

fn main() {
    let input: Vec<String> = read_input_list().unwrap();

    let instruction = Regex::new(r"^(acc|jmp|nop) ([+-]\d+)$").unwrap();

    let mut pc = 0;
    let mut acc = 0;
    let mut executed = vec![false; input.len()];

    loop {
        if executed[pc as usize] {
            println!("Infinite loop detected. acc: {}", acc);
            return;
        } else {
            executed[pc as usize] = true;
        }

        let c = instruction.captures(&input[pc as usize]).unwrap();

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
}