use crate::timed_block;
use crate::y2019d22_slam_shuffle::operations::Op::*;
use crate::y2019d22_slam_shuffle::operations::{operations, Op};

#[cfg(test)]
mod test;

mod operations;

const DECK_SIZE: usize = 119315717514047;
const ITERATIONS: usize = 101741582076661;

pub fn solve(_: &str) {
    let ans = timed_block("Part One", || part_one(2019, 10007));
    println!("{}", ans);
    let ans = timed_block("Part Two", || {
        part_two(2020, DECK_SIZE, ITERATIONS % 100000)
    });
    println!("{}", ans);
}

fn part_one(card: usize, deck_size: usize) -> usize {
    part_one_n(card, deck_size, 1)
}

fn part_one_n(mut card: usize, deck_size: usize, iterations: usize) -> usize {
    let ops = operations();
    for _ in 0..iterations {
        card = shuffle(&ops, card, deck_size);
    }
    card
}

fn part_two(mut position: usize, deck_size: usize, iterations: usize) -> usize {
    // if it's "closer", go around the other direction
    let forward_count = deck_size - iterations - 1;
    if forward_count < iterations {
        return part_one_n(position, deck_size, forward_count);
    }
    let unops = reverse_operations(&operations(), deck_size);
    for _ in 0..iterations {
        position = shuffle(&unops, position, deck_size);
    }
    position
}

fn reverse_operations(ops: &[Op], deck_size: usize) -> Vec<Op> {
    ops.iter()
        .rev()
        .map(|op| match op {
            Reverse => Reverse,
            Cut(n) => Uncut(*n),
            Uncut(n) => Cut(*n),
            Deal(n) => Deal(mult_inv(*n, deck_size)),
        })
        .collect::<Vec<_>>()
}

fn shuffle(ops: &[Op], card: usize, deck_size: usize) -> usize {
    ops.iter().fold(card, |c, op| match op {
        Reverse => deck_size - c - 1,
        Cut(n) => (deck_size + c - n) % deck_size,
        Uncut(n) => (c + n) % deck_size,
        Deal(n) => mult_mod(c, *n, deck_size),
    })
}

/// Finds the multiplicative inverse of `a mod m`.
fn mult_inv(a: usize, m: usize) -> usize {
    bin_pow(a, m - 2, m)
}

/// Finds `a ^ b mod m` using binary exponentiation.
fn bin_pow(mut a: usize, mut b: usize, m: usize) -> usize {
    a %= m;
    let mut res = 1;
    while b > 0 {
        if b & 1 != 0 {
            res = mult_mod(res, a, m);
        }
        a = mult_mod(a, a, m);
        b >>= 1;
    }
    return res;
}

/// Finds `a * b mod m` while avoiding overflow.
fn mult_mod(mut a: usize, mut b: usize, m: usize) -> usize {
    let mut res = 0;
    a = a % m;
    while b > 0 {
        if b % 2 == 1 {
            res = (res + a) % m;
        }
        a = (a * 2) % m;
        b /= 2;
    }
    res % m
}
