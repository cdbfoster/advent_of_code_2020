extern crate advent_of_code;

use advent_of_code::read_input_list;

fn main() {
    let input: Vec<String> = read_input_list().unwrap();

    let width = input[0].len();
    let mut position = 0;
    let mut trees = 0;

    for line in input.iter() {
        if line.chars().nth(position) == Some('#') {
            trees += 1;
        }
        position += 3;
        position %= width;
    }

    println!("{} trees", trees);
}