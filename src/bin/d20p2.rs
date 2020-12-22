extern crate regex;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

use std::str::FromStr;

use regex::Regex;

fn main() {
    let input = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();

    let tile_map = input
        .split("\n\n")
        .map(|s| Tile::from_str(s).unwrap())
        .map(|t| (t.id, RefCell::new(t)))
        .collect::<HashMap<_, _>>();

    let adjacency_map = tile_map
        .iter()
        .fold(HashMap::new(), |edge_map, (id, t)| {
            t.borrow().edges.as_ref().unwrap()
                .iter()
                .fold(edge_map, |mut edge_map, e| {
                    edge_map.entry(Edge::new(e.clone())).or_insert(HashSet::new()).insert(*id);
                    edge_map
                })
        })
        .values()
        .filter(|ids| ids.len() == 2)
        .map(|ids| ids.iter())
        .fold(HashMap::new(), |mut adjacency_map, mut ids| {
            let (a, b) = (*ids.next().unwrap(), *ids.next().unwrap());
            adjacency_map.entry(a).or_insert(HashSet::new()).insert(b);
            adjacency_map.entry(b).or_insert(HashSet::new()).insert(a);
            adjacency_map
        });

    let mut positional_map = HashMap::new();
    build_positional_map(
        *tile_map.keys().next().unwrap(),
        &mut positional_map,
        &adjacency_map,
        &tile_map,
    );

    let (min_extent, max_extent) = positional_map
        .values()
        .fold(((0, 0), (0, 0)), |(min, max), c| (
            (min.0.min(c.0), min.1.min(c.1)),
            (max.0.max(c.0), max.1.max(c.1)),
        ));

    let grid_size = (max_extent.0 - min_extent.0) as usize + 1;
    let tile_size = tile_map.values().next().unwrap().borrow().data.len();

    let mut map = vec![vec![None; grid_size]; grid_size];
    positional_map.iter().for_each(|(id, c)|
        map[(c.1 - min_extent.1) as usize][(c.0 - min_extent.0) as usize] = Some(&tile_map[id])
    );

    let mut combined = Tile::from_data(0,
        map
            .iter()
            .flat_map(|r| (1..tile_size - 1)
                .map(move |l| {
                    let row_segments = r
                    .iter()
                    .map(|t| t.unwrap().borrow().data[l][1..tile_size - 1].to_owned())
                    .collect::<Vec<_>>();
                    row_segments.iter().flat_map(|s| s.chars()).collect::<String>()
                })
            )
            .collect::<Vec<_>>(),
    );

    let sea_monster_body_1 = Regex::new(r"^#....##....##....###").unwrap();
    let sea_monster_body_2 = Regex::new(r"^.#..#..#..#..#..#").unwrap();

    for _ in 0..2 {
        for _ in 0..4 {
            let count = combined.data
                .iter()
                .take(combined.data.len() - 2)
                .enumerate()
                .flat_map(|(i, l)| l[18..]
                    .chars()
                    .enumerate()
                    .filter(move |(_, c)| *c == '#')
                    .map(move |(j, _)| (i, j))
                )
                .filter(|(i, j)|
                    sea_monster_body_1.is_match(&combined.data[i + 1][*j..]) &&
                    sea_monster_body_2.is_match(&combined.data[i + 2][*j..])
                )
                .count();

            if count > 0 {
                println!(
                    "Water roughness: {}",
                    combined.data
                        .iter()
                        .flat_map(|s| s.chars())
                        .filter(|c| *c == '#')
                        .count() - count * 15,
                );
            }
            combined.rotate_clockwise(1);
        }
        combined.flip_horizontally();
    }
}

fn build_positional_map(
    start: usize,
    positional_map: &mut HashMap<usize, (isize, isize)>,
    adjacency_map: &HashMap<usize, HashSet<usize>>,
    tile_map: &HashMap<usize, RefCell<Tile>>,
) {
    let coords = *positional_map.entry(start).or_insert((0, 0));

    for i in &adjacency_map[&start] {
        if !positional_map.contains_key(i) {
            let offset = tile_map[i].borrow_mut().reorient(&*tile_map[&start].borrow());
            positional_map.insert(*i, (coords.0 + offset.0, coords.1 + offset.1));
            build_positional_map(*i, positional_map, adjacency_map, tile_map);
        }
    }
}

pub struct Tile {
    pub id: usize,
    pub data: Vec<String>,
    pub edges: Option<[String; 4]>,
}

impl Tile {
    pub fn from_data(id: usize, data: Vec<String>) -> Self {
        let mut tile = Tile {
            id,
            data,
            edges: None,
        };
        tile.recalc_edges();
        tile
    }

    pub fn reorient(&mut self, other: &Self) -> (isize, isize) {
        let ((i, e), (j, f)) = self.edges.as_ref().unwrap()
            .iter()
            .enumerate()
            .flat_map(|(i, e)| other.edges.as_ref().unwrap()
                .iter()
                .enumerate()
                .map(move |(j, f)| ((i, e), (j, f)))
            )
            .find(|&((_, e), (_, f))| *e == *f || *e == f.chars().rev().collect::<String>())
            .unwrap();

        if *e == *f {
            match i % 2 {
                0 => self.flip_horizontally(),
                1 => self.flip_vertically(),
                _ => panic!("impossible"),
            }
        }

        self.rotate_clockwise((((j + 2 - i) % 4) + 4) % 4);
        match j {
            0 => (0, -1),
            1 => (1, 0),
            2 => (0, 1),
            3 => (-1, 0),
            _ => panic!("impossible"),
        }
    }

    pub fn flip_horizontally(&mut self) {
        self.data.iter_mut().for_each(|l| *l = l.chars().rev().collect());
        self.recalc_edges();
    }

    pub fn flip_vertically(&mut self) {
        self.data = self.data.iter().rev().cloned().collect();
        self.recalc_edges();
    }

    pub fn rotate_clockwise(&mut self, turns: usize) {
        match turns {
            0 => (),
            1 => {
                self.data = (0..self.data.len()).map(|i| self.data.iter().rev().map(|l| l.chars().nth(i).unwrap()).collect()).collect();
                self.edges.as_mut().unwrap().rotate_right(1);
            },
            2 => {
                self.flip_horizontally();
                self.flip_vertically();
            },
            3 => {
                self.data = (0..self.data.len()).rev().map(|i| self.data.iter().map(|l| l.chars().nth(i).unwrap()).collect()).collect();
                self.edges.as_mut().unwrap().rotate_left(1);
            },
            _ => panic!("impossible"),
        }
    }

    fn recalc_edges(&mut self) {
        self.edges = Some([
            self.data.first().unwrap().clone(),
            self.data.iter().map(|l| l.chars().last().unwrap()).collect(),
            self.data.last().unwrap().clone().chars().rev().collect(),
            self.data.iter().map(|l| l.chars().next().unwrap()).collect::<String>().chars().rev().collect(),
        ]);
    }
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id_match = Regex::new(r"Tile (\d+):").unwrap();

        Ok(
            Self::from_data(
                id_match.captures(s.lines().next().unwrap()).unwrap().get(1).unwrap().as_str().parse().unwrap(),
                s.lines().skip(1).map(|l| l.to_owned()).collect(),
            )
        )
    }
}

#[derive(Eq, PartialEq, Hash)]
pub struct Edge(String, String);

impl Edge {
    pub fn new(e: String) -> Self {
        let r = e.chars().rev().collect();
        if e < r {
            Self(e, r)
        } else {
            Self(r, e)
        }
    }
}