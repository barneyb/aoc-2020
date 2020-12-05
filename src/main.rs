use aoc_2020 as aoc;
use std::str::FromStr;

fn main() {
    let passes = aoc::read_lines(|s| s.parse::<BoardingPass>().unwrap());
    let part_one = passes
        .iter()
        .map(|p| p.seat_id())
        .max()
        .unwrap();
    println!("{}", part_one);
    let mut map = [false; 843];
    for p in passes {
        map[p.seat_id()] = true
    }
    for i in 1..841 {
        if map[i-1] && !map[i] && map[i+1] {
            println!("{}", i)
        }
    }
}

#[derive(Eq, PartialEq)]
struct BoardingPass {
    row: usize,
    col: usize,
}

impl BoardingPass {
    fn seat_id(&self) -> usize {
        self.row * 8 + self.col
    }
}

impl FromStr for BoardingPass {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut row = (0, 127);
        let mut col = (0, 7);
        for c in s.chars() {
            match c {
                'F' => row.1 -= (row.1 - row.0 + 1) / 2,
                'B' => row.0 += (row.1 - row.0 + 1) / 2,
                'L' => col.1 -= (col.1 - col.0 + 1) / 2,
                'R' => col.0 += (col.1 - col.0 + 1) / 2,
                _ => panic!("Unrecognized '{}' in input!?", c),
            }
        }
        assert_eq!(row.0, row.1);
        assert_eq!(col.0, col.1);
        Ok(BoardingPass {
            row: row.0,
            col: col.0,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_inputs() {
        let p = "FBFBBFFRLR".parse::<BoardingPass>().unwrap();
        assert_eq!(44, p.row);
        assert_eq!(5, p.col);
        assert_eq!(357, p.seat_id());
        let p = "BFFFBBFRRR".parse::<BoardingPass>().unwrap();
        assert_eq!(70, p.row);
        assert_eq!(7, p.col);
        assert_eq!(567, p.seat_id());
        let p = "FFFBBBFRRR".parse::<BoardingPass>().unwrap();
        assert_eq!(14, p.row);
        assert_eq!(7, p.col);
        assert_eq!(119, p.seat_id());
        let p = "BBFFBBFRLL".parse::<BoardingPass>().unwrap();
        assert_eq!(102, p.row);
        assert_eq!(4, p.col);
        assert_eq!(820, p.seat_id());
    }
}
