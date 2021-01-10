use crate::math::mult_mod;
use crate::y2019d22_slam_shuffle::operations::{
    bind_operation_list, parse_operation_list, reverse_operations, Op, Op::*,
};
use crate::{timed_block, with_duration};
use std::fmt::Display;

#[cfg(test)]
mod test;

mod operations;

const DECK_SIZE: usize = 119_315_717_514_047;
const ITERATIONS: usize = 101_741_582_076_661;

/// The compiler (optimizer?, LLVM?) is doing something stupid that causes this to run a bit more
/// than three times slower than it might otherwise. With very careful arrangements of using and not
/// using `bench`, using and not using `with_duration`, and various `inline` attributes, I have been
/// able to toggle between "speed x" and "speed x/3" (or "speed 3x", if you prefer) without any
/// apparent rhyme or reason.
///
/// It's clear that the idiot here is me, but I don't even have a guess at is causing the
/// discrepancy, other than "appears to be something with compiler optimizations."
///
/// However, at the end of the day, a 30 day runtime vs a 90 day runtime isn't really meaningfully
/// differentiated. So purely an academic interest, not something actually in my way.
pub fn solve(input: &str) {
    let raw_ops = parse_operation_list(&input);

    let ops = bind_operation_list(&raw_ops, 10007);
    let ans = timed_block("Part One", || shuffle(2019, &ops, 10007, 1));
    println!("{}", ans);

    let ops = bind_operation_list(&raw_ops, DECK_SIZE);
    let unops = reverse_operations(&ops);

    let ans = bench(
        "Benchmark Part Two (literal)",
        &unops,
        ITERATIONS,
        5_000_000_000,
        shuffle,
    );
    if 110243237903680 != ans {
        println!(
            "\nERROR: got {:>15}\n  expected {:>15}\n",
            ans, 110243237903680 as usize
        );
    }

    if cfg!(debug_assertions) {
        let ans = bench(
            "Benchmark Part Two (reversed)",
            &ops,
            DECK_SIZE - ITERATIONS - 1,
            100_000_000,
            shuffle,
        );
        assert_eq!(10531478815607, ans);
    } else {
        let ans = bench(
            "Benchmark Part Two (reversed)",
            &ops,
            DECK_SIZE - ITERATIONS - 1,
            10_000_000,
            shuffle,
        );
        assert_eq!(85445347441033, ans);
    }

    // TOO SLOW! Extrapolation above indicates about three months of CPU time.
    // let ans = timed_block("Part Two", || {
    //     // part_two(2020, DECK_SIZE, ITERATIONS) // 67 YEARS!
    //     part_one_n(2020, DECK_SIZE, DECK_SIZE - ITERATIONS - 1)
    // });
    // println!("{}", ans);
}

fn bench<T>(
    lbl: &str,
    ops: &[Op],
    total_iters: usize,
    factor: usize,
    f: fn(usize, &[Op], usize, usize) -> T,
) -> T
where
    T: Display,
{
    let test_iters = total_iters / factor;
    let (ans, d) = with_duration(|| f(2020, ops, DECK_SIZE, test_iters));
    println!(
        "{}\n  answer {}\n  took   {:?}\n  expect {:.1} days",
        lbl,
        ans,
        d,
        d.as_secs_f32() / 86_400_f32 * total_iters as f32 / test_iters as f32,
    );
    ans
}

fn shuffle(mut card: usize, ops: &[Op], _deck_size: usize, iterations: usize) -> usize {
    for _ in 0..iterations {
        card = ops.iter().fold(card, |c, op| match op {
            Reverse(sm1) => sm1 - c,
            Cut(n, u) => {
                if c < *n {
                    c + u
                } else {
                    c - n
                }
            }
            Deal(n, s) => mult_mod(c, *n, *s),
        });
    }
    card
}
