extern crate regex;

use std::collections::HashSet;
use std::env;
use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();

    let attribute = Regex::new(r"([a-z]{3}):[^\s\n]+").unwrap();

    let required_attributes = vec![
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid",
    ].into_iter().collect::<HashSet<_>>();

    println!(
        "{} valid passports",
        input
            .split("\n\n")
            .filter(|p| {
                let attributes = attribute
                    .captures_iter(p)
                    .map(|c| c.get(1).unwrap().as_str())
                    .collect::<HashSet<_>>();

                required_attributes.difference(&attributes).count() == 0
            })
            .count(),
    );
}