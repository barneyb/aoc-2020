use crate::timed_block;

#[cfg(test)]
mod test;

pub fn solve(input: &str) {
    println!("{}", timed_block("Part One", || part_one(input)));
}

fn part_one(input: &str) -> usize {
    input.len()
}
