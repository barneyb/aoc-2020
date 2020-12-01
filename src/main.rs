use aoc_2020::read_input;

fn main() {
    let expenses = read_input()
        .lines()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();
    'outer: for a in &expenses {
        for b in &expenses {
            for c in &expenses {
                if a + b + c == 2020 {
                    println!("{} * {} * {} = {}", a, b, c, a * b * c);
                    break 'outer;
                }
            }
        }
    }
}
