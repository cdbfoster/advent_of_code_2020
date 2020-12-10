extern crate advent_of_code;

use advent_of_code::read_input_list;

fn main() {
    let mut input: Vec<u32> = read_input_list().unwrap();
    input.sort();
    input.insert(0, 0);
    input.push(input.last().unwrap() + 3);

    let branches = input
        .iter()
        .enumerate()
        .map(|(i, &x)| (
            i,
            input[i + 1..input.len().min(i + 4)]
                .iter()
                .filter(|&y| y - x <= 3)
                .count(),
        ));

    let mut paths = vec![0u64; input.len()];
    paths[0] = 1;
    for (i, b) in branches {
        for j in i + 1..=i + b {
            paths[j] += paths[i]
        }
    }

    println!("Possible combinations: {}", paths.last().unwrap());
}