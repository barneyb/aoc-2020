use super::operations::{bind_operation_list, parse_bound_operation_list, parse_operation_list};
use super::*;
use crate::read_input;

fn check_shuffle(expected: &Vec<usize>, ops: &Vec<Op>) {
    let deck_size = expected.len();
    let in_order = (0..deck_size).collect::<Vec<_>>();
    let mut shuffled = Vec::with_capacity(deck_size);
    shuffled.resize(deck_size, 42);
    for &c in &in_order {
        shuffled[shuffle(c, ops, deck_size, 1)] = c;
    }
    assert_eq!(expected, &shuffled);
}

#[test]
fn test_reverse() {
    check_shuffle(
        &vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
        &vec![Reverse(0).for_deck_size(10)],
    );
}

#[test]
fn test_cut_positive() {
    check_shuffle(
        &vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2],
        &vec![Cut(3, 0).for_deck_size(10)],
    );
}

#[test]
fn test_cut_negative() {
    check_shuffle(
        &vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5],
        &vec![Cut(0, 4).for_deck_size(10)],
    );
}

#[test]
fn test_deal() {
    check_shuffle(
        &vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3],
        &vec![Deal(3, 0).for_deck_size(10)],
    );

    let ds = 7;
    let ops = vec![Deal(3, 0).for_deck_size(ds)];
    check_symmetry(&ops, ds)
}

fn check_symmetry(ops: &[Op], ds: usize) {
    let unops = reverse_operations(&ops);
    for card in 0..ds {
        let s = shuffle(card, &ops, ds, 1);
        let us = shuffle(s, &unops, ds, 1);
        assert_eq!(
            card, us,
            "{} shuffled to {}, but unshuffled to {}",
            card, s, us
        );
    }
}

#[test]
fn test_unshuffle() {
    check_symmetry(
        &parse_bound_operation_list(
            "\
        reverse
        cut -2
        deal 7
        cut 8
        cut -4
        deal 7
        cut 3
        deal 3
        cut -1
    ",
            17,
        ),
        17,
    );
}

const EXAMPLE_FOUR: &str = "\
deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1";

const EXAMPLE_FOUR_ALTERNATE: &str = "\
reverse
cut -2
deal 7
cut 8
cut -4
deal 7
cut 3
deal 9
deal 3
cut -1";

#[test]
fn example_four() {
    let raw_ops = parse_operation_list(&EXAMPLE_FOUR);
    assert_eq!(raw_ops, parse_operation_list(&EXAMPLE_FOUR_ALTERNATE));
    let bound_ops = bind_operation_list(&raw_ops, 10);
    let ops = parse_bound_operation_list(&EXAMPLE_FOUR, 10);
    assert_eq!(bound_ops, ops);
    check_shuffle(&vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6], &ops);
}

#[test]
fn test_cyclic_nature() {
    let deck_size = 11933;
    let iterations = 10177;
    let ops = parse_bound_operation_list(&read_input(), deck_size);
    let unops = reverse_operations(&ops);

    // the part one case, going forward a single tick
    assert_eq!(2331, shuffle(2019, &ops, deck_size, 1));
    assert_eq!(2019, shuffle(2331, &unops, deck_size, 1));
    assert_eq!(2331, shuffle(2019, &unops, deck_size, deck_size - 1 - 1));

    // the part two case, going back a bunch of ticks
    assert_eq!(278, shuffle(2020, &unops, deck_size, iterations));
    assert_eq!(2020, shuffle(278, &ops, deck_size, iterations));
    assert_eq!(
        278,
        shuffle(2020, &ops, deck_size, deck_size - iterations - 1)
    );
}

#[test]
fn do_benchmark() {
    let ops = parse_bound_operation_list(&read_input(), DECK_SIZE);
    let ans = bench(
        "[Debug] Bench Part Two",
        &ops,
        DECK_SIZE - ITERATIONS - 1,
        100_000_000,
        shuffle,
    );
    assert_eq!(10531478815607, ans);
}

#[test]
fn test_parts() {
    let raw_ops = parse_operation_list(&read_input());
    let ops = bind_operation_list(&raw_ops, 10007);
    let unops = reverse_operations(&ops);
    assert_eq!(3036, shuffle(2019, &ops, 10007, 1));
    assert_eq!(2019, shuffle(3036, &unops, 10007, 1));

    let iterations = 173; // a prime!
    let ops = bind_operation_list(&raw_ops, DECK_SIZE);
    let unops = reverse_operations(&ops);
    assert_eq!(81897533950870, shuffle(2020, &ops, DECK_SIZE, iterations));
    assert_eq!(2020, shuffle(81897533950870, &unops, DECK_SIZE, iterations));
}
