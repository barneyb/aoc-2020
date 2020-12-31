use super::*;

const EXAMPLE_ONE: &str = "5764801
17807724";

#[test]
fn test_crack() {
    assert_eq!(8, crack_loop_size(7, 5764801));
    assert_eq!(11, crack_loop_size(7, 17807724));
}

#[test]
fn test_encrypt() {
    assert_eq!(14897079, encrypt(17807724, 8));
    assert_eq!(14897079, encrypt(5764801, 11));
}

#[test]
fn example_one() {
    assert_eq!(14897079, part_one(&EXAMPLE_ONE));
}
