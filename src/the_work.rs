use aoc_2020::{read_input, time_block};
use std::collections::HashMap;
use std::str::FromStr;

pub fn the_work() {
    let prog = time_block("Parsing", || parse_program(&read_input()));
    println!("{}", time_block("Part One", || part_one(&prog)));
    println!("{}", time_block("Part Two", || part_two(&prog)));
}

type Mask = (usize, usize); // (and, or)
type Memory = HashMap<usize, usize>;
type Program = Vec<Block>;

fn parse_program(s: &str) -> Program {
    let mut prog = Vec::new();
    for block in s.trim().split("mask = ").skip(1) {
        let mut lines = block.lines();
        let mut b = Block {
            mask: parse_mask(&lines.next().unwrap()),
            writes: Vec::new(),
        };
        for line in lines {
            b.writes.push(line.parse().unwrap())
        }
        prog.push(b);
    }
    prog
}

struct Block {
    mask: Mask,
    writes: Vec<Write>,
}

fn parse_mask(s: &str) -> Mask {
    let mut m_and = 0;
    let mut m_or = 0;
    for c in s.trim().chars() {
        m_and <<= 1;
        m_or <<= 1;
        match c {
            '0' => {}
            '1' => {
                m_and += 1;
                m_or += 1;
            }
            'X' => m_and += 1,
            _ => panic!("Unrecognized mask digit '{}'", c),
        }
    }
    (m_and, m_or)
}

struct Write {
    addr: usize,
    value: usize,
}

impl FromStr for Write {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let icb = s.find(']').expect("no closing bracket");
        let addr = s[4..icb].parse::<usize>().expect("couldn't parse addr");
        let value = s[(icb + 3)..]
            .trim()
            .parse::<usize>()
            .expect("couldn't parse value");
        Ok(Write { addr, value })
    }
}

fn part_one(prog: &Program) -> usize {
    let mut mem: Memory = HashMap::new();
    for b in prog {
        for w in &b.writes {
            mem.insert(w.addr, mask_value(&b.mask, w.value));
        }
    }
    mem.values().sum()
}

#[inline]
fn mask_value(&(m_and, m_or): &Mask, val: usize) -> usize {
    val & m_and | m_or
}

fn part_two(prog: &Program) -> usize {
    let mut mem: Memory = HashMap::new();
    for b in prog {
        for w in &b.writes {
            for a in decode_address(&b.mask, w.addr) {
                mem.insert(a, w.value);
            }
        }
    }
    mem.values().sum()
}

fn decode_address(&(m_and, m_or): &Mask, addr: usize) -> Vec<usize> {
    let floating = m_and ^ m_or;
    let result_count = 2usize.pow(floating.count_ones());
    debug_assert!(result_count < 1000, "That's a lot of results. You sure?");
    let mut result = Vec::with_capacity(result_count);
    result.push(0);
    let mut bit: usize = 1;
    let end = addr.max(m_and).max(floating);
    while bit <= end {
        if floating & bit != 0 {
            // where floating is one, need to bisect the address space
            for i in 0..result.len() {
                let a = result[i] | bit;
                result.push(a);
            }
        } else if m_and & bit == 0 {
            // where m_and is zero, grab addr's bit
            for a in &mut result {
                *a |= addr & bit;
            }
        } else {
            debug_assert!(m_and & bit != 0);
            // where m_and is one, set one
            for a in &mut result {
                *a |= bit;
            }
        }
        bit <<= 1;
    }
    debug_assert_eq!(
        result.capacity(),
        result.len(),
        "The result Vec wasn't filled?!"
    );
    result
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_ONE: &str = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    const MASK: Mask = (
        0b111111111111111111111111111111111101,
        0b000000000000000000000000000001000000,
    );

    #[test]
    fn sanity_check_and_or_pair() {
        let (m_and, m_or) = MASK;
        assert_eq!(73, 11 & m_and | m_or);
        assert_eq!(101, 101 & m_and | m_or);
        assert_eq!(64, 0 & m_and | m_or);
    }

    #[test]
    fn test_mask_value() {
        assert_eq!(73, mask_value(&MASK, 11));
        assert_eq!(101, mask_value(&MASK, 101));
        assert_eq!(64, mask_value(&MASK, 0));
    }

    #[test]
    fn test_parse_mask() {
        assert_eq!(MASK, parse_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"))
    }

    #[test]
    fn example_one() {
        assert_eq!(165, part_one(&parse_program(&EXAMPLE_ONE)));
    }

    const EXAMPLE_TWO: &str = "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn test_decode_address() {
        let mask = parse_mask("000000000000000000000000000000X1001X");
        assert_eq!(vec![26, 27, 58, 59], decode_address(&mask, 42));

        let mask = parse_mask("00000000000000000000000000000000X0XX");
        assert_eq!(
            vec![16, 17, 18, 19, 24, 25, 26, 27],
            decode_address(&mask, 26)
        );
    }

    #[test]
    fn example_two() {
        let prog = parse_program(&EXAMPLE_TWO);
        assert_eq!(208, part_two(&prog));
    }
}
