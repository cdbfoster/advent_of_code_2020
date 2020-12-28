extern crate advent_of_code;

use advent_of_code::read_input_list;

fn main() {
    let input: Vec<usize> = read_input_list().unwrap();

    let card_public_key = input[0];
    let door_public_key = input[1];

    let door_loop_size = find_loop_size(door_public_key, 7);

    println!("{:?}", door_loop_size);

    let encryption_key = transform(card_public_key, door_loop_size);

    println!("{:?}", encryption_key);
}

fn find_loop_size(public_key: usize, subject: usize) -> usize {
    (1..).scan(1, |n, ls| {
        *n = (*n * subject) % 20201227;
        Some((ls, *n))
    })
    .find(|(_, n)| *n == public_key)
    .map(|(ls, _)| ls)
    .unwrap()
}

fn transform(subject: usize, loop_size: usize) -> usize {
    (0..loop_size).fold(1, |n, _| (n * subject) % 20201227)
}