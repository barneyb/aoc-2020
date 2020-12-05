#[macro_use]
extern crate lazy_static;

use aoc_2020 as aoc;

mod passport;

use crate::passport::{break_on_blank_lines, is_valid, parse};

fn main() {
    let input = aoc::read_input();
    let passports = break_on_blank_lines(&input);
    println!(
        "{}",
        passports
            .iter()
            .map(|s| parse(s))
            .filter(|p| is_valid(&p))
            .count()
    );
}
