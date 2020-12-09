use aoc_2020 as aoc;

use aoc_2020::find_pairs::PairFinder;

pub fn the_work() {
    let codes = aoc::read_lines(|l| l.parse::<i64>().unwrap());
    println!("{:?}", find_first_error(&codes, 25));
}

fn find_first_error(codes: &[i64], preamble_len: usize) -> Option<i64> {
    for i in preamble_len..codes.len() {
        if let None = codes[(i - preamble_len)..i].find_pair_with_sum(codes[i]) {
            return Some(codes[i]);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = "
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn example_one() {
        let codes = EXAMPLE_INPUT
            .trim()
            .lines()
            .map(|l| l.parse().unwrap())
            .collect::<Vec<i64>>();
        assert_eq!(Some(127), find_first_error(&codes, 5));
    }
}
