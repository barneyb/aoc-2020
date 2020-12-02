use aoc_2020 as aoc;
use regex::Regex;

#[derive(Debug)]
struct Policy {
    min: usize,
    max: usize,
    char: char,
}

#[derive(Debug)]
struct Record {
    policy: Policy,
    password: String,
}

fn main() {
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
    let database = aoc::read_lines(|s| {
        let parts = re.captures(s).unwrap();
        Record {
            policy: Policy {
                min: parts.get(1).unwrap().as_str().parse().unwrap(),
                max: parts.get(2).unwrap().as_str().parse().unwrap(),
                char: parts.get(3).unwrap().as_str().chars().next().unwrap(),
            },
            password: String::from(parts.get(4).unwrap().as_str()),
        }
    });
    println!("{}", database.iter().filter(|r| {
        let min = r.policy.min - 1;
        let first = r.password.len() > min && r.password.chars().nth(min).expect("failed to get first char") == r.policy.char;
        let max = r.policy.max - 1;
        let last = r.password.len() > max && r.password.chars().nth(max).expect("failed to get second char") == r.policy.char;
        (first && !last) || (!first && last)
    }).count())
}
