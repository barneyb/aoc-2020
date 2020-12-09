use aoc_2020::find_pairs::find_pair_with_sum;

fn main() {
    let expenses = aoc_2020::read_lines(|s| s.parse::<i64>().unwrap());
    if let Some((a, b)) = find_pair_with_sum(&expenses, 2020) {
        println!("{} * {} = {}", a, b, a * b);
    }
    for (i, &a) in expenses.iter().enumerate() {
        if let Some((b, c)) = find_pair_with_sum(&expenses[(i + 1)..], 2020 - a) {
            println!("{} * {} * {} = {}", a, b, c, a * b * c);
            break;
        }
    }
}
