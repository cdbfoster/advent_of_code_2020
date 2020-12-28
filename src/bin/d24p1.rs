extern crate advent_of_code;
extern crate regex;

use std::collections::HashSet;

use regex::Regex;

use advent_of_code::read_input_list;

fn main() {
    let input: Vec<String> = read_input_list().unwrap();

    let direction = Regex::new(r"([ns]?[ew])").unwrap();

    let x = input
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

    println!("{:?}", x.len());
}