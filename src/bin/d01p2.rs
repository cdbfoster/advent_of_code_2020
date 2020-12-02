extern crate advent_of_code;

use std::collections::HashSet;
use advent_of_code::read_input_list;

fn main() {
    let input: Vec<u64> = read_input_list().unwrap();
    let mut differences = vec![HashSet::new(); input.len()];

    for (x, d) in input.iter().zip(differences.iter_mut()) {
        for y in input.iter() {
            d.insert(2020 - *x as i64 - *y as i64);

            if d.contains(&(*y as i64)) {
                println!("{} + {} + {} = 2020", x, y, 2020 - x - y);
                println!("{} * {} * {} = {}", x, y, 2020 - x - y, x * y * (2020 - x - y));
                return;
            }
        }
    }
}