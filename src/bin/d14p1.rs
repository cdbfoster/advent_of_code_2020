extern crate advent_of_code;
extern crate regex;

use std::collections::HashMap;

use regex::Regex;

use advent_of_code::read_input_list;

fn main() {
    let input: Vec<String> = read_input_list().unwrap();

    let instruction_match = Regex::new(r"^(mask|mem)(?:\[(\d+)\])? = (.+)$").unwrap();

    let mut memory = HashMap::new();
    let mut ones = 0u64;
    let mut zeros = 0u64;

    for s in input.iter() {
        let c = instruction_match.captures(s).unwrap();
        let instruction = c.get(1).unwrap().as_str();

        match instruction {
            "mask" => {
                let value = c.get(3).unwrap().as_str();
                ones = u64::from_str_radix(&value.replace("X", "0"), 2).unwrap();
                zeros = u64::from_str_radix(&value.replace("X", "1"), 2).unwrap();
            },
            "mem" => {
                let address: usize = c.get(2).unwrap().as_str().parse().unwrap();
                let value: u64 = (c.get(3).unwrap().as_str().parse::<u64>().unwrap() | ones) & zeros;
                memory.insert(address, value);
            },
            _ => panic!(),
        }
    }

    println!("Sum: {}", memory.values().sum::<u64>());
}