use crate::histogram::Histogram;
use crate::timed_block;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Display, Formatter};
use Dir::*;

#[cfg(test)]
mod test;

pub fn solve(input: &str) {
    let layout = timed_block("Part One", || initial_layout(input));
    println!("{}", layout.len());
    println!("{}", timed_block("Part Two", || part_two(&layout)));
}

type Layout = HashSet<Tile>;

fn initial_layout(input: &str) -> Layout {
    let mut black_tiles = HashSet::new();
    for path in input.lines().map(&parse_path) {
        let t = Tile::origin().walk(&path);
        if black_tiles.contains(&t) {
            black_tiles.remove(&t);
        } else {
            black_tiles.insert(t);
        }
    }
    black_tiles
}

fn part_two(layout: &Layout) -> usize {
    let mut lo = do_step(layout);
    for _ in 1..100 {
        lo = do_step(&lo);
    }
    lo.len()
}

fn do_step(layout: &Layout) -> Layout {
    let mut next = HashSet::new();
    let mut black_neighbor_hist = HashMap::new();
    for tile in layout {
        let mut count = 0;
        for neighbor in tile.neighbors() {
            if layout.contains(&neighbor) {
                count += 1;
            }
            black_neighbor_hist.increment_bucket(neighbor);
        }
        if count == 1 || count == 2 {
            next.insert(tile.clone());
        }
    }
    for (t, _) in black_neighbor_hist
        .iter()
        .filter(|(t, &nc)| nc == 2 && !layout.contains(t))
    {
        next.insert(t.clone());
    }
    next
}

fn parse_path(s: &str) -> Vec<Dir> {
    let mut chars = s.trim().chars();
    // this won't be exact, but it'll be pretty close
    let mut result = Vec::new();
    loop {
        match chars.next() {
            None => break,
            Some('n') => match chars.next() {
                None => break,
                Some('e') => result.push(NorthEast),
                Some('w') => result.push(NorthWest),
                Some(c) => panic!("Unrecognized '{}' after 'n' in path", c),
            },
            Some('s') => match chars.next() {
                None => break,
                Some('e') => result.push(SouthEast),
                Some('w') => result.push(SouthWest),
                Some(c) => panic!("Unrecognized '{}' after 's' in path", c),
            },
            Some('e') => result.push(East),
            Some('w') => result.push(West),
            Some(c) => panic!("Unrecognized '{}' in path", c),
        }
    }
    result
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Dir {
    NorthWest,
    NorthEast,
    East,
    SouthEast,
    SouthWest,
    West,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
struct Tile {
    x: i32,
    y: i32,
    z: i32,
}

impl Tile {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Tile { x, y, z }
    }

    fn origin() -> Self {
        Tile::new(0, 0, 0)
    }

    fn walk(&self, path: &[Dir]) -> Self {
        path.iter().fold(self.clone(), |t, d| t.step(*d))
    }

    fn step(&self, d: Dir) -> Self {
        self.step_by(d, 1)
    }

    fn step_by(&self, d: Dir, n: i32) -> Self {
        match d {
            NorthWest => Tile::new(self.x + n, self.y - n, self.z),
            NorthEast => Tile::new(self.x + n, self.y, self.z - n),
            East => Tile::new(self.x, self.y + n, self.z - n),
            SouthEast => Tile::new(self.x - n, self.y + n, self.z),
            SouthWest => Tile::new(self.x - n, self.y, self.z + n),
            West => Tile::new(self.x, self.y - n, self.z + n),
        }
    }

    fn neighbors(&self) -> Neighbors {
        Neighbors {
            tile: self,
            next: Some(NorthWest),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

struct Neighbors<'a> {
    tile: &'a Tile,
    next: Option<Dir>,
}

impl Iterator for Neighbors<'_> {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(d) = self.next {
            let t = self.tile.step(d);
            self.next = match d {
                NorthWest => Some(NorthEast),
                NorthEast => Some(East),
                East => Some(SouthEast),
                SouthEast => Some(SouthWest),
                SouthWest => Some(West),
                West => None,
            };
            Some(t)
        } else {
            None
        }
    }
}
