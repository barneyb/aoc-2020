use aoc_2020::read_input;

pub fn the_work() {
    let (start, busses) = load(&read_input());
    println!("{} {:?}", start, busses);
    let (next, bus) = earliest_departure(start, &busses);
    println!("{} {}", next, bus);
    println!("{:?}", next * bus);
    println!("{}", win_contest(&busses));
}

fn load(input: &str) -> (usize, Vec<(usize, usize)>) {
    let mut lines = input.lines();
    let start = lines.next().unwrap().parse::<usize>().unwrap();
    let busses = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, s)| s != &"x")
        .map(|(i, s)| (i, s.parse::<usize>().unwrap()))
        .collect::<Vec<(usize, usize)>>();
    (start, busses)
}

fn earliest_departure(start: usize, busses: &[(usize, usize)]) -> (usize, usize) {
    let mut soonest = start;
    let mut bus_num = 0;
    for &(_, b) in busses {
        let next_time = b - (start % b);
        if next_time < soonest {
            soonest = next_time;
            bus_num = b;
        }
    }
    (soonest, bus_num)
}

fn win_contest(busses: &[(usize, usize)]) -> usize {
    let mut with_ones = Vec::with_capacity(busses[busses.len() - 1].0);
    for &(i, n) in busses {
        for _ in 0..(i - with_ones.len()) {
            with_ones.push(1);
        }
        with_ones.push(n);
    }
    calc(&with_ones)
}

fn calc(bs: &[usize]) -> usize {
    // duplicate it into some space for me!
    let mut scan = bs.iter().map(|n| *n).collect::<Vec<_>>();
    // println!("{:?}", scan);
    'outer: loop {
        for i in 1..scan.len() {
            if scan[i] <= scan[i - 1] {
                if bs[i] == 1 {
                    scan[i] = scan[i - 1] + 1;
                } else {
                    let gap = scan[i - 1] - scan[i];
                    scan[i] += (gap / bs[i] + 1) * bs[i];
                }
                // println!("  set {} to {}", i, scan[i]);
                if scan[i] != scan[i - 1] + 1 {
                    scan[0] += bs[0..i].iter().product::<usize>();
                    // println!("  cycle!");
                    continue 'outer;
                }
            }
        }
        break;
    }
    // println!("{:?}", scan);
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
        assert_eq!(vec![(0, 7), (1, 13), (4, 59), (6, 31), (7, 19)], busses);
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
            assert_eq!(3417, calc(&[17, 1, 13, 19]));
            assert_eq!(754018, calc(&[67, 7, 59, 61]));
            assert_eq!(1202161486, calc(&[1789, 37, 47, 1889]));
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
