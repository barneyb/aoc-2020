use aoc_2020::read_input;

pub fn the_work() {
    let (start, busses) = load(&read_input());
    println!("{} {:?}", start, busses);
    let (next, bus) = earliest_departure(start, &busses);
    println!("{} {}", next, bus);
    println!("{:?}", next * bus);
    println!("{}", win_contest(&busses));
}

fn load(input: &str) -> (usize, Vec<usize>) {
    let mut lines = input.lines();
    let start = lines.next().unwrap().parse::<usize>().unwrap();
    let busses = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| match s {
            "x" => 1,
            _ => s.parse::<usize>().unwrap(),
        })
        .collect::<Vec<usize>>();
    (start, busses)
}

fn earliest_departure(start: usize, busses: &[usize]) -> (usize, usize) {
    let mut best = (start, 0);
    for &b in busses {
        // it's a one-bus
        if b == 1 {
            // to the next bus!
            continue;
        }
        // how long until it next depart
        let next_departure = b - (start % b);
        // it's the best so far
        if next_departure < best.0 {
            // write it down
            best = (next_departure, b);
        }
    }
    best
}

fn win_contest(busses: &[usize]) -> usize {
    // duplicate it into some space for me!
    let mut scan = busses.iter().map(|n| *n).collect::<Vec<_>>();
    'outer: loop {
        // for each bus except the first
        for i in 1..scan.len() {
            // it's not one more than the prior bus
            if scan[i] <= scan[i - 1] {
                // it's a one-bus
                if busses[i] == 1 {
                    // just set it to the right value
                    scan[i] = scan[i - 1] + 1;
                    // to the next bus!
                    continue;
                } else {
                    // compute how far behind it is
                    let gap = scan[i - 1] - scan[i];
                    // move it forward by at least that much
                    scan[i] += (gap / busses[i] + 1) * busses[i];
                }
                // we overshot
                if scan[i] != scan[i - 1] + 1 {
                    // move the first bus forward by the minimum step size for the prior busses
                    scan[0] += busses[0..i].iter().product::<usize>();
                    // give up on this cycle
                    continue 'outer;
                }
            }
        }
        // all in sequence!
        break;
    }
    scan[0]
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc_2020::benchmark_times;

    const EXAMPLE_ONE: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn example_one() {
        let (start, busses) = load(EXAMPLE_ONE);
        assert_eq!(939, start);
        assert_eq!(vec![7, 13, 1, 1, 59, 1, 31, 19], busses);
        let (next, bus) = earliest_departure(start, &busses);
        assert_eq!((5, 59), (next, bus));
        assert_eq!(295, next * bus)
    }

    #[test]
    fn example_two() {
        let (_, busses) = load(EXAMPLE_ONE);
        assert_eq!(1068781, win_contest(&busses));
    }

    #[test]
    fn example_three() {
        let (_, busses) = load("0\n17,x,13,19");
        assert_eq!(3417, win_contest(&busses));
    }

    #[test]
    fn example_four() {
        let (_, busses) = load("0\n67,7,59,61");
        assert_eq!(754018, win_contest(&busses));
    }

    #[test]
    fn example_five() {
        let (_, busses) = load("0\n67,x,7,59,61");
        assert_eq!(779210, win_contest(&busses));
    }

    #[test]
    fn example_six() {
        let (_, busses) = load("0\n67,7,x,59,61");
        assert_eq!(1261476, win_contest(&busses));
    }

    #[test]
    fn example_seven() {
        let (_, busses) = load("0\n1789,37,47,1889");
        assert_eq!(1202161486, win_contest(&busses));
    }

    #[test]
    fn do_calc() {
        benchmark_times(1, || {
            assert_eq!(3417, win_contest(&[17, 1, 13, 19]));
            assert_eq!(754018, win_contest(&[67, 7, 59, 61]));
            assert_eq!(1202161486, win_contest(&[1789, 37, 47, 1889]));
        });
    }

    #[test]
    fn scratch() {
        print(&[(0, 17), (2, 13), (3, 19)]);
        println!("{}", "=".repeat(80));
        print(
            &vec![67, 7, 59, 61]
                .into_iter()
                .enumerate()
                .collect::<Vec<_>>(),
        );
        println!("{}", "=".repeat(80));
        print(&[(0, 67), (1, 7), (3, 59), (4, 61)]);
        // print(&vec![1789,37,47,1889].into_iter().enumerate().collect::<Vec<_>>());
    }

    fn print(bs: &[(usize, i32)]) {
        let mut curr = bs.iter().map(|it| it.1).collect::<Vec<_>>();
        let mut found = 0;
        let mut prior: Option<Vec<i32>> = None;
        loop {
            let t = curr
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.cmp(b))
                .unwrap()
                .0;
            curr[t] += bs[t].1;
            if curr
                .iter()
                .enumerate()
                .all(|(i, n)| *n == curr[0] + bs[i].0 as i32)
            {
                let next = curr
                    .iter()
                    .enumerate()
                    .map(|(i, n)| n / bs[i].1)
                    .collect::<Vec<_>>();
                print!(
                    "{:?}",
                    next.iter()
                        .enumerate()
                        .map(|(i, n)| format!("{} * {} = {}", bs[i].1, n, bs[i].1 * n))
                        .collect::<Vec<_>>()
                );
                match prior {
                    Some(p) => println!(
                        " : {:?}",
                        p.iter()
                            .zip(next.iter())
                            .map(|(a, b)| b - a)
                            .collect::<Vec<_>>()
                    ),
                    None => println!(),
                }
                found += 1;
                if found == 2 {
                    break;
                }
                prior = Some(next);
            }
        }
    }
}
