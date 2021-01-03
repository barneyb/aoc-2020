use super::*;

fn check_shuffle(expected: &Vec<usize>, op: &Op) {
    let in_order = (0..expected.len()).collect::<Vec<_>>();
    let mut shuffled = Vec::with_capacity(expected.len());
    shuffled.resize(expected.len(), 42);
    for &c in &in_order {
        shuffled[op.execute(c)] = c;
    }
    assert_eq!(expected, &shuffled);
}

#[test]
fn test_reverse() {
    check_shuffle(
        &vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
        &Reverse(0).for_deck_size(10),
    );
}

#[test]
fn test_cut_positive() {
    check_shuffle(
        &vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2],
        &Cut(3, 0).for_deck_size(10),
    );
}

#[test]
fn test_cut_negative() {
    check_shuffle(
        &vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5],
        &Cut(0, 4).for_deck_size(10),
    );
}

#[test]
fn test_deal() {
    check_shuffle(
        &vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3],
        &Deal(3, 0).for_deck_size(10),
    );

    let ds = 7;
    let ops = vec![Deal(3, 0).for_deck_size(ds)];
    check_symmetry(&ops, ds)
}

fn check_symmetry(ops: &[Op], ds: usize) {
    let unops = reverse_operations(&ops);
    for card in 0..ds {
        let s = shuffle(card, &ops);
        let us = shuffle(s, &unops);
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
