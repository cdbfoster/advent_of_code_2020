use std::env;
use std::fs;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let mut grid = Grid::from_str(&input).unwrap();

    while advance(&mut grid) {}

    println!("Occupied seats: {}", grid.data.iter().filter(|&c| *c == '#').count());
}

fn occupied_neighbors(grid: &Grid<char>, x: usize, y: usize) -> usize {
    (-1..=1)
        .flat_map(|y| (-1..=1).map(move |x| (x, y)))
        .filter(|(ox, oy)| !(*ox == 0 && *oy == 0))
        .map(|(ox, oy)| (x as isize + ox, y as isize + oy))
        .filter(|(px, py)| grid.in_bounds(*px, *py))
        .filter(|(px, py)| *grid.get(*px as usize, *py as usize) == '#')
        .count()
}

fn advance(grid: &mut Grid<char>) -> bool {
    let occupied = (0..grid.height)
        .flat_map(|y| (0..grid.width).map(move |x| (x, y)))
        .map(|(x, y)| occupied_neighbors(grid, x, y));

    let next = grid.data
        .iter()
        .zip(occupied)
        .map(|(c, o)| {
            if *c == 'L' && o == 0 {
                '#'
            } else if *c == '#' && o >= 4 {
                'L'
            } else {
                *c
            }
        }).collect();

    let change = grid.data != next;
    grid.data = next;
    change
}

struct Grid<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    fn get(&self, x: usize, y: usize) -> &T {
        &self.data[y * self.width + x]
    }

    fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize
    }
}

impl FromStr for Grid<char> {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s.lines().flat_map(|l| l.chars()).collect();

        Ok(
            Self {
                data,
                width: s.lines().next().unwrap().len(),
                height: s.lines().count(),
            }
        )
    }
}
