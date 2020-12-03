use aoc_2020 as aoc;

fn main() {
    let tree_count = aoc::read_input()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars().nth((i * 3) % line.len()).unwrap()
        })
        .filter(|&c| c == '#')
        .count();
    println!("{}", tree_count);
}
