extern crate advent_of_code;

use advent_of_code::read_input_list;

fn main() {
    let input: Vec<String> = read_input_list().unwrap();

    let (pos, _) = input.iter().fold(((0, 0), (10, 1)), |(pos, dir), s| {
        let i = &s[..1];
        let v: isize = s[1..].parse().unwrap();

        match (i, v) {
            ("N", _) => (pos, (dir.0, dir.1 + v)),
            ("S", _) => (pos, (dir.0, dir.1 - v)),
            ("E", _) => (pos, (dir.0 + v, dir.1)),
            ("W", _) => (pos, (dir.0 - v, dir.1)),
            ("L", 90) | ("R", 270) => (pos, (-dir.1, dir.0)),
            ("R", 90) | ("L", 270) => (pos, (dir.1, -dir.0)),
            ("L", 180) | ("R", 180) => (pos, (-dir.0, -dir.1)),
            ("F", _) => ((pos.0 + v * dir.0, pos.1 + v * dir.1), dir),
            _ => panic!(),
        }
    });

    println!("Manhattan distance: {}", pos.0.abs() + pos.1.abs());
}