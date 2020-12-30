use crate::timed_block;
use std::collections::VecDeque;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[cfg(test)]
mod test;

pub fn solve(input: &str) {
    let mut cups = timed_block("Parsing", || Cups::from(input));
    println!("{}", timed_block("Part One", || part_one(&mut cups)));
}

fn part_one(cups: &mut Cups) -> String {
    while cups.moves < 100 {
        cups.tick()
    }
    cups.to_string()
}

#[derive(Eq, PartialEq)]
struct Cups {
    moves: usize,
    size: usize,
    ring: VecDeque<usize>,
}

impl Cups {
    fn tick(&mut self) {
        // The crab picks up the three cups that are immediately clockwise of the current cup. They
        // are removed from the circle; cup spacing is adjusted as necessary to maintain the circle.
        let mut queue = Vec::with_capacity(3);
        self.ring.rotate_left(1);
        queue.push(self.ring.remove(0).unwrap());
        queue.push(self.ring.remove(0).unwrap());
        queue.push(self.ring.remove(0).unwrap());
        self.ring.rotate_right(1);

        // The crab selects a destination cup: the cup with a label equal to the current cup's label
        // minus one. If this would select one of the cups that was just picked up, the crab will
        // keep subtracting one until it finds a cup that wasn't just picked up. If at any point in
        // this process the value goes below the lowest value on any cup's label, it wraps around to
        // the highest value on any cup's label instead.
        let mut dest_num = *self.ring.front().unwrap();
        loop {
            dest_num = (dest_num + self.size - 1) % self.size;
            if !queue.contains(&dest_num) {
                break;
            }
        }

        // The crab places the cups it just picked up so that they are immediately clockwise of the
        // destination cup. They keep the same order as when they were picked up.
        let dest_idx = self.ring.iter().position(|n| *n == dest_num).unwrap();
        self.ring.rotate_left(dest_idx + 1);
        self.ring.insert(0, queue.pop().unwrap());
        self.ring.insert(0, queue.pop().unwrap());
        self.ring.insert(0, queue.pop().unwrap());

        // The crab selects a new current cup: the cup which is immediately clockwise of the current
        // cup.
        self.ring.rotate_right(dest_idx);
        self.moves += 1;
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
            size: 0,
            ring: VecDeque::new(),
        };
        while n > 0 {
            cups.ring.push_front(n % 10 - 1);
            n /= 10;
        }
        cups.ring.shrink_to_fit();
        cups.size = cups.ring.len();
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
        let mut buffer = self.ring.clone();
        let curr = self.moves % self.size;
        buffer.rotate_right(curr);
        for (i, n) in buffer.iter().enumerate() {
            if i == curr {
                write!(f, "({})", n + 1)?;
            } else {
                write!(f, " {} ", n + 1)?;
            }
        }
        Ok(())
    }
}
