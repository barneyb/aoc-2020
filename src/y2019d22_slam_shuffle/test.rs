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

fn prime_factorization(number: usize) -> Option<Vec<usize>> {
    let mut result = Vec::new();
    if number <= 3 {
        return None;
    }
    let mut n = number;
    let sqrt = (n as f64).sqrt() as usize;
    for f in 2..=sqrt {
        while n % f == 0 {
            result.push(f);
            n /= f;
            if n == 1 {
                return Some(result);
            }
        }
    }
    if n == number {
        return None;
    }
    result.push(n);
    Some(result)
}

#[test]
fn test_prime_factorization() {
    assert_eq!(None, prime_factorization(0));
    assert_eq!(None, prime_factorization(1));
    assert_eq!(None, prime_factorization(2));
    assert_eq!(None, prime_factorization(3));
    assert_eq!(Some(vec![2, 2]), prime_factorization(4));
    assert_eq!(None, prime_factorization(17));
    assert_eq!(Some(vec![2, 2, 5]), prime_factorization(20));
    assert_eq!(Some(vec![2, 2, 2, 3]), prime_factorization(24));
    assert_eq!(Some(vec![5, 5]), prime_factorization(25));
    assert_eq!(Some(vec![2, 13]), prime_factorization(26));
    assert_eq!(Some(vec![2, 5, 5]), prime_factorization(50));
    assert_eq!(Some(vec![3, 3, 3, 5, 13]), prime_factorization(1755));
    assert_eq!(
        Some(vec![3, 3, 5, 87887, 4443619]),
        prime_factorization(17574135437385)
    );
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
