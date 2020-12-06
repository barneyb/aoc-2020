use aoc_2020 as aoc;
use aoc_2020::histogram::Histogram;

pub fn the_work() {
    let input = aoc::read_input();
    let (one, two) = both_parts(&input);
    println!("{}", one);
    println!("{}", two);
}

fn both_parts(input: &str) -> (usize, usize) {
    const A: usize = 'a' as usize;
    input
        .split("\n\n")
        .map(|group| {
            let mut hist = Vec::new();
            for c in group.chars().filter(|&c| c != '\n') {
                hist.increment((c as usize) - A);
            }
            let person_count = group.split('\n').count();
            hist.iter().fold((0, 0), |a, &count| {
                if count == 0 {
                    return a;
                }
                (a.0 + 1, a.1 + if count == person_count { 1 } else { 0 })
            })
        })
        .fold((0, 0), |a, p| (a.0 + p.0, a.1 + p.1))
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn test_both_parts() {
        assert_eq!((11, 6), both_parts(EXAMPLE_INPUT))
    }

    #[test]
    fn run_benchmarks() {
        use aoc_2020::benchmark_for;
        use std::time::Duration;

        fn old_style(input: &str) -> usize {
            let a = 'a' as usize;
            input
                .split("\n\n")
                .map(|g| {
                    let mut map = [0; 26];
                    for c in g.chars().filter(|&c| c != '\n') {
                        map[(c as usize) - a] += 1;
                    }
                    let pc = g.split('\n').count();
                    // filter/count is a bit slower than fold
                    // map.iter().filter(|&&c| c == pc).count()
                    map.iter().fold(0, |s, &c| if c == pc { s + 1 } else { s })
                })
                .sum()
        }

        benchmark_for(Duration::from_secs(1), || old_style(EXAMPLE_INPUT));
        benchmark_for(Duration::from_secs(1), || both_parts(EXAMPLE_INPUT));
    }
}
