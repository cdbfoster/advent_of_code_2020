extern crate regex;

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();

    let id = Regex::new(r"Tile (\d+):").unwrap();

    println!("Product of corner ids: {}", input
        .split("\n\n")
        .map(|s| (
            id.captures(s).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap(),
            s,
        ))
        .fold(HashMap::new(), |edge_map, (id, s)| {
            let edges = vec![
                Edge::new(s.lines().skip(1).next().unwrap().to_string()),
                Edge::new(s.lines().skip(1).map(|l| l.chars().last().unwrap()).collect()),
                Edge::new(s.lines().skip(1).last().unwrap().to_string()),
                Edge::new(s.lines().skip(1).map(|l| l.chars().next().unwrap()).collect()),
            ];

            edges
                .into_iter()
                .fold(edge_map, |mut edge_map, e| {
                    edge_map.entry(e).or_insert(HashSet::new()).insert(id);
                    edge_map
                })
        })
        .values()
        .filter(|ids| ids.len() == 1)
        .fold(HashMap::new(), |mut unique_edges, ids| {
            *unique_edges.entry(*ids.iter().next().unwrap()).or_insert(0) += 1;
            unique_edges
        })
        .into_iter()
        .filter(|(_, unique_edges)| *unique_edges == 2)
        .map(|(id, _)| id)
        .product::<usize>());
}

#[derive(Eq, PartialEq, Hash)]
struct Edge(String, String);

impl Edge {
    fn new(e: String) -> Self {
        let r = e.chars().rev().collect();
        if e < r {
            Self(e, r)
        } else {
            Self(r, e)
        }
    }
}