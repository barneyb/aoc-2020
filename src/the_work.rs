use aoc_2020::histogram::Histogram;
use aoc_2020::read_lines;

pub fn the_work() {
    let adapters = read_lines(|l| l.parse::<usize>().unwrap());
    println!("{}", jolt_distribution(&adapters));
}

fn jolt_distribution(adapters: &[usize]) -> usize {
    let mut scratch = Vec::from(adapters);
    scratch.sort();
    scratch.push(scratch[scratch.len() - 1] + 3);
    let mut hist = Vec::new();
    scratch.iter().fold(&0, |a, b| {
        hist.increment_bucket(b - a);
        b
    });
    hist[1] * hist[3]
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
        assert_eq!(35, jolt_distribution(&EXAMPLE_ONE));
        assert_eq!(220, jolt_distribution(&EXAMPLE_TWO));
    }
}
