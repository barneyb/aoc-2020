use aoc_2020 as aoc;

fn main() {
    let expenses = aoc::read_lines(|s| s.parse::<i32>().unwrap());
    'outer: for (i, &a) in expenses.iter().enumerate() {
        for (j, &b) in expenses.iter().skip(i + 1).enumerate() {
            let remaining = 2020 - a - b;
            if remaining <= 0 {
                continue;
            }
            for &c in expenses.iter().skip(j + 1) {
                if c == remaining {
                    println!("{} * {} * {} = {}", a, b, c, a * b * c);
                    break 'outer;
                }
            }
        }
    }
}
