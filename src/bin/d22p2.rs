use std::collections::HashSet;
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

    let p1 = decks.next().unwrap();
    let p2 = decks.next().unwrap();

    let winner = game(p1, p2);

    println!(
        "Winning score: {}",
        match winner {
            Winner::Player1(p1) => p1,
            Winner::Player2(p2) => p2,
        }.iter().rev().enumerate().map(|(i, c)| c * (i + 1)).sum::<usize>(),
    );
}

type Deck = Vec<usize>;

enum Winner {
    Player1(Deck),
    Player2(Deck),
}

fn game(mut p1: Deck, mut p2: Deck) -> Winner {
    let mut previous = HashSet::new();

    while !(p1.is_empty() || p2.is_empty()) {
        if !previous.insert((p1.clone(), p2.clone())) {
            return Winner::Player1(p1);
        }

        let c1 = p1.remove(0);
        let c2 = p2.remove(0);

        let winner = if c1 <= p1.len() && c2 <= p2.len() {
            let mut p1 = p1.clone();
            let mut p2 = p2.clone();
            p1.truncate(c1);
            p2.truncate(c2);
            game(p1, p2)
        } else if c1 > c2 {
            Winner::Player1(p1.clone())
        } else {
            Winner::Player2(p2.clone())
        };

        match winner {
            Winner::Player1(_) => {
                p1.push(c1);
                p1.push(c2);
            },
            Winner::Player2(_) => {
                p2.push(c2);
                p2.push(c1);
            }
        }
    }

    if p1.is_empty() {
        Winner::Player2(p2)
    } else {
        Winner::Player1(p1)
    }
}