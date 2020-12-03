extern crate advent_of_code;

use advent_of_code::read_input_list;

fn main() {
    let input: Vec<String> = read_input_list().unwrap();

    let width = input[0].len();
    let intervals = [1, 3, 5, 7];
    let mut positions = [0, 0, 0, 0, 0];
    let mut trees = [0, 0, 0, 0, 0];

    for (i, line) in input.iter().enumerate() {
        // The 1 down slopes
        for ((interval, position), tree_count) in intervals.iter().zip(positions.iter_mut()).zip(trees.iter_mut()) {
            if line.chars().nth(*position) == Some('#') {
                *tree_count += 1;
            }
            *position += interval;
            *position %= width;
        }

        // The 2 down slope
        if i % 2 == 0 {
            if line.chars().nth(positions[4]) == Some('#') {
                trees[4] += 1;
            }
            positions[4] += 1;
            positions[4] %= width;
        }
    }

    println!("{:?} trees", trees);
    println!("Product is {}", trees.iter().product::<u64>());
}