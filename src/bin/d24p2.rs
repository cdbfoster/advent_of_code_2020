extern crate advent_of_code;
extern crate regex;

use std::collections::{HashMap, HashSet};

use regex::Regex;

use advent_of_code::read_input_list;

fn main() {
    let input: Vec<String> = read_input_list().unwrap();

    let direction = Regex::new(r"([ns]?[ew])").unwrap();

    let mut black_tiles = input
        .iter()
        .map(|l| direction.captures_iter(l)
            .map(|c| c.get(1).unwrap().as_str())
            .map(|c| match c {
                "w" => (-2, 0),
                "e" => (2, 0),
                "nw" => (-1, 1),
                "ne" => (1, 1),
                "sw" => (-1, -1),
                "se" => (1, -1),
                _ => panic!("impossible"),
            })
            .fold((0, 0), |(x, y), (ox, oy)| (x + ox, y + oy))
        )
        .fold(HashSet::new(), |mut flipped, tile| {
            if !flipped.insert(tile) {
                flipped.remove(&tile);
            }
            flipped
        });

    for _ in 0..100 {
        let black_neighbors = black_tiles
            .iter()
            .flat_map(|(x, y)| [(-2, 0), (2, 0), (-1, 1), (1, 1), (-1, -1), (1, -1)].iter().map(move |&(ox, oy)| (x + ox, y + oy)))
            .fold(HashMap::new(), |mut black_neighbors, c| {
                *black_neighbors.entry(c).or_insert(0) += 1;
                black_neighbors
            });

        black_tiles = black_tiles
            .iter()
            .filter(|&c| black_neighbors.contains_key(c))
            .filter(|&c| black_neighbors[c] <= 2)
            .copied()
            .collect();

        let new_black_tiles = black_neighbors
            .iter()
            .filter(|(_, &n)| n == 2)
            .filter(|(c, _)| !black_tiles.contains(c))
            .map(|(&c, _)| c)
            .collect::<HashSet<_>>();

        black_tiles = &black_tiles | &new_black_tiles;
    }

    println!("{:?}", black_tiles.len());
}