use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();

    println!(
        "{} questions answered.",
        input
            .split("\n\n")
            .map(|group| group
                .replace("\n", "")
                .chars()
                .collect::<HashSet<_>>()
                .len()
            )
            .sum::<usize>(),
    );
}