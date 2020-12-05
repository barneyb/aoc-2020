use aoc::boarding_pass::{find_empty_seat, BoardingPass};
use aoc_2020 as aoc;

fn the_work() {
    let passes = aoc::read_lines(|s| s.parse::<BoardingPass>().unwrap());
    let part_one = passes.iter().map(|p| p.seat_id()).max().unwrap();
    println!("{}", part_one);
    if let Ok(id) = find_empty_seat(passes) {
        println!("{}", id);
    }
}

fn main() {
    let (_, elapsed) = aoc::with_duration(the_work);
    console::set_colors_enabled(true);
    let success = console::Style::new().bold().green();
    println!("{:>12} {:?}", success.apply_to("Finished"), elapsed);
}
