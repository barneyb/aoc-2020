use crate::timed_block;
use std::collections::VecDeque;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[cfg(test)]
mod test;

pub fn solve(input: &str) {
    let cups = timed_block("Parsing", || Cups::from(input));
    println!("{:?}", timed_block("Part One", || part_one(&cups)));
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
