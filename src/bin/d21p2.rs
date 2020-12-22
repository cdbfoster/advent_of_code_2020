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

    let (mut allergen_map, all_allergens) = foods
        .iter()
        .fold(
            (HashMap::new(), HashSet::new()),
            |(mut allergen_map, all_allergens), (ingredients, allergens)| {
                for a in allergens.iter() {
                    let possible_ingredients = allergen_map.entry(*a).or_insert(ingredients.clone());
                    *possible_ingredients = &*possible_ingredients & ingredients;
                }
                (allergen_map, &all_allergens | &allergens)
            },
        );

     loop {
        let assigned_allergens = allergen_map.iter().filter(|(_, v)| v.len() == 1).map(|(k, _)| *k).collect::<HashSet<_>>();
        let unassigned_allergens = &all_allergens - &assigned_allergens;

        if unassigned_allergens.is_empty() {
            break;
        }

        let assigned_ingredients = assigned_allergens.iter().fold(HashSet::new(), |h, a| &h | &allergen_map[a]);

        for a in unassigned_allergens {
            allergen_map.insert(a, &allergen_map[a] - &assigned_ingredients);
        }
    }

    let mut allergen_ingredients = allergen_map
        .iter()
        .map(|(k, v)| (*k, *v.iter().next().unwrap()))
        .collect::<Vec<_>>();
    allergen_ingredients.sort();

    println!(
        "Dangerous ingredient list: {}",
        allergen_ingredients
            .iter()
            .map(|(_, v)| v.to_string())
            .collect::<Vec<_>>()
            .join(","),
    );
}