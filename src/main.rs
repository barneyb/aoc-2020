use aoc_2020 as aoc;

use aoc::password::Record;

fn main() {
    let database = aoc::read_lines(|s| s.parse::<Record>().unwrap());
    let num_valid = database.iter().filter(|r| r.is_valid()).count();
    println!("{}", num_valid)
}
