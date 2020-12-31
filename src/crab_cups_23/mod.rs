use crate::timed_block;
use std::collections::VecDeque;
use std::fmt;
use std::fmt::{Display, Formatter};

#[cfg(test)]
mod test;

pub fn solve(input: &str) {
    println!("{}", timed_block("Part One", || part_one(input)));
    println!("{}", timed_block("Part Two", || part_two(input)));
}

fn part_one(input: &str) -> String {
    let mut cups = Cups::from(input);
    while cups.moves < 100 {
        cups.tick()
    }
    cups.to_string()
}

fn part_two(input: &str) -> usize {
    let mut cups = Cups::from(input);
    cups.extend_to(1_000_000);
    while cups.moves < 10_000_000 {
        cups.tick()
    }
    let (a, b) = cups.pair_after_one();
    a * b
}

struct Cups {
    head: usize,
    tail: usize,
    curr: usize,
    moves: usize,
    size: usize,
    arr: Vec<usize>,
}

impl<'a> Cups {
    fn new(seed: &[usize]) -> Cups {
        let mut arr = Vec::new();
        arr.resize(seed.len() + 1, 0);
        for (i, &n) in seed.iter().enumerate() {
            arr[n] = seed[(i + 1) % seed.len()];
        }
        Cups {
            head: seed[0],
            tail: seed[seed.len() - 1],
            curr: seed[0],
            moves: 0,
            size: seed.len(), // one less than arr.len()!
            arr,
        }
    }

    fn extend_to(&mut self, n: usize) {
        debug_assert!(n > self.size, "Can't extend to a smaller size");
        self.arr.resize(n + 1, 0);
        let mut curr = self.tail;
        assert_eq!(self.head, self.arr[curr]);
        for i in (self.size + 1)..(n + 1) {
            self.arr[curr] = i;
            curr = i;
        }
        self.arr[curr] = self.head;
        self.size = n;
    }

    fn tick(&mut self) {
        let r = self.arr[self.curr];
        let r2 = self.arr[r];
        let r3 = self.arr[r2];
        let mut d = self.curr;
        loop {
            d -= 1;
            if d == 0 {
                d = self.size;
            }
            if d != r && d != r2 && d != r3 {
                break;
            }
        }
        self.arr[self.curr] = self.arr[r3];
        self.arr[r3] = self.arr[d];
        self.arr[d] = r;
        self.curr = self.arr[self.curr];
        self.moves += 1;
    }

    fn pair_after_one(&self) -> (usize, usize) {
        let first = self.arr[1];
        (first, self.arr[first])
    }

    #[allow(dead_code)]
    fn one_first(&self) -> Vec<usize> {
        self.iter_from_one().take(self.size).collect()
    }

    fn iter_from_one(&self) -> Labels {
        Labels {
            arr: &self.arr,
            idx: 1,
        }
    }
}

struct Labels<'a> {
    arr: &'a Vec<usize>,
    idx: usize,
}

impl<'a> Iterator for Labels<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.idx;
        self.idx = self.arr[self.idx];
        Some(r)
    }
}

impl From<&str> for Cups {
    fn from(s: &str) -> Self {
        let mut seed = VecDeque::with_capacity(s.len());
        let mut n = s.parse::<usize>().unwrap();
        while n > 0 {
            seed.push_front(n % 10);
            n /= 10;
        }
        let slices = seed.as_mut_slices();
        assert_eq!(0, slices.1.len());
        Cups::new(slices.0)
    }
}

impl Display for Cups {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for d in self.iter_from_one().take(self.size).skip(1) {
            write!(f, "{}", d)?;
        }
        Ok(())
    }
}
