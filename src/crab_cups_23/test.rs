use super::*;
use crate::with_duration;

#[test]
fn test_parse_and_unparse() {
    let cups = Cups::from("3412");
    assert_eq!(0, cups.moves);
    assert_eq!(vec![1usize, 2, 3, 4], cups.one_first());
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
fn bench() {
    let tick_count = 10_000;

    let mut cups = Cups::from(EXAMPLE_ONE);
    cups.extend_to(20);

    let (_, run) = with_duration(|| {
        for _ in 0..tick_count {
            cups.tick();
        }
    });
    // this is for 10_000 ticks w/ 20 cups
    let rotated = cups.one_first();
    assert_eq!(vec![1, 19, 15, 5, 2, 13, 4, 12, 9, 20, 10, 8, 11, 16, 3, 17, 6, 7, 18, 14], rotated);

    let expected_run = run.as_millis() * 1_000_000 * 10_000 / cups.size as u128 / tick_count;
    println!("expect {}s to solve part two", expected_run);
}

#[test]
fn example_one() {
    let mut cups = Cups::from(EXAMPLE_ONE);
    assert_eq!("67384529", part_one(&mut cups));
}
