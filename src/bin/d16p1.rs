extern crate regex;

use std::env;
use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let sections = input.split("\n\n").collect::<Vec<_>>();

    let rule_match = Regex::new(r"^([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();

    let rules = sections[0].lines().map(|s| {
        let c = rule_match.captures(s).unwrap();
        let name = c.get(1).unwrap().as_str();
        let l1: usize = c.get(2).unwrap().as_str().parse().unwrap();
        let h1: usize = c.get(3).unwrap().as_str().parse().unwrap();
        let l2: usize = c.get(4).unwrap().as_str().parse().unwrap();
        let h2: usize = c.get(5).unwrap().as_str().parse().unwrap();
        (name, [(l1, h1), (l2, h2)])
    }).collect::<Vec<_>>();

    println!(
        "Ticket scanning error rate: {}",
        sections[2]
            .lines()
            .skip(1)
            .flat_map(|s| s.split(","))
            .map(|v| v.parse::<usize>().unwrap())
            .filter(|x| !rules
                .iter()
                .flat_map(|r| r.1.iter())
                .any(|(l, h)| l <= x && x <= h)
            )
            .sum::<usize>(),
    );

}