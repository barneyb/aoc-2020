use aoc_2020 as aoc;

use aoc::boarding_pass::BoardingPass;

fn main() {
    let passes = aoc::read_lines(|s| s.parse::<BoardingPass>().unwrap());
    let part_one = passes.iter().map(|p| p.seat_id()).max().unwrap();
    println!("{}", part_one);
    let mut map = [false; 843];
    for p in passes {
        map[p.seat_id()] = true
    }
    for i in 1..841 {
        if map[i - 1] && !map[i] && map[i + 1] {
            println!("{}", i)
        }
    }
}
