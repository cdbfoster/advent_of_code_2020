extern crate advent_of_code;

use advent_of_code::read_input_list;

fn main() {
    let input: Vec<String> = read_input_list().unwrap();

    let mut seats = input
        .iter()
        .map(|l| usize::from_str_radix(
            &l
                .replace("F", "0")
                .replace("B", "1")
                .replace("L", "0")
                .replace("R", "1"),
            2,
        ).unwrap())
        .collect::<Vec<_>>();
    seats.sort();

    for (i, x) in seats.iter().enumerate().skip(1) {
        if x - seats[i - 1] > 1 {
            println!("Seat {} is missing.", x - 1);
        }
    }
}