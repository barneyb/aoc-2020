#[macro_use]
extern crate lazy_static;

extern crate num_traits;

use std::fmt;
use std::fs;
use std::time::{Duration, Instant};

pub mod ascii;
pub mod boarding_pass;
pub mod histogram;
pub mod passport;
pub mod password;

pub fn read_input() -> String {
    fs::read_to_string("input.txt").unwrap().trim().to_string()
}

pub fn read_lines<T, F>(f: F) -> Vec<T>
where
    F: Fn(&str) -> T,
{
    read_input().lines().map(f).collect::<Vec<T>>()
}

/// I convert a multi-line `&str` into a `Vec<String>` by splitting on "paragraph breaks" which are
/// defined as two sequential newline characters. Each paragraph is further "unwrapped" by replacing
/// all internal newlines w/ a single space.
///
/// # Examples
///
/// ```
/// let s = "I'm a
/// long paragraph
/// of text.
///
/// A second paragraph!";
///
/// assert_eq!(aoc_2020::unwrap_paragraphs(s), vec![
///     "I'm a long paragraph of text.",
///     "A second paragraph!"
/// ])
/// ```
pub fn unwrap_paragraphs(input: &str) -> Vec<String> {
    input.split("\n\n").map(|s| s.replace('\n', " ")).collect()
}

pub fn with_duration<T, F>(f: F) -> (T, Duration)
where
    F: Fn() -> T,
{
    let start = Instant::now();
    let r = f();
    let elapsed = start.elapsed();
    (r, elapsed)
}

pub fn print_duration<T, F>(f: F) -> T
where
    F: Fn() -> T,
{
    let (r, elapsed) = with_duration(f);
    println!("{:?}", elapsed);
    r
}

pub struct Benchmark {
    iterations: usize,
    total_time: Duration,
    average_time: Duration,
}

impl Benchmark {
    fn new(iterations: usize, total_time: Duration) -> Benchmark {
        let b = Benchmark {
            iterations,
            total_time,
            average_time: total_time / iterations as u32,
        };
        // printing in a constructor is weird, but it matches the normal use case
        println!("{}", &b);
        b
    }
}

impl fmt::Display for Benchmark {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} average, {:?} total ({} iterations)",
            self.average_time, self.total_time, self.iterations,
        )
    }
}

/// I benchmark the passed `Fn` for up to 500ms _or_ 1000 iterations, whichever comes first. This
/// is a great place to start if you have no idea about performance characteristics. If the results
/// are unsatisfactory, switch to `benchmark_for` or `benchmark_times` with your newfound knowledge.
pub fn benchmark<T, F>(f: F) -> Benchmark
where
    F: Fn() -> T,
{
    let max_runtime = Duration::from_millis(500);
    let max_iterations = 1000;

    let start = Instant::now();
    let mut times: usize = 0;
    let mut total = Duration::new(0, 0);
    for _ in 0..max_iterations {
        times += 1;
        bench_itr(&f, &mut total);
        if start.elapsed() >= max_runtime {
            break;
        }
    }
    Benchmark::new(times, total)
}

pub fn benchmark_for<T, F>(duration: Duration, f: F) -> Benchmark
where
    F: Fn() -> T,
{
    let start = Instant::now();
    let mut times: usize = 0;
    let mut total = Duration::new(0, 0);
    while start.elapsed() < duration {
        times += 1;
        bench_itr(&f, &mut total)
    }
    Benchmark::new(times, total)
}

pub fn benchmark_times<T, F>(times: usize, f: F) -> Benchmark
where
    F: Fn() -> T,
{
    let mut total = Duration::new(0, 0);
    for _ in 0..times {
        bench_itr(&f, &mut total)
    }
    Benchmark::new(times, total)
}

#[inline]
fn bench_itr<T, F>(f: F, duration: &mut Duration)
where
    F: Fn() -> T,
{
    let (_, elapsed) = with_duration(&f);
    *duration = *duration + elapsed;
}
