extern crate advent_of_code;

use std::collections::HashSet;

use advent_of_code::read_input_list;

fn main() {
    let input: Vec<i64> = read_input_list().unwrap();

    let target = input
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
        .unwrap();

    println!(
        "Range ends sum: {}",
        input
            .iter()
            .enumerate()
            .find_map(|(i, &x)| {
                let mut acc = x;
                let mut min = x;
                let mut max = x;

                for j in i + 1..input.len() {
                    acc += input[j];
                    min = min.min(input[j]);
                    max = max.max(input[j]);

                    if acc == target {
                        return Some(min + max);
                    } else if acc > target {
                        break;
                    }
                }
                None
            })
            .unwrap(),
    );
}