extern crate advent_of_code;

use std::collections::HashSet;

use advent_of_code::read_input_list;

fn main() {
    let input: Vec<i64> = read_input_list().unwrap();

    println!(
        "Broken sequence value: {}",
        input
            .iter()
            .enumerate()
            .skip(25)
            .find_map(|(i, &x)| {
                let mut differences = HashSet::new();

                for v in input[i - 25..i].iter() {
                    differences.insert(x - v);

                    if differences.contains(v) {
                        return None;
                    }
                }
                Some(x)
            })
            .unwrap(),
    );
}