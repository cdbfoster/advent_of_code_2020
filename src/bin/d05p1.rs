extern crate advent_of_code;

use advent_of_code::read_input_list;

fn main() {
    let input: Vec<String> = read_input_list().unwrap();

    println!(
        "Final seat index: {}",
        input
            .iter()
            .map(|l| usize::from_str_radix(
                &l
                    .replace("F", "0")
                    .replace("B", "1")
                    .replace("L", "0")
                    .replace("R", "1"),
                2,
            ).unwrap())
            .max()
            .unwrap(),
    );
}