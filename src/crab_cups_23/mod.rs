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

struct Cups {
    moves: usize,
    size: usize,
    ring: VecDeque<usize>,
}

impl<'a> Cups {

    fn new(seed: &[usize]) -> Cups {
        Cups {
            moves: 0,
            size: seed.len(),
            ring: seed.iter().cloned().collect(),
        }
    }

    fn extend_to(&mut self, n: usize) {
        for i in self.size..n {
            self.ring.push_back(i);
        }
        self.size = self.ring.len();
    }

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

    fn candy_stripe_current(&self) -> Vec<usize> {
        self.partition_ring(self.size - (self.moves % self.size))
    }

    fn one_first(&self) -> Vec<usize> {
        let idx = self.ring.iter().position(|v| *v == 0).unwrap();
        self.partition_ring(idx)
    }

    fn partition_ring(&self, idx: usize) -> Vec<usize> {
        let mut result = self.ring.iter().skip(idx).map(|v| *v + 1).collect::<Vec<_>>();
        for v in self.ring.iter().take(idx) {
            result.push(*v + 1);
        }
        result
    }
}

impl From<&str> for Cups {
    fn from(s: &str) -> Self {
        let mut seed = VecDeque::with_capacity(s.len());
        let mut n = s.parse::<usize>().unwrap();
        while n > 0 {
            seed.push_front(n % 10 - 1);
            n /= 10;
        }
        let slices = seed.as_mut_slices();
        assert_eq!(0, slices.1.len());
        Cups::new(slices.0)
    }
}

impl Display for Cups {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let buffer = self.one_first();
        for d in buffer.iter().skip(1) {
            write!(f, "{}", d)?;
        }
        Ok(())
    }
}

impl Debug for Cups {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let buffer = self.candy_stripe_current();
        let curr = self.moves % self.size;
        for (i, n) in buffer.iter().enumerate() {
            if i == curr {
                write!(f, "({})", n)?;
            } else {
                write!(f, " {} ", n)?;
            }
        }
        Ok(())
    }
}
