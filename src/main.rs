use aoc_2020 as aoc;

mod the_work;

fn main() {
    console::set_colors_enabled(true);
    let (_, elapsed) = aoc::with_duration(the_work::the_work);
    let success = console::Style::new().bold().green();
    println!("\n{:>12} {:?}", success.apply_to("Finished"), elapsed);
}
