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
    check_shuffle(vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2], &vec![Cut(3, 10 - 3)]);
}

#[test]
fn test_cut_negative() {
    check_shuffle(vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5], &vec![Cut(10 - 4, 4)]);
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
            Cut(10 - 2, 2),
            Deal(7),
            Cut(8, 10 - 8),
            Cut(10 - 4, 4),
            Deal(7),
            Cut(3, 10 - 3),
            Deal(9),
            Deal(3),
            Cut(10 - 1, 1),
        ],
    );
}

#[test]
fn test_unshuffle() {
    check_symmetry(
        &vec![
            Reverse,
            Cut(17 - 2, 2),
            Deal(7),
            Cut(8, 17 - 8),
            Cut(17 - 4, 4),
            Deal(7),
            Cut(3, 17 - 3),
            Deal(3),
            Cut(17 - 1, 1),
        ],
        17,
    );
}

#[test]
fn test_factoring() {
    // 1755 = 13 * 5 * 3 * 3 * 3
    let deck_size = 11933;
    let iterations = 10177;
    assert_eq!(278, go_forward(2020, deck_size, deck_size - iterations - 1));
    assert_eq!(278, go_forward(2020, deck_size, 1755));
    let a = go_forward(2020, deck_size, 1);
    let b = go_forward(2021, deck_size, 1);
    println!("{} {}", a, b);
}

#[test]
fn test_cyclic_nature() {
    let deck_size = 11933;
    let iterations = 10177;

    // the part one case, going forward a single tick
    assert_eq!(2331, go_forward(2019, deck_size, 1));
    assert_eq!(2019, go_back(2331, deck_size, 1));
    assert_eq!(2331, go_back(2019, deck_size, deck_size - 1 - 1));

    // the part two case, going back a bunch of ticks
    assert_eq!(278, go_back(2020, deck_size, iterations));
    assert_eq!(2020, go_forward(278, deck_size, iterations));
    assert_eq!(278, go_forward(2020, deck_size, deck_size - iterations - 1));
}

#[test]
fn test_parts() {
    assert_eq!(3036, go_forward(2019, 10007, 1));
    assert_eq!(2019, go_back(3036, 10007, 1));

    let iterations = 173; // a prime!
    let r = go_forward(2020, DECK_SIZE, iterations);
    assert_eq!(2020, go_back(r, DECK_SIZE, iterations));
}
