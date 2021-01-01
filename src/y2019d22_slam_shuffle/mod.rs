use crate::y2019d22_slam_shuffle::operations::Op::*;
use crate::y2019d22_slam_shuffle::operations::{operations, Op};
use crate::{timed_block, with_duration};
use std::fmt::Display;
use std::time::Instant;

#[cfg(test)]
mod test;

mod operations;

const DECK_SIZE: usize = 119_315_717_514_047;
const ITERATIONS: usize = 101_741_582_076_661;

///
/// For unknown reasons I can't even guess at, this configuration runs
/// well more than three times faster compared to any of:
///
/// 1.  remove the benchmark of `go_back`
/// 1.  inline the benchmark of `go_back`, instead of using `bench`
/// 1.  use `bench` for `go_forward`, instead of inlining it
/// 1.  use `with_duration` for `go_forward` instead of manually
///     computing (the `with_duration` inside `bench` is fine).
///
/// In all four configurations, the returned answers are the same, as
/// you'd expect.
///
pub fn solve(_: &str) {
    let ans = timed_block("Part One", || go_forward(2019, 10007, 1));
    println!("{}", ans);

    let ans = bench(
        "Benchmark Part Two (literal)",
        ITERATIONS,
        1_000_000_000,
        go_back,
    );
    if 29649929027069 != ans {
        println!(
            "\nERROR: got {:>15}\n  expected {:>15}\n",
            ans, 29649929027069 as usize
        );
    }

    let total_iters = DECK_SIZE - ITERATIONS - 1;
    let test_iters = total_iters / 5_000_000;
    let start = Instant::now();
    let ans = go_forward(2020, DECK_SIZE, test_iters);
    let d = start.elapsed();
    println!(
        "{}\n  answer {}\n  took   {:?}\n  expect {:.1} days",
        "Benchmark Part Two (reversed)",
        ans,
        d,
        d.as_secs_f32() / 86_400_f32 * total_iters as f32 / test_iters as f32,
    );
    assert_eq!(23436842529065, ans);

    // TOO SLOW! Extrapolation above indicates about one month of CPU time.
    // let ans = timed_block("Part Two", || {
    //     // part_two(2020, DECK_SIZE, ITERATIONS) // 18.5 YEARS!
    //     part_one_n(2020, DECK_SIZE, DECK_SIZE - ITERATIONS - 1)
    // });
    // println!("{}", ans);
}

fn bench<T>(lbl: &str, total_iters: usize, factor: usize, f: fn(usize, usize, usize) -> T) -> T
where
    T: Display,
{
    let test_iters = total_iters / factor;
    let (ans, d) = with_duration(|| f(2020, DECK_SIZE, test_iters));
    println!(
        "{}\n  answer {}\n  took   {:?}\n  expect {:.1} days",
        lbl,
        ans,
        d,
        d.as_secs_f32() / 86_400_f32 * total_iters as f32 / test_iters as f32,
    );
    ans
}

fn go_forward(mut card: usize, deck_size: usize, iterations: usize) -> usize {
    let ops = operations(deck_size);
    for _ in 0..iterations {
        card = shuffle(&ops, card, deck_size);
    }
    card
}

fn go_back(mut position: usize, deck_size: usize, iterations: usize) -> usize {
    let unops = reverse_operations(&operations(deck_size), deck_size);
    for _ in 0..iterations {
        position = shuffle(&unops, position, deck_size);
    }
    position
}

fn reverse_operations(ops: &[Op], deck_size: usize) -> Vec<Op> {
    ops.iter()
        .rev()
        .map(|op| match *op {
            Reverse(n) => Reverse(n),
            Cut(n, u) => Cut(u, n),
            Deal(n) => Deal(mult_inv(n, deck_size)),
        })
        .collect::<Vec<_>>()
}

fn shuffle(ops: &[Op], card: usize, deck_size: usize) -> usize {
    ops.iter().fold(card, |c, op| match *op {
        Reverse(n) => n - c,
        Cut(n, u) => {
            if c < n {
                c + u
            } else {
                c - n
            }
        }
        Deal(n) => mult_mod(c, n, deck_size),
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
    if let Some(r) = a.checked_mul(b) {
        return r % m;
    }
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
