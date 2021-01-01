use super::*;

fn check_shuffle(expected: Vec<usize>, ops: &Vec<Op>) {
    let in_order = (0..expected.len()).collect::<Vec<_>>();
    let mut shuffled = Vec::with_capacity(expected.len());
    shuffled.resize(expected.len(), 42);
    for &c in &in_order {
        shuffled[shuffle(ops, c, expected.len())] = c;
    }
    assert_eq!(expected, shuffled);
}

#[test]
fn test_reverse() {
    check_shuffle(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0], &vec![Reverse]);
}

#[test]
fn test_cut_positive() {
    check_shuffle(vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2], &vec![Cut(3)]);
}

#[test]
fn test_cut_negative() {
    check_shuffle(vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5], &vec![Uncut(4)]);
}

#[test]
fn test_deal() {
    let ds = 7;
    let ops = vec![Deal(3)];
    check_shuffle(vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3], &ops);
    check_symmetry(&ops, ds)
}

fn check_symmetry(ops: &[Op], ds: usize) {
    let unops = reverse_operations(&ops, ds);
    for card in 0..ds {
        let s = shuffle(&ops, card, ds);
        let us = shuffle(&unops, s, ds);
        assert_eq!(
            card, us,
            "{} shuffled to {}, but unshuffled to {}",
            card, s, us
        );
    }
}

#[test]
fn example_four() {
    check_shuffle(
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

#[test]
fn test_unshuffle() {
    check_symmetry(
        &vec![
            Reverse,
            Uncut(2),
            Deal(7),
            Cut(8),
            Uncut(4),
            Deal(7),
            Cut(3),
            Deal(3),
            Uncut(1),
        ],
        17,
    );
}

#[test]
fn test_reciprocity() {
    let deck_size = 11933;
    let iterations = 10177;
    let r = part_one_n(2020, deck_size, iterations);
    assert_eq!(2020, part_two(r, deck_size, iterations));
    assert_eq!(2020, part_one_n(r, deck_size, deck_size - iterations - 1));
}

#[test]
fn test_parts() {
    assert_eq!(3036, part_one(2019, 10007));
    assert_eq!(2019, part_two(3036, 10007, 1));

    let iterations = 173; // a prime!
    let r = part_one_n(2020, DECK_SIZE, iterations);
    assert_eq!(2020, part_two(r, DECK_SIZE, iterations));
}
