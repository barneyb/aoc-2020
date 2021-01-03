use crate::timed_block;

#[cfg(test)]
mod test;

pub fn solve(input: &str) {
    println!("{}", timed_block("Part One", || part_one(input)));
}

const MODULUS: usize = 20201227;

fn part_one(input: &str) -> usize {
    let public_keys = input
        .trim()
        .lines()
        .map(|l| l.trim().parse::<usize>().unwrap())
        .take(2)
        .collect::<Vec<_>>();
    let card_loop_size = crack_loop_size(7, public_keys[0]);
    encrypt(public_keys[1], card_loop_size)
}

fn encrypt(subject_num: usize, loop_size: usize) -> usize {
    let mut v = 1;
    for _ in 0..loop_size {
        v *= subject_num;
        v %= MODULUS;
    }
    v
}

fn crack_loop_size(subject_num: usize, result: usize) -> usize {
    let mut v = 1;
    let mut iteration = 0;
    loop {
        v *= subject_num;
        v %= MODULUS;
        iteration += 1;
        if v == result {
            return iteration;
        }
    }
}
