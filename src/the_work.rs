// use aoc_2020 as aoc;

pub fn the_work() {
    // let codes = aoc::read_lines(|l| l.parse::<u64>().unwrap());
}

fn find_first_error(codes: &[u64], preamble_len: usize) -> u64 {
    let mut preamble = Vec::with_capacity(preamble_len);
    let mut code_iter = codes.iter();
    for _ in 0..preamble_len {
        preamble.push(*code_iter.next().unwrap());
    }


    1
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
            .collect::<Vec<u64>>();
        assert_eq!(127, find_first_error(&codes, 5));
    }

}
