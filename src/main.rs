use aoc_2020::read_lines;

fn main() {
    let expenses = read_lines(|s| s.parse::<i32>().unwrap());
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
