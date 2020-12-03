#[macro_use]
extern crate lazy_static;

use aoc_2020 as aoc;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
struct Policy {
    min: usize,
    max: usize,
    char: char,
}

impl Policy {
    fn is_valid(&self, pw: &str) -> bool {
        let first = self.test_char(pw, self.min - 1);
        let last = self.test_char(pw, self.max - 1);
        first ^ last
    }

    fn test_char(&self, pw: &str, i: usize) -> bool {
        pw.len() > i && pw.chars().nth(i).expect("failed to get char") == self.char
    }
}

#[derive(Debug)]
struct Record {
    policy: Policy,
    password: String,
}

lazy_static! {
    static ref RECORD_RE: Regex = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
}

impl FromStr for Record {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = RECORD_RE.captures(s).unwrap();
        Ok(Record {
            policy: Policy {
                min: parts.get(1).unwrap().as_str().parse().unwrap(),
                max: parts.get(2).unwrap().as_str().parse().unwrap(),
                char: parts.get(3).unwrap().as_str().chars().next().unwrap(),
            },
            password: String::from(parts.get(4).unwrap().as_str()),
        })
    }
}

fn main() {
    let database = aoc::read_lines(|s| s.parse::<Record>().unwrap());
    let num_valid = database
        .iter()
        .filter(|r| r.policy.is_valid(&r.password))
        .count();
    println!("{}", num_valid)
}
