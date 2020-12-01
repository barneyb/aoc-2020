use aoc_2020::read_input;

fn main() {
    let expenses = read_input()
        .lines()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();
    'outer: for a in &expenses {
        for b in &expenses {
            if a + b == 2020 {
                println!("{} * {} = {}", a, b, a * b);
                break 'outer;
            }
        }
    }
}
