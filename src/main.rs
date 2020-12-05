#[macro_use]
extern crate lazy_static;

use aoc_2020 as aoc;

mod passport;

use crate::passport::Passport;

fn main() {
    let input = aoc::read_input();
    let passports = aoc::unwrap_paragraphs(&input);
    println!(
        "{}",
        passports
            .iter()
            .map(|s| s.parse::<Passport>().unwrap())
            .filter(|p| p.is_valid())
            .count()
    );
}
