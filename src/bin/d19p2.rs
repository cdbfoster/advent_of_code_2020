extern crate regex;

use std::collections::HashMap;
use std::env;
use std::fs;

use regex::Regex;

enum Rule<'a> {
    Value(&'a str),
    Composition([Option<usize>; 2], [Option<usize>; 2]),
}

impl<'a> Rule<'a> {
    fn compose(&self, rules: &HashMap<usize, Rule>) -> String {
        match self {
            Rule::Value(c) => c.to_string(),
            Rule::Composition(a, b) => {
                let rule_a: String = a
                    .iter()
                    .filter(|c| c.is_some())
                    .map(|c| rules[&c.unwrap()].compose(rules))
                    .collect::<Vec<_>>()
                    .iter()
                    .flat_map(|s| s.chars())
                    .collect();
                let rule_b: String = b
                    .iter()
                    .filter(|c| c.is_some())
                    .map(|c| rules[&c.unwrap()].compose(rules))
                    .collect::<Vec<_>>()
                    .iter()
                    .flat_map(|s| s.chars())
                    .collect();
                if rule_b.is_empty() {
                    rule_a
                } else {
                    format!("(?:{}|{})", rule_a, rule_b)
                }
            },
        }
    }
}

fn main() {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let sections = input.split("\n\n").collect::<Vec<_>>();

    let rule = Regex::new(r#"^(\d+): "?(\d+|[ab])"?(?: (\d+))?(?: \| (\d+)(?: (\d+))?)?$"#).unwrap();

    let rules = sections[0].lines().map(|l| {
        let c = rule.captures(l).unwrap();
        (
            c.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            match c.get(2).unwrap().as_str() {
                "a" | "b" => Rule::Value(c.get(2).unwrap().as_str()),
                _ => Rule::Composition(
                    [
                        c.get(2).map(|c| c.as_str().parse().ok()).flatten(),
                        c.get(3).map(|c| c.as_str().parse().ok()).flatten(),
                    ],
                    [
                        c.get(4).map(|c| c.as_str().parse().ok()).flatten(),
                        c.get(5).map(|c| c.as_str().parse().ok()).flatten(),
                    ],
                ),
            },
        )
    })
    .collect::<HashMap<_, _>>();

    let r42 = rules[&42].compose(&rules);
    let r31 = rules[&31].compose(&rules);

    let rg = Regex::new(&format!("^((?:{})+)((?:{})+)$", r42, r31)).unwrap();
    let rg42 = Regex::new(&r42).unwrap();
    let rg31 = Regex::new(&r31).unwrap();

    println!(
        "Matching strings: {}",
        sections[1]
            .lines()
            .filter_map(|l| rg.captures(l))
            .filter(|c|
                rg42.find_iter(c.get(1).unwrap().as_str()).count() >
                rg31.find_iter(c.get(2).unwrap().as_str()).count()
            )
            .count(),
    );
}