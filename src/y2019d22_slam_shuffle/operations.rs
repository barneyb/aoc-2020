use crate::math::mult_inv;
use std::iter::Map;
use std::str::{FromStr, Lines};
use Op::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Op {
    Reverse(usize),
    Cut(usize, usize),
    Deal(usize, usize),
}

pub fn parse_operation_list(s: &str) -> Vec<Op> {
    __str_to_op_iter(s).collect()
}

#[allow(dead_code)]
pub fn parse_bound_operation_list(s: &str, deck_size: usize) -> Vec<Op> {
    __str_to_op_iter(s)
        .map(|op| op.for_deck_size(deck_size))
        .collect()
}

pub fn bind_operation_list(ops: &[Op], deck_size: usize) -> Vec<Op> {
    ops.iter().map(|op| op.for_deck_size(deck_size)).collect()
}

fn __str_to_op_iter(s: &str) -> Map<Lines, fn(&str) -> Op> {
    s.trim().lines().map(|l| l.parse::<Op>().unwrap())
}

pub fn reverse_operations(ops: &[Op]) -> Vec<Op> {
    ops.iter()
        .rev()
        .map(|op| match *op {
            Reverse(n) => Reverse(n),
            Cut(n, u) => Cut(u, n),
            Deal(n, s) => Deal(mult_inv(n, s), s),
        })
        .collect::<Vec<_>>()
}

impl Op {
    pub fn for_deck_size(&self, deck_size: usize) -> Op {
        match self {
            Reverse(0) => Reverse(deck_size - 1),
            Cut(0, u) => Cut(deck_size - *u, *u),
            Cut(n, 0) => Cut(*n, deck_size - *n),
            Deal(n, 0) => Deal(*n, deck_size),
            it => panic!("Already-bound {:?} found?!", it),
        }
    }
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        /*
        deal with increment 26
        redeal 26
        deal into new stack
        reverse
        cut -2168
         */
        let mut words = s.trim().split(' ');
        Ok(match words.next().unwrap() {
            "deal" => match words.next().unwrap() {
                "with" => Deal(words.nth(1).unwrap().parse::<usize>().unwrap(), 0),
                "into" => Reverse(0),
                w => panic!("Unrecognized 'deal {}' op", w),
            },
            "redeal" => Deal(words.next().unwrap().parse::<usize>().unwrap(), 0),
            "reverse" => Reverse(0),
            "cut" => {
                let i = words.next().unwrap().parse::<isize>().unwrap();
                if i < 0 {
                    Cut(0, (i * -1) as usize)
                } else {
                    Cut(i as usize, 0)
                }
            }
            w => panic!("Unrecognized '{}' op", w),
        })
    }
}
