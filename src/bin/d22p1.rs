use std::env;
use std::fs;

fn main() {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let mut decks = input
        .split("\n\n")
        .map(|s| s
            .lines()
            .skip(1)
            .map(|l| l.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>()
        .into_iter();

    let mut p1 = decks.next().unwrap();
    let mut p2 = decks.next().unwrap();

    while !(p1.is_empty() || p2.is_empty()) {
        let c1 = p1.remove(0);
        let c2 = p2.remove(0);

        if c1 > c2 {
            p1.push(c1);
            p1.push(c2);
        } else {
            p2.push(c2);
            p2.push(c1);
        }
    }

    println!(
        "Winning score: {}",
        if p1.is_empty() {
            p2
        } else {
            p1
        }.iter().rev().enumerate().map(|(i, c)| c * (i + 1)).sum::<usize>(),
    );
}
