use std::str::FromStr;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

fn read_list<T, P>(path: P) -> Result<Vec<T>>
where
    T: FromStr,
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(
        BufReader::new(file)
            .lines()
            .filter_map(|l| l.map(|s| T::from_str(&s).ok()).unwrap_or(None))
            .collect()
    )
}

pub fn read_input_list<T>() -> Result<Vec<T>>
where
    T: FromStr,
{
    read_list(env::args().nth(1).unwrap())
}