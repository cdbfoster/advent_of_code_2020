extern crate advent_of_code;

use advent_of_code::read_input_list;

fn main() {
    let input: Vec<String> = read_input_list().unwrap();

    let min_time: u64 = input[0].parse().unwrap();
    let (bus, depart_at) = input[1]
        .split(",")
        .filter_map(|b| b.parse::<u64>().ok())
        .map(|b| (b, (min_time / b + 1) * b))
        .min_by_key(|(_, t)| *t)
        .unwrap();

    println!("Waiting minutes * bus: {}", (depart_at - min_time) * bus);
}