use crate::timed_block;
use std::fmt;
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;
use Loc::*;

#[cfg(test)]
mod test;

pub fn solve(input: &str) {
    let map = load_map(&input);
    let stable = stabilize_map(&map);
    println!(
        "{}",
        timed_block("Part Two", || stable.occupied_seat_count())
    );
}

fn load_map(s: &str) -> Map {
    let lines: Vec<&str> = s.trim().split('\n').collect();
    let width = lines[0].len();
    let height = lines.len();
    let mut locations = Vec::with_capacity(width * height);
    for l in lines {
        for c in l.chars() {
            locations.push(c.to_string().parse().unwrap());
        }
    }
    Map::new(width, height, locations)
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Loc {
    Floor,
    Empty,
    Occupied,
}

impl FromStr for Loc {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Floor),
            "L" => Ok(Empty),
            "#" => Ok(Occupied),
            _ => Err(format!("Unrecognized '{}' in map", s)),
        }
    }
}

impl Display for Loc {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_char(match self {
            Floor => '.',
            Empty => 'L',
            Occupied => '#',
        })
    }
}

#[derive(Eq, PartialEq)]
struct Map {
    width: usize,
    height: usize,
    locations: Vec<Loc>,
}

impl Map {
    fn new(width: usize, height: usize, locations: Vec<Loc>) -> Map {
        Map {
            width,
            height,
            locations,
        }
    }

    fn occupied_neighbor_count(&self, i: usize) -> usize {
        let from_x = (i % self.width) as isize;
        let from_y = (i / self.width) as isize;
        let max_x = self.width as isize - 1;
        let max_y = self.height as isize - 1;
        let find_seat = |dx, dy| {
            let mut x = from_x;
            let mut y = from_y;
            loop {
                x += dx;
                y += dy;
                if x < 0 || x > max_x || y < 0 || y > max_y {
                    return None;
                }
                match self.at(x, y) {
                    Floor => {}
                    Empty => return Some(Empty),
                    Occupied => return Some(Occupied),
                }
            }
        };
        [
            find_seat(-1, -1),
            find_seat(-1, 0),
            find_seat(-1, 1),
            find_seat(0, -1),
            // not "this"
            find_seat(0, 1),
            find_seat(1, -1),
            find_seat(1, 0),
            find_seat(1, 1),
        ]
        .iter()
        .map(|it| match it {
            Some(Occupied) => 1,
            _ => 0,
        })
        .sum()
    }

    fn at(&self, x: isize, y: isize) -> &Loc {
        &self.locations[y as usize * self.width + x as usize]
    }

    fn step(&self) -> Map {
        let locations = self
            .locations
            .iter()
            .enumerate()
            .map(|(i, l)| match l {
                Floor => Floor,
                Empty => match self.occupied_neighbor_count(i) {
                    0 => Occupied,
                    _ => Empty,
                },
                Occupied => {
                    if self.occupied_neighbor_count(i) >= 5 {
                        Empty
                    } else {
                        Occupied
                    }
                }
            })
            .collect::<Vec<Loc>>();
        debug_assert_eq!(locations.len(), self.locations.len());
        Map {
            width: self.width,
            height: self.height,
            locations,
        }
    }

    #[cfg(test)]
    fn empty_seat_count(&self) -> usize {
        self.locations.iter().filter(|&&it| it == Empty).count()
    }

    fn occupied_seat_count(&self) -> usize {
        self.locations.iter().filter(|&&it| it == Occupied).count()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = String::with_capacity(self.width * (self.height + 1));
        for y in 0..self.height {
            if y > 0 {
                s.push('\n');
            }
            for x in 0..self.width {
                s.push_str(&self.locations[y * self.width + x].to_string());
            }
        }
        write!(f, "{}", s)
    }
}

fn stabilize_map(m: &Map) -> Map {
    let mut curr = m.step();
    loop {
        let next = curr.step();
        if curr == next {
            return next;
        }
        curr = next;
    }
}
