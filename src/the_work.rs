use aoc_2020::time_block;
use std::collections::VecDeque;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

pub fn the_work() {
    let cups = time_block("Parsing", || Cups::from("469217538"));
    println!("{:?}", time_block("Part One", || part_one(&cups)));
}

fn part_one(cups: &Cups) -> String {
    cups.to_string()
}

#[derive(Eq, PartialEq)]
struct Cups {
    moves: usize,
    ring: VecDeque<usize>,
}

impl Cups {
    fn tick(&mut self) {
        self.moves += 1;
        // todo: do some things!
    }
}

impl From<&str> for Cups {
    fn from(s: &str) -> Self {
        Cups::from(s.parse::<usize>().unwrap())
    }
}

/*
 * Parsing shift numbers down by one, so that we can use modulo arithmetic
 * easily. Rendering shifts numbers up by one so they rematch the problem.
 */

impl From<usize> for Cups {
    fn from(mut n: usize) -> Self {
        let mut cups = Cups {
            moves: 0,
            ring: VecDeque::new(),
        };
        while n > 0 {
            cups.ring.push_front(n % 10 - 1);
            n /= 10;
        }
        cups
    }
}

impl Display for Cups {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut buffer = self.ring.clone();
        loop {
            if let Some(0) = buffer.front() {
                break;
            }
            buffer.rotate_left(1);
        }
        for d in buffer.iter().skip(1) {
            write!(f, "{}", d + 1)?;
        }
        Ok(())
    }
}

impl Debug for Cups {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let curr = self.moves % self.ring.len();
        for (i, n) in self.ring.iter().enumerate() {
            if i == curr {
                write!(f, "({})", n + 1)?;
            } else {
                write!(f, " {} ", n + 1)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_and_unparse() {
        let cups = Cups::from("3412");
        assert_eq!(0, cups.moves);
        assert_eq!(VecDeque::from(vec![2usize, 3, 0, 1]), cups.ring);
        assert_eq!("(3) 4  1  2 ", format!("{:?}", cups));
        assert_eq!("234", cups.to_string());
    }

    const EXAMPLE_ONE: &str = "389125467";

    #[test]
    fn test_single_moves() {
        let mut cups = Cups::from(EXAMPLE_ONE);
        assert_eq!("(3) 8  9  1  2  5  4  6  7 ", format!("{:?}", cups));
        cups.tick();
        assert_eq!(" 3 (2) 8  9  1  5  4  6  7 ", format!("{:?}", cups));
        cups.tick();
        assert_eq!(" 3  2 (5) 4  6  7  8  9  1 ", format!("{:?}", cups));
        cups.tick();
        assert_eq!(" 7  2  5 (8) 9  1  3  4  6 ", format!("{:?}", cups));
        cups.tick();
        assert_eq!(" 3  2  5  8 (4) 6  7  9  1 ", format!("{:?}", cups));
        cups.tick();
        assert_eq!(" 9  2  5  8  4 (1) 3  6  7 ", format!("{:?}", cups));
        cups.tick();
        assert_eq!(" 7  2  5  8  4  1 (9) 3  6 ", format!("{:?}", cups));
        cups.tick();
        assert_eq!(" 8  3  6  7  4  1  9 (2) 5 ", format!("{:?}", cups));
        cups.tick();
        assert_eq!(" 7  4  1  5  8  3  9  2 (6)", format!("{:?}", cups));
        cups.tick();
        assert_eq!("(5) 7  4  1  8  3  9  2  6 ", format!("{:?}", cups));
        cups.tick();
        assert_eq!(" 5 (8) 3  7  4  1  9  2  6 ", format!("{:?}", cups));
        assert_eq!("92658374", cups.to_string())
    }

    #[test]
    fn example_one() {
        let ring = Cups::from(EXAMPLE_ONE);
        assert_eq!("67384529", part_one(&ring));
    }
}
