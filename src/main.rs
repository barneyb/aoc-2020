use aoc_2020 as aoc;

fn main() {
    let input = aoc::read_input();
    let mut total = 1;
    let one_one = tree_count(&input, 1, 1);
    total *= one_one;
    println!("{} => {}", one_one, total);
    let three_one = tree_count(&input, 3, 1);
    total *= three_one;
    println!("{} => {}", three_one, total);
    let five_one = tree_count(&input, 5, 1);
    total *= five_one;
    println!("{} => {}", five_one, total);
    let seven_one = tree_count(&input, 7, 1);
    total *= seven_one;
    println!("{} => {}", seven_one, total);
    let one_two = tree_count(&input, 1, 2);
    total *= one_two;
    println!("{} => {}", one_two, total);
}

fn tree_count(input: &String, right: usize, down: usize) -> usize {
    input
        .lines()
        .enumerate()
        .filter(|(i, _)| i % down == 0)
        .map(|(_, line)| line)
        .enumerate()
        .map(|(i, line)| line.chars().nth((i * right) % line.len()).unwrap())
        .filter(|&c| c == '#')
        .count()
}

