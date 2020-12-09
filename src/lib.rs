use std::fs;
use std::time::{Duration, Instant};

pub mod find_pairs;

pub fn read_input() -> String {
    fs::read_to_string("input.txt").unwrap().trim().to_string()
}

pub fn read_lines<T, F>(f: F) -> Vec<T>
where
    F: Fn(&str) -> T,
{
    read_input().lines().map(f).collect::<Vec<T>>()
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
