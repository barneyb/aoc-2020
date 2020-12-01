use aoc_2020::read_input;

fn main() {
    let expenses = read_input()
        .lines()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();
    'outer: for (i, a) in expenses.iter().enumerate() {
        for (j, b) in expenses.iter().skip(i + 1).enumerate() {
            for c in expenses.iter().skip(j + 1) {
                if a + b + c == 2020 {
                    println!("{} * {} * {} = {}", a, b, c, a * b * c);
                    break 'outer;
                }
            }
        }
    }
}
