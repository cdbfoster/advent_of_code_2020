use std::env;
use std::fs;

fn main() {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let mut cups = input.chars().rev().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<_>>();

    println!("{:?}", cups.iter().rev().collect::<Vec<_>>());

    for _ in 0..100 {
        cups.rotate_right(1);
        let grabbed = cups.split_off(cups.len() - 3);
        cups.rotate_left(1);

        let destination = cups
            .iter()
            .map(|&c| *cups.last().unwrap() - c)
            .enumerate()
            .filter(|(_, c)| *c > 0)
            .min_by_key(|&(_, c)| c)
            .or(
                cups
                .iter()
                .copied()
                .enumerate()
                .max_by_key(|&(_, c)| c),
            )
            .unwrap().0;

        cups.splice(destination..destination, grabbed);
        cups.rotate_right(1);
    }

    println!("{}", cups
            .iter()
            .rev()
            .cycle()
            .skip_while(|&&c| c != 1)
            .skip(1).
            take(cups.len() - 1)
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(""),
        );


}