use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let n: usize = env::args().nth(2).unwrap().parse().unwrap();

    let mut numbers = HashMap::new();
    println!(
        "Value {} in the sequence: {}",
        n,
        input
            .split(",")
            .map(|x| x.parse::<usize>().ok())
            .chain(std::iter::once(None).cycle())
            .take(n - 1)
            .enumerate()
            .fold(None, |last, (i, start_value)| Some(
                i - numbers.insert(start_value.or(last).unwrap(), i).unwrap_or(i)
            ))
            .unwrap(),
    );
}