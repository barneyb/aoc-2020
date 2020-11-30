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
            .enumerate()
            .fold((0, None as Option<usize>), |(mut floor, pos), (i, d)| {
                if pos.is_none() {
                    floor += d;
                    if floor == -1 {
                        return (floor, Some(i + 1));
                    }
                }
                (floor, pos)
            })
            .1
            .unwrap()
    );
}

fn read_input() -> String {
    fs::read_to_string("input.txt").unwrap().trim().to_string()
}
