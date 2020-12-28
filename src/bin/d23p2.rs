use std::env;
use std::fs;

fn main() {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let start = input.chars().map(|c| c.to_digit(10).unwrap() as usize - 1).collect::<Vec<_>>();

    let mut cups = (1..=1_000_000).collect::<Vec<_>>();
    *cups.last_mut().unwrap() = start[0];
    start.iter().skip(1).chain(std::iter::once(&start.len())).fold(start[0], |last, &c| {
        cups[last] = c;
        c
    });

    let mut current = start[0];
    let mut grabbed = [0; 3];

    for _ in 0..10_000_000 {
        grabbed[0] = cups[current];
        for i in 1..3 {
            grabbed[i] = cups[grabbed[i - 1]];
        }
        cups[current] = cups[grabbed[2]];

        let mut target = current;
        loop {
            if target == 0 {
                target = 1_000_000;
            }
            target -= 1;

            if !grabbed.iter().any(|&c| c == target) {
                break;
            }
        }

        let after = cups[target];
        cups[target] = grabbed[0];
        cups[grabbed[2]] = after;

        current = cups[current];
    }

    let c1 = cups[0];
    let c2 = cups[c1];

    println!("{:?}", (c1 + 1) * (c2 + 1));
}