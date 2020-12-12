extern crate advent_of_code;

use advent_of_code::read_input_list;

fn main() {
    let input: Vec<String> = read_input_list().unwrap();

    let (pos, _) = input.iter().fold(((0, 0), (1, 0)), |(pos, dir), s| {
        let i = &s[..1];
        let v: isize = s[1..].parse().unwrap();

        match (i, v) {
            ("N", _) => ((pos.0, pos.1 + v), dir),
            ("S", _) => ((pos.0, pos.1 - v), dir),
            ("E", _) => ((pos.0 + v, pos.1), dir),
            ("W", _) => ((pos.0 - v, pos.1), dir),
            ("L", 90) | ("R", 270) => (pos, (-dir.1, dir.0)),
            ("R", 90) | ("L", 270) => (pos, (dir.1, -dir.0)),
            ("L", 180) | ("R", 180) => (pos, (-dir.0, -dir.1)),
            ("F", _) => ((pos.0 + v * dir.0, pos.1 + v * dir.1), dir),
            _ => panic!(),
        }
    });

    println!("Manhattan distance: {}", pos.0.abs() + pos.1.abs());
}