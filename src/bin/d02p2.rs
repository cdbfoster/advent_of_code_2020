extern crate advent_of_code;
extern crate regex;

use std::str::FromStr;

use regex::Regex;
use advent_of_code::read_input_list;

fn main() {
    let input: Vec<String> = read_input_list().unwrap();

    let parser = Regex::new(r"^(\d{1,2})-(\d{1,2}) ([a-z]): (.+)$").unwrap();

    let mut successes = 0;
    for line in input.iter() {
        let matches = parser.captures(line).unwrap();

        let low = usize::from_str(matches.get(1).unwrap().as_str()).unwrap() - 1;
        let high = usize::from_str(matches.get(2).unwrap().as_str()).unwrap() - 1;
        let letter = matches.get(3).unwrap().as_str().chars().nth(0).unwrap();
        let password = matches.get(4).unwrap().as_str();

        if (password.chars().nth(low).unwrap() == letter) ^ (password.chars().nth(high).unwrap() == letter) {
            successes += 1;
        } else {
            println!("'{}' fails", line);
        }
    }

    println!("{} successes", successes);
}