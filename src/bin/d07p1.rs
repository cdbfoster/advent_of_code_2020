extern crate advent_of_code;
extern crate regex;

use std::collections::{HashMap, HashSet};

use regex::Regex;

use advent_of_code::read_input_list;

fn main() {
    let input: Vec<String> = read_input_list().unwrap();

    let match_bag = Regex::new(r"^([a-z]+ [a-z]+) bags contain").unwrap();
    let match_contained = Regex::new(r"(?: \d+ ([a-z]+ [a-z]+) bags?[,.])").unwrap();

    let rules: HashMap<&str, HashSet<&str>> = input
        .iter()
        .map(|rule| (
            match_bag.captures(rule).unwrap().get(1).unwrap().as_str(),
            match_contained.captures_iter(rule).map(|c| c.get(1).unwrap().as_str()).collect(),
        ))
        .collect();

    println!(
        "{} bags contain a shiny gold bag.",
        rules
            .keys()
            .filter(|bag| search(&rules, bag, "shiny gold"))
            .count() - 1, // Subtract 1 because shiny gold will find itself
    );
}

fn search(rules: &HashMap<&str, HashSet<&str>>, parent: &str, needle: &str) -> bool {
    parent == needle || rules[parent]
        .iter()
        .any(|bag| search(rules, bag, needle))
}