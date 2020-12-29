use aoc_2020::geom2d::Dir;
use aoc_2020::{read_lines, vector_type};
use std::str::FromStr;

pub fn the_work() {
    let s = read_lines(|l| l.parse::<Action>().unwrap())
        .iter()
        .fold(Ship::new(), |s, a| s.perform(a));
    println!("{:?}", s);
    println!("{}", s.pos.manhattan_distance(&Point::origin()))
}

vector_type!(Point, isize, x, y);

impl Point {
    pub fn step_by(&self, d: Dir, steps: isize) -> Point {
        use Dir::*;
        match d {
            North => Point {
                x: self.x,
                y: self.y - steps,
            },
            South => Point {
                x: self.x,
                y: self.y + steps,
            },
            East => Point {
                x: self.x + steps,
                y: self.y,
            },
            West => Point {
                x: self.x - steps,
                y: self.y,
            },
        }
    }
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
            Move(d, n) => Ship {
                pos: self.pos,
                way: self.way.step_by(*d, *n),
            },
            Rotate(n) => Ship {
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
    Move(Dir, isize),
    Rotate(isize),
    Forward(isize),
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Action::*;
        let n = s[1..].parse::<isize>().unwrap();
        match &s[0..1] {
            "N" => Ok(Move(Dir::North, n)),
            "S" => Ok(Move(Dir::South, n)),
            "E" => Ok(Move(Dir::East, n)),
            "W" => Ok(Move(Dir::West, n)),
            "L" => Ok(Rotate(360 - n % 360)),
            "R" => Ok(Rotate(n % 360)),
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

    #[test]
    fn test_step_by() {
        let origin = Point::origin();
        assert_eq!(Point::new(0, -1), origin.step_by(Dir::North, 1));
        assert_eq!(Point::new(0, 1), origin.step_by(Dir::South, 1));
        assert_eq!(Point::new(1, 0), origin.step_by(Dir::East, 1));
        assert_eq!(Point::new(-1, 0), origin.step_by(Dir::West, 1));
    }
}
