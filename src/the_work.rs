use aoc_2020::read_input;
use std::collections::HashSet;
use std::fmt::{self, Display, Formatter, Write};
use std::ops::Add;

pub fn the_work() {
    let s = read_input();
    println!("{:?}", part_one(&s));
}

fn part_one(input: &str) -> usize {
    let g = (0..6).fold(Game::new(input), |g, _| g.cycle());
    g.get_active_cell_count()
}

struct Game {
    active: HashSet<Point>,
    min: Point,
    max: Point,
    cycle_count: usize,
}

impl Game {
    fn new(input: &str) -> Game {
        let mut active = HashSet::new();
        for (y, l) in input.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                if c == '#' {
                    active.insert(Point::new(x as isize, y as isize, 0));
                }
            }
        }
        Game {
            active,
            min: Point::origin(),
            max: Point::new(
                input.lines().next().unwrap().len() as isize - 1,
                input.lines().count() as isize - 1,
                0,
            ),
            cycle_count: 0,
        }
    }

    fn cycle(&self) -> Game {
        let mut active = HashSet::new();
        let mut min = Point::origin();
        let mut max = Point::origin();
        for z in (self.min.z - 1)..=(self.max.z + 1) {
            for y in (self.min.y - 1)..=(self.max.y + 1) {
                for x in (self.min.x - 1)..=(self.max.x + 1) {
                    let p = Point::new(x, y, z);
                    let active_neighbor_count =
                        p.neighbors().filter(|p| self.active.contains(p)).count();
                    if self.active.contains(&p) {
                        if active_neighbor_count == 2 || active_neighbor_count == 3 {
                            active.insert(p);
                            min = min.rectilinear_min(&p);
                            max = max.rectilinear_max(&p);
                        }
                    } else {
                        if active_neighbor_count == 3 {
                            active.insert(p);
                            min = min.rectilinear_min(&p);
                            max = max.rectilinear_max(&p);
                        }
                    }
                }
            }
        }
        Game {
            active,
            min,
            max,
            cycle_count: self.cycle_count + 1,
        }
    }

    fn get_active_cell_count(&self) -> usize {
        self.active.len()
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for z in self.min.z..=self.max.z {
            writeln!(f, "z={}", z)?;
            for y in self.min.y..=self.max.y {
                for x in self.min.x..=self.max.x {
                    if self.active.contains(&Point::new(x, y, z)) {
                        f.write_char('#')?;
                    } else {
                        f.write_char('.')?;
                    }
                }
                f.write_char('\n')?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Point {
    pub const fn new(x: isize, y: isize, z: isize) -> Point {
        Point { x, y, z }
    }

    #[allow(dead_code)]
    pub const fn origin() -> Point {
        Point::new(0, 0, 0)
    }

    pub fn neighbors(&self) -> Neighbors {
        Neighbors { p: self, i: 0 }
    }

    pub fn rectilinear_min(&self, other: &Point) -> Point {
        Point {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }

    pub fn rectilinear_max(&self, other: &Point) -> Point {
        Point {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

const NEIGHBOR_OFFSETS: [Point; 26] = [
    Point::new(-1, -1, -1),
    Point::new(-1, -1, 0),
    Point::new(-1, -1, 1),
    Point::new(-1, 0, -1),
    Point::new(-1, 0, 0),
    Point::new(-1, 0, 1),
    Point::new(-1, 1, -1),
    Point::new(-1, 1, 0),
    Point::new(-1, 1, 1),
    Point::new(0, -1, -1),
    Point::new(0, -1, 0),
    Point::new(0, -1, 1),
    Point::new(0, 0, -1),
    // Point::new(0, 0, 0),
    Point::new(0, 0, 1),
    Point::new(0, 1, -1),
    Point::new(0, 1, 0),
    Point::new(0, 1, 1),
    Point::new(1, -1, -1),
    Point::new(1, -1, 0),
    Point::new(1, -1, 1),
    Point::new(1, 0, -1),
    Point::new(1, 0, 0),
    Point::new(1, 0, 1),
    Point::new(1, 1, -1),
    Point::new(1, 1, 0),
    Point::new(1, 1, 1),
];

pub struct Neighbors<'a> {
    p: &'a Point,
    i: usize,
}

impl Iterator for Neighbors<'_> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= NEIGHBOR_OFFSETS.len() {
            return None;
        }
        let d = NEIGHBOR_OFFSETS[self.i];
        self.i += 1;
        Some(*self.p + d)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_ONE: &str = ".#.
..#
###";

    #[test]
    fn example_one() {
        let s = EXAMPLE_ONE.trim();
        assert_eq!(112, part_one(s));
    }

    #[test]
    fn example_one_cycle_by_cycle() {
        let g = Game::new(EXAMPLE_ONE.trim());
        println!("Before any cycles:\n\n{}", g);
        assert_eq!(0, g.cycle_count);
        assert_eq!(5, g.get_active_cell_count());
        let g = g.cycle();
        println!("After 1 cycle:\n\n{}", g);
        assert_eq!(1, g.cycle_count);
        assert_eq!(11, g.get_active_cell_count());
        let g = g.cycle();
        println!("After 2 cycles:\n\n{}", g);
        assert_eq!(2, g.cycle_count);
        assert_eq!(21, g.get_active_cell_count());
        let g = g.cycle();
        println!("After 3 cycles:\n\n{}", g);
        assert_eq!(3, g.cycle_count);
        assert_eq!(38, g.get_active_cell_count());
        let g = g.cycle();
        let g = g.cycle();
        let g = g.cycle();
        assert_eq!(6, g.cycle_count);
        assert_eq!(112, g.get_active_cell_count());
    }
}
