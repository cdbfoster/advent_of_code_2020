extern crate advent_of_code;
extern crate regex;

use std::collections::{HashMap, HashSet};

use regex::Regex;

use advent_of_code::read_input_list;

fn main() {
    let input: Vec<String> = read_input_list().unwrap();

    let food = Regex::new(r"^((?:\w+ )*?\w+) \(contains ((?:\w+, )*?\w+)\)$").unwrap();

    let foods = input
        .iter()
        .map(|l| {
            let c = food.captures(l).unwrap();
            let ingredients = c.get(1).unwrap().as_str().split(" ").collect::<HashSet<_>>();
            let allergens = c.get(2).unwrap().as_str().split(", ").collect::<HashSet<_>>();
            (ingredients, allergens)
        })
        .collect::<Vec<_>>();

    let (allergen_map, all_ingredients) = foods
        .iter()
        .fold(
            (HashMap::new(), HashSet::new()),
            |(mut allergen_map, all_ingredients), (ingredients, allergens)| {
                for a in allergens.iter() {
                    let possible_ingredients = allergen_map.entry(a).or_insert(ingredients.clone());
                    *possible_ingredients = &*possible_ingredients & ingredients;
                }
                (allergen_map, &all_ingredients | &ingredients)
            },
        );

    let allergen_free_ingredients = &all_ingredients - &allergen_map
        .values()
        .fold(HashSet::new(), |allergen_ingredients, i| &allergen_ingredients | i);

    println!(
        "Allergen-free ingredient occurrences: {}",
        foods
            .iter()
            .map(|(i, _)| i.intersection(&allergen_free_ingredients).count())
            .sum::<usize>(),
    );
}