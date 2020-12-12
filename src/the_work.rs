use aoc_2020::read_input;
use std::fmt;
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;
use Loc::*;

pub fn the_work() {
    let map = load_map(&read_input());
    let stable = stabilize_map(&map);
    println!("{}", stable.occupied_seat_count());
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
        let x = i % self.width;
        let y = i / self.width;
        let mut pairs = Vec::new();
        // the column to the left
        if x > 0 {
            if y > 0 {
                pairs.push((x - 1, y - 1));
            }
            pairs.push((x - 1, y));
            if y < self.height - 1 {
                pairs.push((x - 1, y + 1));
            }
        }
        // this column
        if y > 0 {
            pairs.push((x, y - 1));
        }
        if y < self.height - 1 {
            pairs.push((x, y + 1));
        }
        // the column to the right
        if x < self.width - 1 {
            if y > 0 {
                pairs.push((x + 1, y - 1));
            }
            pairs.push((x + 1, y));
            if y < self.height - 1 {
                pairs.push((x + 1, y + 1));
            }
        }
        pairs
            .iter()
            .filter(|(x, y)| {
                debug_assert!(y * self.width + x < self.width * self.height);
                self.locations[y * self.width + x] == Occupied
            })
            .count()
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
                    if self.occupied_neighbor_count(i) >= 4 {
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

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_ONE: &str = "
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_load_map() {
        let m = load_map(EXAMPLE_ONE);
        assert_eq!(10, m.width);
        assert_eq!(10, m.height);
        assert_eq!(71, m.empty_seat_count());
        assert_eq!(0, m.occupied_seat_count());
    }

    #[test]
    fn test_display() {
        assert_eq!(EXAMPLE_ONE.trim(), load_map(EXAMPLE_ONE).to_string())
    }

    #[test]
    fn test_step() {
        let m = load_map(EXAMPLE_ONE);
        assert_eq!(0, m.occupied_seat_count());
        let m = m.step();
        assert_eq!(
            "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##",
            m.to_string()
        );
        assert_eq!(71, m.occupied_seat_count());
        let m = m.step();
        assert_eq!(
            "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##",
            m.to_string()
        );
        assert_eq!(20, m.occupied_seat_count());
        let m = m.step();
        assert_eq!(
            "#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##",
            m.to_string()
        );
        assert_eq!(51, m.occupied_seat_count());
    }

    #[test]
    fn test_stabilize() {
        let m = load_map(EXAMPLE_ONE);
        let m = stabilize_map(&m);
        assert_eq!(37, m.occupied_seat_count());
    }
}
