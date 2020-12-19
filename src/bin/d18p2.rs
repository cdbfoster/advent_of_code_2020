extern crate advent_of_code;

use advent_of_code::read_input_list;

fn main() {
    let mut input: Vec<String> = read_input_list().unwrap();

    // Doctor the expressions to give addition precedence
    for l in input.iter_mut() {
        let mut r = 0..l.len();

        while let Some(i) = l[r.clone()].find('+') {
            // Insert left parenthesis
            let mut p = 0;
            for j in (0..r.start + i).rev() {
                let c = l.chars().nth(j).unwrap();
                match c {
                    ')' => p += 1,
                    '(' => p -= 1,
                    _ => (),
                }
                match c {
                    ' ' | ')' => (),
                    _ => if p == 0 {
                        l.insert(j, '(');
                        break;
                    }
                }
            }

            // Insert right parenthesis
            let mut p = 0;
            for j in r.start + i + 2..r.end {
                let c = l.chars().nth(j).unwrap();
                match c {
                    '(' => p += 1,
                    ')' => p -= 1,
                    _ => (),
                }
                match c {
                    ' ' | '(' => (),
                    _ => if p == 0 {
                        l.insert(j + 1, ')');
                        break;
                    }
                }
            }

            r = r.start + i + 2..l.len();
        }
    }

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