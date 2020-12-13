extern crate advent_of_code;

use advent_of_code::read_input_list;

fn main() {
    let input: Vec<String> = read_input_list().unwrap();

    let buses = input[1]
        .split(",")
        .map(|b| b.parse::<usize>().unwrap_or(0))
        .collect::<Vec<_>>();

    let n = buses.iter().filter(|b| **b != 0).product::<usize>();

    let x = buses
        .iter()
        .zip((0..buses.len()).rev())
        .filter(|(mi, _)| **mi != 0)
        .map(|(mi, i)| (*mi, i % mi, n / mi))
        .map(|(mi, bi, ni)| (mi, bi, ni, (1..).find(|xi| (xi * ni % mi) % mi == 1).unwrap()))
        .map(|(_, bi, ni, xi)| bi * ni * xi)
        .sum::<usize>() % n;

    println!("Earliest time: {}", x - (buses.len() - 1));
}