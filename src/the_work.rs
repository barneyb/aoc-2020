use aoc_2020::histogram::Histogram;
use aoc_2020::read_lines;

pub fn the_work() {
    let adapters = read_lines(|l| l.parse::<usize>().unwrap());
    let diffs = compute_diffs(&adapters);
    println!("{}", jolt_distribution(&diffs));
    println!("{}", count_arrangements(&diffs));
}

fn jolt_distribution(diffs: &[usize]) -> usize {
    let mut hist = Vec::new();
    for &d in diffs {
        hist.increment_bucket(d);
    }
    hist[1] * hist[3]
}

fn compute_diffs(adapters: &[usize]) -> Vec<usize> {
    let mut scratch = Vec::from(adapters);
    scratch.sort();
    scratch.push(scratch[scratch.len() - 1] + 3);
    let mut diffs = Vec::new();
    let mut last = 0;
    for jolts in scratch {
        diffs.push(jolts - last);
        last = jolts;
    }
    diffs
}

fn count_arrangements(diffs: &[usize]) -> usize {
    let mut product = 1;
    let mut len = 0;
    for &d in diffs {
        if d == 1 {
            len += 1;
        } else if len > 0 {
            product *= match len {
                1 => 1,
                2 => 2,
                3 => 4,
                4 => 7,
                _ => panic!("Runs of {} aren't supported", len),
            };
            len = 0;
        }
    }
    product
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_ONE: [usize; 11] = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    const EXAMPLE_TWO: [usize; 31] = [
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ];

    #[test]
    fn test_jolt_distribution() {
        let diffs = compute_diffs(&EXAMPLE_ONE);
        assert_eq!(35, jolt_distribution(&diffs));
        let diffs = compute_diffs(&EXAMPLE_TWO);
        assert_eq!(220, jolt_distribution(&diffs));
    }

    #[test]
    fn test_arrangements() {
        let diffs = compute_diffs(&EXAMPLE_ONE);
        assert_eq!(8, count_arrangements(&diffs));
        let diffs = compute_diffs(&EXAMPLE_TWO);
        assert_eq!(19208, count_arrangements(&diffs));
    }
}
