use aoc::boarding_pass::{find_empty_seat, BoardingPass};
use aoc_2020 as aoc;

pub fn the_work() {
    let passes = aoc::read_lines(|s| s.parse::<BoardingPass>().unwrap());
    let part_one = passes.iter().map(|p| p.seat_id()).max().unwrap();
    println!("{}", part_one);
    if let Ok(id) = find_empty_seat(passes) {
        println!("{}", id);
    }
}
