use std::collections::HashSet;
use std::env;
use std::fs;
use std::hash::Hash;

fn main() {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut grid = Grid::new(
        input
            .lines()
            .flat_map(|l| l.chars())
            .map(|c| c == '#'),
        (0, 0, 0, 0),
        (width as isize - 1, height as isize - 1, 0, 0),
    );

    (0..6).for_each(|_| advance(&mut grid));

    println!("Cells active: {}", grid.population());
}

type Coord = (isize, isize, isize, isize);

fn advance(grid: &mut Grid<Coord>) {
    let (low, high) = {
        let (low, high) = grid.get_extents();
        grid.ensure_coordinates(&(low.0 - 1, low.1 - 1, low.2 - 1, low.3 - 1));
        grid.ensure_coordinates(&(high.0 + 1, high.1 + 1, high.2 + 1, high.3 + 1));
        grid.get_extents()
    };

    let volume = Coord::volume(&low, &high);

    let neighbors = (0..volume)
        .map(|i| Coord::from_index(i, &low, &high))
        .map(|c| (-1..=1)
            .flat_map(move |ow| (-1..=1)
                .flat_map(move |oz| (-1..=1)
                    .flat_map(move |oy| (-1..=1)
                        .map(move |ox| (ox, oy, oz, ow))
                    )
                )
            )
            .filter(|o| o.0 != 0 || o.1 != 0 || o.2 != 0 || o.3 != 0)
            .filter(|o| grid.get(&(c.0 + o.0, c.1 + o.1, c.2 + o.2, c.3 + o.3)))
            .count()
        )
        .collect::<Vec<_>>();

    for i in 0..volume {
        let n = neighbors[i];
        let c = Coord::from_index(i, &low, &high);
        let active = grid.get(&c);

        if active && !(n == 2 || n == 3) {
            grid.clear(&c);
        } else if !active && n == 3 {
            grid.set(&c);
        }
    }
}

#[derive(Debug)]
struct Grid<T: Eq + Hash> {
    data: HashSet<T>,
    min: T,
    max: T,
}

impl<T: Clone + Coordinate + Eq + Hash> Grid<T> {
    fn new<I: IntoIterator<Item=bool>>(data: I, min: T, max: T) -> Self {
        Self {
            data: data
                .into_iter()
                .enumerate()
                .filter(|(_, x)| *x)
                .map(|(i, _)| T::from_index(i, &min, &max))
                .collect(),
            min,
            max,
        }
    }

    fn get(&self, c: &T) -> bool {
        self.data.contains(c)
    }

    fn set(&mut self, c: &T) {
        self.ensure_coordinates(c);
        self.data.insert(c.clone());
    }

    fn clear(&mut self, c: &T) {
        self.data.remove(c);
    }

    fn get_extents(&self) -> (T, T) {
        (self.min.clone(), self.max.clone())
    }

    fn ensure_coordinates(&mut self, c: &T) {
        self.min = self.min.min_extent(c);
        self.max = self.max.max_extent(c);
    }

    fn population(&self) -> usize {
        self.data.len()
    }
}

trait Coordinate {
    fn min_extent(&self, other: &Self) -> Self;
    fn max_extent(&self, other: &Self) -> Self;
    fn volume(min: &Self, max: &Self) -> usize;
    fn from_index(i: usize, min: &Self, max: &Self) -> Self;
}

impl Coordinate for Coord {
    fn min_extent(&self, other: &Self) -> Self {
        (self.0.min(other.0), self.1.min(other.1), self.2.min(other.2), self.3.min(other.3))
    }

    fn max_extent(&self, other: &Self) -> Self {
        (self.0.max(other.0), self.1.max(other.1), self.2.max(other.2), self.3.max(other.3))
    }

    fn volume(min: &Self, max: &Self) -> usize {
        let d = (max.0 - min.0 + 1, max.1 - min.1 + 1, max.2 - min.2 + 1, max.3 - min.3 + 1);
        (d.0 * d.1 * d.2 * d.3) as usize
    }

    fn from_index(i: usize, min: &Self, max: &Self) -> Self {
        let d = (max.0 - min.0 + 1, max.1 - min.1 + 1, max.2 - min.2 + 1, max.3 - min.3 + 1);
        (
            i as isize % d.0 + min.0,
            (i as isize / d.0) % d.1 + min.1,
            (i as isize / d.0 / d.1) % d.2 + min.2,
            (i as isize / d.0 / d.1 / d.2) % d.3 + min.3,
        )
    }
}
