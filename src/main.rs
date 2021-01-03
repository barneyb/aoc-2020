use aoc_2020::day25_combo_breaker::solve;
use aoc_2020::{read_input, with_duration};

fn main() {
    let (_, elapsed) = with_duration(|| solve(&read_input()));
    let success = console::Style::new().bold().green();
    println!("\n{:>12} {:?}", success.apply_to("Finished"), elapsed);
}
