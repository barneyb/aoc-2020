use crate::math::{mult_inv, mult_mod};
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

    let ans = bench(
        "Benchmark Part Two (reversed)",
        &ops,
        DECK_SIZE - ITERATIONS - 1,
        50_000_000,
        shuffle,
    );
    assert_eq!(95670451920023, ans);

    let ans = bench(
        "Benchmark Part Two (montgomery)",
        &ops,
        DECK_SIZE - ITERATIONS - 1,
        50_000_000,
        montgomery_shuffle,
    );
    assert_eq!(95670451920023, ans);

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

#[allow(non_snake_case)]
fn get_redc_primes(R: u128, N: u128) -> (u128, u128) {
    let R_prime = mult_inv(R as usize, N as usize).unwrap() as u128;
    debug_assert_eq!(1, mult_mod(R as usize, R_prime as usize, N as usize));
    // RR' - NN' = 1 :: should be Hensel's lemma, but I don't understand...
    let N_prime = (R * R_prime - 1) / N;
    (R_prime, N_prime)
}

#[allow(non_snake_case)]
fn REDC(R: u128, N: u128, N_prime: u128, T: u128) -> u128 {
    let m = ((T % R) * N_prime) % R;
    let t = (T + m * N) / R;
    if t >= N {
        t - N
    } else {
        t
    }
}

#[allow(non_snake_case)]
fn REDC_pow_2(R_mod_and: u128, R_div_shift: u32, N: u128, N_prime: u128, T: u128) -> u128 {
    let m = ((T & R_mod_and) * N_prime) & R_mod_and;
    let t = (T + m * N) >> R_div_shift;
    if t >= N {
        t - N
    } else {
        t
    }
}

#[allow(non_snake_case)]
fn montgomery_shuffle(
    starting_card: usize,
    ops: &[Op],
    deck_size: usize,
    iterations: usize,
) -> usize {
    let mut card = starting_card as u128;

    let N = deck_size as u128;
    let R = 2_u128.pow((N as f64).log2().ceil() as u32);
    debug_assert!(R > N);
    let (_, N_prime) = get_redc_primes(R, N);
    let R_squared = R * R % N;

    let R_mod_and = R - 1;
    let R_div_shift = R_mod_and.count_ones();
    let redc = |T| REDC_pow_2(R_mod_and, R_div_shift, N, N_prime, T);

    #[derive(Debug)]
    enum MOp {
        Reverse(u128),
        Cut(u128),
        Deal(u128),
    }

    card = redc(card * R_squared);
    let mops = ops
        .iter()
        .map(|op| match op {
            Reverse(dsm1) => MOp::Reverse(redc(*dsm1 as u128 * R_squared) + N),
            Cut(n, _) => MOp::Cut(redc((deck_size - n) as u128 * R_squared)),
            Deal(n, ds) => {
                debug_assert_eq!(*ds, deck_size);
                MOp::Deal(redc(*n as u128 * R_squared))
            }
        })
        .collect::<Vec<_>>();

    for _ in 0..iterations {
        card = mops.iter().fold(card, |c, op| match op {
            MOp::Reverse(dsm1) => *dsm1 - c,
            MOp::Cut(n) => *n + c, // this may > N, but it'll fold away
            MOp::Deal(n) => redc(*n * c),
        })
    }

    redc(card) as usize
}

fn shuffle(mut card: usize, ops: &[Op], _deck_size: usize, iterations: usize) -> usize {
    for _ in 0..iterations {
        card = ops.iter().fold(card, |c, op| match op {
            Reverse(dsm1) => dsm1 - c,
            Cut(n, u) => {
                if c < *n {
                    c + u
                } else {
                    c - n
                }
            }
            Deal(n, ds) => mult_mod(c, *n, *ds),
        });
    }
    card
}
