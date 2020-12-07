extern crate advent_of_code;
extern crate regex;

use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use regex::Regex;

use advent_of_code::read_input_list;

fn main() {
    let input: Vec<String> = read_input_list().unwrap();

    let match_bag = Regex::new(r"^([a-z]+ [a-z]+) bags contain").unwrap();
    let match_contained = Regex::new(r"(?: (\d+) ([a-z]+ [a-z]+) bags?[,.])").unwrap();

    let rules: HashMap<&str, HashSet<(&str, usize)>> = input
        .iter()
        .map(|rule| (
            match_bag.captures(rule).unwrap().get(1).unwrap().as_str(),
            match_contained.captures_iter(rule).map(|c| (
                c.get(2).unwrap().as_str(),
                usize::from_str(c.get(1).unwrap().as_str()).unwrap(),
            )).collect(),
        ))
        .collect();

    fn count_contained(rules: &HashMap<&str, HashSet<(&str, usize)>>, parent: &str) -> usize {
        rules[parent]
            .iter()
            .fold(0, |count, (bag, number)| count + number + number * count_contained(rules, bag))
    }

    println!("Shiny gold bags contain {} other bags.", count_contained(&rules, "shiny gold"));
}