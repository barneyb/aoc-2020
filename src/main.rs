use aoc_2020 as aoc;

mod the_work;

fn main() {
    let (_, elapsed) = aoc::with_duration(the_work::the_work);
    console::set_colors_enabled(true);
    let success = console::Style::new().bold().green();
    println!("{:>12} {:?}", success.apply_to("Finished"), elapsed);
}
