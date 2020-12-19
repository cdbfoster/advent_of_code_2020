extern crate advent_of_code;

use advent_of_code::read_input_list;

fn main() {
    let input: Vec<String> = read_input_list().unwrap();

    println!("Sum of expressions: {}", input.iter().map(|l| eval(&mut l.chars())).sum::<usize>());
}

fn eval<I: Iterator<Item = char>>(expr: &mut I) -> usize {
    let mut value = 0;
    let mut op = '+';

    fn apply_op(a: usize, b: usize, op: char) -> usize {
        match op {
            '+' => a + b,
            '*' => a * b,
            _ => panic!(),
        }
    }

    while let Some(c) = expr.next() {
        match c {
            '(' => value = apply_op(value, eval(expr), op),
            ')' => return value,
            '+' | '*' => op = c,
            ' ' => (),
            _ => value = apply_op(value, c.to_digit(10).unwrap() as usize, op)
        }
    }

    value
}