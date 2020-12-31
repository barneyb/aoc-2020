use super::*;

const EXAMPLE_ONE: &str = "389125467";

#[test]
fn test_parse_and_unparse() {
    let cups = Cups::from("3412");
    assert_eq!(0, cups.moves);
    assert_eq!(vec![1usize, 2, 3, 4], cups.one_first());
    assert_eq!("234", cups.to_string());

    let cups = Cups::from(EXAMPLE_ONE);
    assert_eq!(vec![0, 2, 5, 8, 6, 4, 7, 3, 9, 1], cups.arr);
    assert_eq!(vec![1, 2, 5, 4, 6, 7, 3, 8, 9], cups.one_first());

    let mut cups = Cups::from(EXAMPLE_ONE);
    cups.extend_to(20);
    assert_eq!(
        vec![1, 2, 5, 4, 6, 7, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 3, 8, 9],
        cups.one_first()
    );
    assert_eq!("254671011121314151617181920389", cups.to_string());
    assert_eq!((2, 5), cups.pair_after_one());
}

#[test]
fn test_single_moves() {
    let mut cups = Cups::from(EXAMPLE_ONE);
    assert_eq!("25467389", cups.to_string());
    cups.tick();
    assert_eq!("54673289", cups.to_string());
    cups.tick();
    assert_eq!("32546789", cups.to_string());
    cups.tick();
    assert_eq!("34672589", cups.to_string());
    cups.tick();
    assert_eq!("32584679", cups.to_string());
    cups.tick();
    assert_eq!("36792584", cups.to_string());
    cups.tick();
    assert_eq!("93672584", cups.to_string());
    cups.tick();
    assert_eq!("92583674", cups.to_string());
    cups.tick();
    assert_eq!("58392674", cups.to_string());
    cups.tick();
    assert_eq!("83926574", cups.to_string());
    cups.tick();
    assert_eq!("92658374", cups.to_string());
}

#[test]
fn extended_list_sanity() {
    let mut cups = Cups::from(EXAMPLE_ONE);
    cups.extend_to(20);
    cups.tick();
    assert_eq!(
        vec![1, 5, 4, 6, 7, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 3, 2, 8, 9],
        cups.one_first()
    );
    assert_eq!("546710111213141516171819203289", cups.to_string());
    assert_eq!((5, 4), cups.pair_after_one());
    for _ in 0..100 {
        cups.tick();
    }
    assert_eq!(
        vec![1, 9, 6, 13, 3, 12, 8, 17, 10, 14, 20, 18, 5, 19, 2, 4, 7, 16, 11, 15],
        cups.one_first()
    );
    assert_eq!("961331281710142018519247161115", cups.to_string());
    assert_eq!((9, 6), cups.pair_after_one());
}

#[test]
fn example_one_part_one() {
    assert_eq!("67384529", part_one(&EXAMPLE_ONE));
}

#[test]
fn example_one_part_two() {
    assert_eq!(149245887792, part_two(&EXAMPLE_ONE));
}
