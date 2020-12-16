extern crate regex;

use std::collections::HashSet;
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

    let mut fields = (0..rules.len())
        .map(|i| (
            i,
            rules
                .iter()
                .map(|(f, _)| *f)
                .collect::<HashSet<_>>(),
        ))
        .collect::<Vec<_>>();

    sections[1]
        .lines()
        .skip(1)
        .chain(sections[2].lines().skip(1))
        .map(|s| s
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
        )
        .filter(|t| t
            .iter()
            .all(|x| rules
                .iter()
                .flat_map(|r| r.1.iter())
                .any(|(l, h)| l <= x && x <= h)
            )
        )
        .for_each(|t| t
            .iter()
            .zip(fields.iter_mut())
            .for_each(|(x, (_, v))| rules
                .iter()
                .for_each(|r| if !r.1.iter().any(|(l, h)| l <= x && x <= h) {
                    v.remove(r.0);
                })
            )
        );

    fields.sort_by_key(|(_, v)| v.len());

    let mut assignments = fields
        .iter()
        .scan(
            rules
                .iter()
                .map(|(f, _)| *f)
                .collect::<HashSet<_>>(),
            |available, (i, possible)| {
                let assign = *available.intersection(possible).next().unwrap();
                available.remove(assign);
                Some((i, assign))
            },
        )
    .collect::<Vec<_>>();
    assignments.sort_by_key(|(i, _)| *i);

    println!(
        "Product of departure* fields: {}",
        sections[1]
            .lines()
            .skip(1)
            .flat_map(|s| s
                .split(",")
                .map(|v| v.parse::<usize>().unwrap())
            )
            .zip(assignments)
            .filter_map(|(x, (_, f))| if f.starts_with("departure") {
                Some(x)
            } else {
                None
            })
            .product::<usize>(),
    );
}