use super::*;

#[test]
fn test_parse_and_unparse() {
    let cups = Cups::from("3412");
    assert_eq!(0, cups.moves);
    assert_eq!(VecDeque::from(vec![2usize, 3, 0, 1]), cups.ring);
    assert_eq!("(3) 4  1  2 ", format!("{:?}", cups));
    assert_eq!("234", cups.to_string());
}

const EXAMPLE_ONE: &str = "389125467";

#[test]
fn test_single_moves() {
    let mut cups = Cups::from(EXAMPLE_ONE);
    assert_eq!("(3) 8  9  1  2  5  4  6  7 ", format!("{:?}", cups));
    cups.tick();
    assert_eq!(" 3 (2) 8  9  1  5  4  6  7 ", format!("{:?}", cups));
    cups.tick();
    assert_eq!(" 3  2 (5) 4  6  7  8  9  1 ", format!("{:?}", cups));
    cups.tick();
    assert_eq!(" 7  2  5 (8) 9  1  3  4  6 ", format!("{:?}", cups));
    cups.tick();
    assert_eq!(" 3  2  5  8 (4) 6  7  9  1 ", format!("{:?}", cups));
    cups.tick();
    assert_eq!(" 9  2  5  8  4 (1) 3  6  7 ", format!("{:?}", cups));
    cups.tick();
    assert_eq!(" 7  2  5  8  4  1 (9) 3  6 ", format!("{:?}", cups));
    cups.tick();
    assert_eq!(" 8  3  6  7  4  1  9 (2) 5 ", format!("{:?}", cups));
    cups.tick();
    assert_eq!(" 7  4  1  5  8  3  9  2 (6)", format!("{:?}", cups));
    cups.tick();
    assert_eq!("(5) 7  4  1  8  3  9  2  6 ", format!("{:?}", cups));
    cups.tick();
    assert_eq!(" 5 (8) 3  7  4  1  9  2  6 ", format!("{:?}", cups));
    assert_eq!("92658374", cups.to_string())
}

#[test]
fn example_one() {
    let ring = Cups::from(EXAMPLE_ONE);
    assert_eq!("67384529", part_one(&ring));
}
