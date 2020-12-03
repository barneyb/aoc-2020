use aoc_2020 as aoc;
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

impl FromStr for Record {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let di = s.find('-').expect("failed to find dash");
        let si = s.find(' ').expect("failed to find space");
        let ci = s.find(':').expect("failed to find colon");
        Ok(Record {
            policy: Policy {
                min: s[0..di].parse().expect("failed to parse min"),
                max: s[(di + 1)..si].parse().expect("failed to parse max"),
                char: s[(si + 1)..ci]
                    .chars()
                    .next()
                    .expect("failed to get policy char"),
            },
            password: String::from(s[(ci + 1)..s.len()].trim()),
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
