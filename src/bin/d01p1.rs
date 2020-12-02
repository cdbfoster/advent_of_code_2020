extern crate advent_of_code;

use std::collections::HashSet;
use advent_of_code::read_input_list;

fn main() {
    let input: Vec<u64> = read_input_list().unwrap();
    let mut differences = HashSet::new();

    for x in input.iter() {
        differences.insert(2020 - x);

        if differences.contains(&x) {
            println!("{} + {} = 2020", x, 2020 - x);
            println!("{} * {} = {}", x, 2020 - x, x * (2020 - x));
            return;
        }
    }
}