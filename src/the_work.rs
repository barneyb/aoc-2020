use aoc_2020::read_input;

pub fn the_work() {
    let (start, busses) = load(&read_input());
    println!("{} {:?}", start, busses);
    let (next, bus) = earliest_departure(start, &busses);
    println!("{} {}", next, bus);
    println!("{:?}", next * bus);
}

fn load(input: &str) -> (usize, Vec<usize>) {
    let mut lines = input.lines();
    let start = lines.next().unwrap().parse::<usize>().unwrap();
    let busses = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|s| s != &"x")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    (start, busses)
}

fn earliest_departure(start: usize, busses: &[usize]) -> (usize, usize) {
    let mut soonest = start;
    let mut bus_num = 0;
    for &b in busses {
        let next_time = b - (start % b);
        if next_time < soonest {
            soonest = next_time;
            bus_num = b;
        }
    }
    (soonest, bus_num)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_ONE: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn test_load() {
        let (start, busses) = load(EXAMPLE_ONE);
        assert_eq!(939, start);
        assert_eq!(vec![7, 13, 59, 31, 19], busses)
    }

    #[test]
    fn example_one() {
        assert_eq!((5, 59), earliest_departure(939, &[7, 13, 59, 31, 19]));
    }
}
