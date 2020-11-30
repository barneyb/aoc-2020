use std::fs;

fn main() {
    println!(
        "{:?}",
        read_input()
            .chars()
            .map(|c| match c {
                '(' => 1,
                ')' => -1,
                _ => panic!("Unrecognized char '{}'", c),
            })
            .sum::<i32>()
    );
}

fn read_input() -> String {
    fs::read_to_string("input.txt").unwrap().trim().to_string()
}
