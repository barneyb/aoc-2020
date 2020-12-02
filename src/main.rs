use aoc_2020 as aoc;

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
    let database = aoc::read_lines(|s| {
        let di = s.find('-').expect("failed to find dash");
        let si = s.find(' ').expect("failed to find space");
        let ci = s.find(':').expect("failed to find colon");
        let min = s[0..di].parse().expect("failed to parse min");
        let max = s[(di + 1)..si].parse().expect("failed to parse max");
        let &char = &s[(si + 1)..ci].chars().next().expect("failed to get policy char");
        let password = String::from(s[(ci + 1)..s.len()].trim());
        Record {
            policy: Policy {
                min,
                max,
                char,
            },
            password,
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
