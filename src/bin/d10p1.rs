extern crate advent_of_code;

use advent_of_code::read_input_list;

fn main() {
    let mut input: Vec<u32> = read_input_list().unwrap();
    input.sort();
    input.insert(0, 0);
    input.push(input.last().unwrap() + 3);

    let ones = input
        .iter()
        .enumerate()
        .skip(1)
        .filter(|(i, &x)| x - input[i - 1] == 1)
        .count();

    let threes = input
        .iter()
        .enumerate()
        .skip(1)
        .filter(|(i, &x)| x - input[i - 1] == 3)
        .count();

    println!("Ones * threes: {}", ones * threes);
}