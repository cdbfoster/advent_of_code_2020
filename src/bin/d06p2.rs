use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();

    println!(
        "{} questions agreed upon.",
        input
            .split("\n\n")
            .map(|group| group
                .lines()
                .map(|person| person
                    .chars()
                    .collect::<HashSet<_>>()
                ).fold(
                    "abcdefghijklmnopqrstuvwxyz"
                        .chars()
                        .collect::<HashSet<_>>(),
                    |consensus, person| consensus
                        .intersection(&person)
                        .copied()
                        .collect(),
                )
                .len()
            )
            .sum::<usize>(),
    );
}