use crate::math::{mult_inv, mult_mod};
use std::iter::Map;
use std::num::ParseIntError;
use std::str::{FromStr, Lines};
use Op::*;

#[cfg(test)]
mod test;

pub fn shuffle(card: usize, ops: &[Op]) -> usize {
    ops.iter().fold(card, |c, op| op.execute(c))
}

#[derive(Debug, Eq, PartialEq)]
pub enum Op {
    Reverse(usize),
    Cut(usize, usize),
    Deal(usize, usize),
}

impl Op {
    pub fn execute(&self, c: usize) -> usize {
        match self {
            Reverse(sm1) => sm1 - c,
            Cut(n, u) => {
                if c < *n {
                    c + u
                } else {
                    c - n
                }
            }
            // Deal(n) => c * n % deck_size,
            Deal(n, s) => mult_mod(c, *n, *s),
        }
    }
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
    s.trim().lines().map(|l| l.trim().parse::<Op>().unwrap())
}

pub fn reverse_operations(ops: &[Op]) -> Vec<Op> {
    ops.iter()
        .rev()
        .map(|op| match *op {
            Reverse(n) => Reverse(n),
            Cut(n, u) => Cut(u, n),
            Deal(_, 0) => panic!("Unbound {:?} found?!", op),
            Deal(n, s) => Deal(mult_inv(n, s).unwrap(), s),
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
    type Err = ParseOpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ParseOpError::*;
        /*
        deal with increment 26
        deal 26
        deal into new stack
        reverse
        cut -2168
         */
        let mut words = s.trim().split(' ');
        match words.next().unwrap() {
            "deal" => match words.next().unwrap() {
                "with" => {
                    let w = words.nth(1).unwrap();
                    match w.parse() {
                        Ok(n) => Ok(Deal(n, 0)),
                        Err(e) => Err(BadDealWithIncrement(e)),
                    }
                }
                "into" => Ok(Reverse(0)),
                w => match w.parse() {
                    Ok(n) => Ok(Deal(n, 0)),
                    Err(e) => Err(BadDeal(e)),
                },
            },
            "reverse" => Ok(Reverse(0)),
            "cut" => {
                let w = words.next().unwrap();
                match w.parse::<i32>() {
                    Ok(n) if n > 0 => Ok(Cut(n as usize, 0)),
                    Ok(n) => Ok(Cut(0, n.abs() as usize)),
                    Err(e) => Err(BadCut(e)),
                }
            }
            _ => Err(Unrecognized),
        }
    }
}

#[derive(Debug)]
pub enum ParseOpError {
    Unrecognized,
    BadCut(ParseIntError),
    BadDeal(ParseIntError),
    BadDealWithIncrement(ParseIntError),
}
