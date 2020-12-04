extern crate regex;

use std::collections::{HashSet, HashMap};
use std::env;
use std::fs;
use std::str::FromStr;

use regex::Regex;

fn main() {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();

    let attribute = Regex::new(r"([a-z]{3}):([^\s\n]+)").unwrap();
    let height = Regex::new(r"^(\d{2,3})(cm|in)$").unwrap();
    let haircolor = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let eyecolor = Regex::new(r"^(?:amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    let passportid = Regex::new(r"^\d{9}$").unwrap();

    let required_attributes = [
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid",
    ].iter().collect::<HashSet<_>>();

    println!(
        "{} valid passports",
        input
            .split("\n\n")
            .filter_map(|p| {
                let attributes = attribute
                    .captures_iter(p)
                    .map(|c| (
                        c.get(1).unwrap().as_str(),
                        c.get(2).unwrap().as_str(),
                    ))
                    .collect::<HashMap<_, _>>();

                if required_attributes.difference(&attributes.keys().collect::<HashSet<_>>()).count() == 0 {
                    Some(attributes)
                } else {
                    None
                }
            })
            .filter(|a| valid_range(a["byr"], 1920, 2002))
            .filter(|a| valid_range(a["iyr"], 2010, 2020))
            .filter(|a| valid_range(a["eyr"], 2020, 2030))
            .filter(|a| {
                height.captures(a["hgt"]).map(|c| {
                    match c.get(2).unwrap().as_str() {
                        "cm" => valid_range(c.get(1).unwrap().as_str(), 150, 193),
                        "in" => valid_range(c.get(1).unwrap().as_str(), 59, 76),
                        _ => false,
                    }
                })
                .unwrap_or(false)
            })
            .filter(|a| haircolor.is_match(a["hcl"]))
            .filter(|a| eyecolor.is_match(a["ecl"]))
            .filter(|a| passportid.is_match(a["pid"]))
            .count(),
    );
}

fn valid_range<T>(value: &str, low: T, high: T) -> bool
where
    T: FromStr + PartialOrd,
{
    T::from_str(value).map(|v| v >= low && v <= high).unwrap_or(false)
}