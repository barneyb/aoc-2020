use super::*;

fn check_ops(expected: Vec<usize>, ops: &Vec<Op>) {
    let mut actual = Vec::with_capacity(expected.len());
    actual.resize(expected.len(), 0);
    for c in 0..expected.len() {
        actual[shuffle(ops, c, expected.len())] = c;
    }
    assert_eq!(expected, actual);
}

#[test]
fn test_reverse() {
    check_ops(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0], &vec![Reverse])
}

#[test]
fn test_cut_positive() {
    check_ops(vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2], &vec![Cut(3)])
}

#[test]
fn test_cut_negative() {
    check_ops(vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5], &vec![Uncut(4)])
}

#[test]
fn test_deal() {
    check_ops(vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3], &vec![Deal(3)])
}

#[test]
fn example_four() {
    check_ops(
        vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6],
        &vec![
            Reverse,
            Uncut(2),
            Deal(7),
            Cut(8),
            Uncut(4),
            Deal(7),
            Cut(3),
            Deal(9),
            Deal(3),
            Uncut(1),
        ],
    );
}
