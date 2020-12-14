extern crate advent_of_code;
extern crate regex;

use std::collections::HashMap;

use regex::Regex;

use advent_of_code::read_input_list;

fn main() {
    let input: Vec<String> = read_input_list().unwrap();

    let instruction_match = Regex::new(r"^(mask|mem)(?:\[(\d+)\])? = (.+)$").unwrap();

    let mut memory = HashMap::new();
    let mut ones = 0usize;
    let mut float = 0usize;

    for s in input.iter() {
        let c = instruction_match.captures(s).unwrap();
        let instruction = c.get(1).unwrap().as_str();

        match instruction {
            "mask" => {
                let value = c.get(3).unwrap().as_str();
                ones = usize::from_str_radix(&value.replace("X", "0"), 2).unwrap();
                float = usize::from_str_radix(&value.replace("1", "0").replace("X", "1"), 2).unwrap();
            },
            "mem" => {
                let address: usize = c.get(2).unwrap().as_str().parse::<usize>().unwrap() | ones;
                let value: u64 = c.get(3).unwrap().as_str().parse().unwrap();

                for i in 0..2usize.pow(float.count_ones()) {
                    memory.insert(spread_bits(float, i, address), value);
                }
            },
            _ => panic!(),
        }
    }

    println!("Sum: {}", memory.values().sum::<u64>());
}

/// Spreads value across target, filling the bits set in mask
fn spread_bits(mask: usize, value: usize, target: usize) -> usize {
    (0..64)
        .map(|i| (i, mask & (0x01 << i)))
        .filter_map(|(i, m)| if m > 0 {
            Some(i)
        } else {
            None
        })
        .enumerate()
        .fold(target, |v, (i, l)| if value & (0x01 << i) > 0 {
            v | (0x01 << l)
        } else {
            v & !(0x01 << l)
        })
}