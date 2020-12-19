use aoc_2020::read_input;
use std::collections::HashMap;

pub fn the_work() {
    let s = read_input();
    println!("{:?}", do_the_thing(&s));
}

fn do_the_thing(s: &str) -> usize {
    let mut mask = [None; 36];
    let mut memory = HashMap::new();
    for line in s.trim().lines() {
        match &line[0..4] {
            "mask" => {
                for (i, c) in line[6..].trim().chars().enumerate() {
                    mask[i] = match c {
                        'X' => None,
                        '1' => Some(1),
                        '0' => Some(0),
                        _ => panic!("Unrecognized mask digit '{}'", c),
                    }
                }
            }
            "mem[" => {
                let icb = line.find(']').expect("no closing bracket");
                let addr = line[4..icb].parse::<usize>().expect("couldn't parse addr");
                let val = line[(icb + 3)..]
                    .trim()
                    .parse::<usize>()
                    .expect("couldn't parse value");
                memory.insert(addr, apply_mask(&mask, val));
            }
            _ => panic!("Unrecognized line '{}'", line),
        }
    }
    memory.values().sum()
}

fn apply_mask(mask: &[Option<usize>], val: usize) -> usize {
    let mut sum: usize = 0;
    let mut step: usize = 1;
    for i in (0..36).rev() {
        if let Some(d) = mask[i] {
            sum += d * step;
        } else {
            sum |= val & step;
        }
        step <<= 1;
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_ONE: &str = "
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn example_one() {
        assert_eq!(165, do_the_thing(&EXAMPLE_ONE));
    }
}
