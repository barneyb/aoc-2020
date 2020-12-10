use aoc_2020 as aoc;

use aoc_2020::find_pairs::PairFinder;

pub fn the_work() {
    let codes = aoc::read_lines(|l| l.parse::<i64>().unwrap());
    let first_error = find_first_error(&codes, 25).unwrap();
    println!("{}", first_error);
    println!("{}", find_weakness(&codes, first_error));
}

fn find_weakness(codes: &[i64], first_error: i64) -> i64 {
    let range = find_range_of(&codes, first_error);
    range.iter().min().unwrap() + range.iter().max().unwrap()
}

fn find_first_error(codes: &[i64], preamble_len: usize) -> Option<i64> {
    for i in preamble_len..codes.len() {
        if let None = codes[(i - preamble_len)..i].find_pair_with_sum(codes[i]) {
            return Some(codes[i]);
        }
    }
    None
}

fn find_range_of(codes: &[i64], sum: i64) -> &[i64] {
    for i in 0..codes.len() {
        let mut s = codes[i];
        for j in (i + 1)..codes.len() {
            s += codes[j];
            if s == sum {
                return &codes[i..=j];
            }
            if s > sum {
                break;
            }
        }
    }
    panic!("No range totaling {} found", sum);
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_CODES: [i64; 20] = [
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];

    #[test]
    fn example_one() {
        assert_eq!(Some(127), find_first_error(&EXAMPLE_CODES, 5));
    }

    #[test]
    fn example_two() {
        assert_eq!(&[15, 25, 47, 40], find_range_of(&EXAMPLE_CODES, 127));
        assert_eq!(62, find_weakness(&EXAMPLE_CODES, 127));
    }
}
