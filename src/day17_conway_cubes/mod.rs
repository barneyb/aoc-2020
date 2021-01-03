use crate::histogram::Histogram;
use crate::{timed_block, vector_type};
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Display, Formatter, Write};
use std::ops::Add;

#[cfg(test)]
mod test;

pub fn solve(input: &str) {
    println!("{}", timed_block("Part Two", || run_simulation(input)));
}

fn run_simulation(input: &str) -> usize {
    let g = (0..6).fold(Game::new(input), |g, _| g.cycle());
    g.get_active_cell_count()
}

struct Game {
    active: HashSet<Point>,
    cycle_count: usize,
}

impl Game {
    fn new(input: &str) -> Game {
        let mut active = HashSet::new();
        for (y, l) in input.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                if c == '#' {
                    active.insert(Point::new(x as isize, y as isize, 0, 0));
                }
            }
        }
        Game {
            active,
            cycle_count: 0,
        }
    }

    fn cycle(&self) -> Game {
        let mut active = HashSet::new();
        let mut neighbor_hist = HashMap::new();
        for p in &self.active {
            let mut count = 0;
            for n in p.neighbors() {
                if self.active.contains(&n) {
                    count += 1;
                }
                neighbor_hist.increment_bucket(n);
            }
            if count == 2 || count == 3 {
                active.insert(p.clone());
            }
        }
        for (p, nc) in neighbor_hist {
            if nc == 3 && !self.active.contains(&p) {
                active.insert(p.clone());
            }
        }
        Game {
            active,
            cycle_count: self.cycle_count + 1,
        }
    }

    fn get_active_cell_count(&self) -> usize {
        self.active.len()
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (min, max) = self
            .active
            .iter()
            .fold((Point::origin(), Point::origin()), |(min, max), p| {
                (min.rectilinear_min(p), max.rectilinear_max(p))
            });
        for w in min.w..=max.w {
            for z in min.z..=max.z {
                writeln!(f, "z={}, w={}", z, w)?;
                for y in min.y..=max.y {
                    for x in min.x..=max.x {
                        if self.active.contains(&Point::new(x, y, z, w)) {
                            f.write_char('#')?;
                        } else {
                            f.write_char('.')?;
                        }
                    }
                    f.write_char('\n')?;
                }
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

vector_type![Point, isize, x, y, z, w];
type Offset = [isize; 4];

impl Point {
    pub fn neighbors(&self) -> Neighbors {
        Neighbors { p: self, i: 0 }
    }
}

impl Add<&Offset> for &Point {
    type Output = Point;

    fn add(self, rhs: &Offset) -> Self::Output {
        Point::new(
            self.x + rhs[0],
            self.y + rhs[1],
            self.z + rhs[2],
            self.w + rhs[3],
        )
    }
}

const NEIGHBOR_OFFSETS: [Offset; 80] = [
    [-1, -1, -1, -1],
    [-1, -1, -1, 0],
    [-1, -1, -1, 1],
    [-1, -1, 0, -1],
    [-1, -1, 0, 0],
    [-1, -1, 0, 1],
    [-1, -1, 1, -1],
    [-1, -1, 1, 0],
    [-1, -1, 1, 1],
    [-1, 0, -1, -1],
    [-1, 0, -1, 0],
    [-1, 0, -1, 1],
    [-1, 0, 0, -1],
    [-1, 0, 0, 0],
    [-1, 0, 0, 1],
    [-1, 0, 1, -1],
    [-1, 0, 1, 0],
    [-1, 0, 1, 1],
    [-1, 1, -1, -1],
    [-1, 1, -1, 0],
    [-1, 1, -1, 1],
    [-1, 1, 0, -1],
    [-1, 1, 0, 0],
    [-1, 1, 0, 1],
    [-1, 1, 1, -1],
    [-1, 1, 1, 0],
    [-1, 1, 1, 1],
    [0, -1, -1, -1],
    [0, -1, -1, 0],
    [0, -1, -1, 1],
    [0, -1, 0, -1],
    [0, -1, 0, 0],
    [0, -1, 0, 1],
    [0, -1, 1, -1],
    [0, -1, 1, 0],
    [0, -1, 1, 1],
    [0, 0, -1, -1],
    [0, 0, -1, 0],
    [0, 0, -1, 1],
    [0, 0, 0, -1],
    // [0, 0, 0, 0],
    [0, 0, 0, 1],
    [0, 0, 1, -1],
    [0, 0, 1, 0],
    [0, 0, 1, 1],
    [0, 1, -1, -1],
    [0, 1, -1, 0],
    [0, 1, -1, 1],
    [0, 1, 0, -1],
    [0, 1, 0, 0],
    [0, 1, 0, 1],
    [0, 1, 1, -1],
    [0, 1, 1, 0],
    [0, 1, 1, 1],
    [1, -1, -1, -1],
    [1, -1, -1, 0],
    [1, -1, -1, 1],
    [1, -1, 0, -1],
    [1, -1, 0, 0],
    [1, -1, 0, 1],
    [1, -1, 1, -1],
    [1, -1, 1, 0],
    [1, -1, 1, 1],
    [1, 0, -1, -1],
    [1, 0, -1, 0],
    [1, 0, -1, 1],
    [1, 0, 0, -1],
    [1, 0, 0, 0],
    [1, 0, 0, 1],
    [1, 0, 1, -1],
    [1, 0, 1, 0],
    [1, 0, 1, 1],
    [1, 1, -1, -1],
    [1, 1, -1, 0],
    [1, 1, -1, 1],
    [1, 1, 0, -1],
    [1, 1, 0, 0],
    [1, 1, 0, 1],
    [1, 1, 1, -1],
    [1, 1, 1, 0],
    [1, 1, 1, 1],
];

struct Neighbors<'a> {
    p: &'a Point,
    i: usize,
}

impl Iterator for Neighbors<'_> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= NEIGHBOR_OFFSETS.len() {
            return None;
        }
        let offset = &NEIGHBOR_OFFSETS[self.i];
        self.i += 1;
        Some(self.p + offset)
    }
}
