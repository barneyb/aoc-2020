#[macro_use]
extern crate lazy_static;

use std::fs;
use std::time::{Duration, Instant};

pub mod boarding_pass;
pub mod passport;

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
/// use aoc_2020::unwrap_paragraphs;
///
/// let s = "I'm a
/// long paragraph
/// of text.
///
/// A second paragraph!";
/// assert_eq!(unwrap_paragraphs(s), vec!["I'm a long paragraph of text.", "A second paragraph!"])
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

pub fn benchmark<T, F>(iterations: usize, f: F)
where
    F: Fn() -> T,
{
    let mut total = Duration::new(0, 0);
    for _ in 0..iterations {
        let (_, elapsed) = with_duration(&f);
        total += elapsed;
    }
    println!("{:?}", total / iterations as u32);
}
