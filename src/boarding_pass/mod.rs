use std::str::FromStr;

pub fn find_empty_seat(passes: Vec<BoardingPass>) -> Result<usize, &'static str> {
    let mut map = [false; 843];
    for p in passes {
        map[p.seat_id()] = true
    }
    for i in 1..841 {
        if map[i - 1] && !map[i] && map[i + 1] {
            return Ok(i);
        }
    }
    Err("No empty seat found")
}

#[derive(Eq, PartialEq)]
pub struct BoardingPass {
    row: usize,
    col: usize,
}

impl BoardingPass {
    pub fn seat_id(&self) -> usize {
        self.row * 8 + self.col
    }
}

impl FromStr for BoardingPass {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows = 0..128;
        let mut cols = (0, 7);
        for c in s.chars() {
            match c {
                'F' => rows.end = rows.start + rows.len() / 2,
                'B' => rows.start += rows.len() / 2,
                'L' => cols.1 -= (cols.1 - cols.0 + 1) / 2,
                'R' => cols.0 += (cols.1 - cols.0 + 1) / 2,
                _ => return Err(format!("Unrecognized '{}' in input!?", c)),
            }
        }
        debug_assert!(rows.len() == 1, "Pass didn't restrict to a single row");
        debug_assert!(cols.0 == cols.1, "Pass didn't restrict to a single col");
        Ok(BoardingPass {
            row: rows.start,
            col: cols.0,
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
