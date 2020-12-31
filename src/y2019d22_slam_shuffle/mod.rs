use crate::timed_block;
use crate::y2019d22_slam_shuffle::operations::Op::*;
use crate::y2019d22_slam_shuffle::operations::{operations, Op};

#[cfg(test)]
mod test;

mod operations;

pub fn solve(_: &str) {
    println!("{}", timed_block("Part One", || part_one()));
}

fn part_one() -> usize {
    // use the hard coded stuff
    shuffle(&operations(), 2019, 10007)
}

fn shuffle(ops: &[Op], card: usize, deck_size: usize) -> usize {
    ops.iter().fold(card, |c, op| match op {
        Reverse => deck_size - c - 1,
        Cut(n) => deck_size + c - n,
        Uncut(n) => c + n,
        Deal(n) => c * n,
    } % deck_size)
}
