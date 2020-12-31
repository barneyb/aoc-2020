use crate::timed_block;
use std::collections::HashSet;

#[cfg(test)]
mod test;

pub fn solve(input: &str) {
    println!("{}", timed_block("Part One", || part_one(input)));
}

fn part_one(input: &str) -> usize {
    let mut black_tiles = HashSet::new();
    for path in input.lines() {
        let t = Tile::origin().walk(&parse_path(&path));
        if black_tiles.contains(&t) {
            black_tiles.remove(&t);
        } else {
            black_tiles.insert(t);
        }
    }
    black_tiles.len()
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
use Dir::*;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
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
            NorthEast => Tile::new(self.x + 1, self.y, self.z - 1),
            East => Tile::new(self.x, self.y + 1, self.z - 1),
            SouthEast => Tile::new(self.x - n, self.y + n, self.z),
            SouthWest => Tile::new(self.x - n, self.y, self.z + n),
            West => Tile::new(self.x, self.y - n, self.z + n),
        }
    }
}
