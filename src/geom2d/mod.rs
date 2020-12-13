/// Describes rectilinear directions on a coordinate plane.
#[derive(Copy, Clone, Debug)]
pub enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    pub fn clockwise(&self) -> Dir {
        use Dir::*;
        match self {
            North => East,
            South => West,
            East => South,
            West => North,
        }
    }
}

/// A location on a coordinate plane, laid out as a graphics editor would, not a mathematician. That
/// is, the `y` coordinate increases as you move `South`.
///
/// # Example
///
/// ```
/// use aoc_2020::geom2d::{Dir, Point};
///
/// let p1 = Point::origin();
/// assert_eq!(0, p1.x);
/// assert_eq!(0, p1.y);
/// let p2 = p1.step_by(Dir::East, 2).step_by(Dir::North, 1);
/// assert_eq!(2, p2.x);
/// assert_eq!(-1, p2.y);
/// assert_eq!(3, p1.manhattan_distance(&p2));
/// ```
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    pub fn origin() -> Point {
        Point::new(0, 0)
    }

    pub fn manhattan_distance(&self, p: &Point) -> usize {
        ((self.x - p.x).abs() + (self.y - p.y).abs()) as usize
    }

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::geom2d::Dir::*;

    #[test]
    fn test_manhattan_distance() {
        let origin = Point::origin();
        assert_eq!(25, Point::new(17, 8).manhattan_distance(&origin));
        assert_eq!(25, Point::new(17, -8).manhattan_distance(&origin));
        assert_eq!(25, Point::new(-17, 8).manhattan_distance(&origin));
        assert_eq!(25, Point::new(-17, -8).manhattan_distance(&origin));
    }

    #[test]
    fn test_step_by() {
        let origin = Point::origin();
        assert_eq!(Point::new(0, -1), origin.step_by(North, 1));
        assert_eq!(Point::new(0, 1), origin.step_by(South, 1));
        assert_eq!(Point::new(1, 0), origin.step_by(East, 1));
        assert_eq!(Point::new(-1, 0), origin.step_by(West, 1));
    }
}
