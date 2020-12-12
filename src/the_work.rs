use aoc_2020::geom2d::{Dir, Point};
use aoc_2020::read_lines;
use std::str::FromStr;

pub fn the_work() {
    let s = read_lines(|l| l.parse::<Action>().unwrap())
        .iter()
        .fold(Ship::new(), |s, a| s.perform(a));
    println!("{:?}", s);
    println!("{}", s.pos.manhattan_distance(&Point::origin()))
}

#[derive(Debug)]
struct Ship {
    pos: Point,
    way: Point,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            pos: Point::origin(),
            way: Point::origin()
                .step_by(Dir::East, 10)
                .step_by(Dir::North, 1),
        }
    }

    fn perform(&self, a: &Action) -> Ship {
        use Action::*;
        match a {
            North(n) => Ship {
                pos: self.pos,
                way: self.way.step_by(Dir::North, *n),
            },
            South(n) => Ship {
                pos: self.pos,
                way: self.way.step_by(Dir::South, *n),
            },
            East(n) => Ship {
                pos: self.pos,
                way: self.way.step_by(Dir::East, *n),
            },
            West(n) => Ship {
                pos: self.pos,
                way: self.way.step_by(Dir::West, *n),
            },
            Right(n) => Ship {
                pos: self.pos,
                way: match n / 90 {
                    0 => Point::new(self.way.x, self.way.y),
                    1 => Point::new(-self.way.y, self.way.x),
                    2 => Point::new(-self.way.x, -self.way.y),
                    3 => Point::new(self.way.y, -self.way.x),
                    _ => panic!("Unrecognized {} turn", n),
                },
            },
            Forward(n) => Ship {
                pos: self
                    .pos
                    .step_by(Dir::East, n * self.way.x)
                    .step_by(Dir::South, n * self.way.y),
                way: self.way,
            },
        }
    }
}

enum Action {
    North(isize),
    South(isize),
    East(isize),
    West(isize),
    Right(isize),
    Forward(isize),
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Action::*;
        let n = s[1..].parse::<isize>().unwrap();
        match &s[0..1] {
            "N" => Ok(North(n)),
            "S" => Ok(South(n)),
            "E" => Ok(East(n)),
            "W" => Ok(West(n)),
            "L" => Ok(Right(360 - n)),
            "R" => Ok(Right(n)),
            "F" => Ok(Forward(n)),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_ONE: &str = "
F10
N3
F7
R90
F11";

    #[test]
    fn example_one() {
        let s = EXAMPLE_ONE
            .trim()
            .lines()
            .map(|l| l.parse::<Action>().unwrap())
            .fold(Ship::new(), |s, a| s.perform(&a));
        assert_eq!(Point::new(214, 72), s.pos);
    }
}
