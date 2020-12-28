use aoc_2020::{read_input, time_block};
use std::collections::VecDeque;

pub fn the_work() {
    let game = time_block("Deal", || deal(&read_input()));
    println!("{:?}", time_block("Part One", || part_one(game)));
}

type Card = usize;
type Deck = VecDeque<Card>;
type Game = [Deck; 2];

fn part_one([mut one, mut two]: Game) -> usize {
    // while both players have cards...
    while !one.is_empty() && !two.is_empty() {
        // each flips their top card
        let a = one.pop_front().unwrap();
        let b = two.pop_front().unwrap();
        // the player with the larger card gets both (theirs first)
        if a > b {
            one.push_back(a);
            one.push_back(b);
        } else {
            two.push_back(b);
            two.push_back(a);
        }
    }
    // return the winner's score
    compute_score(&if one.is_empty() { two } else { one })
}

fn compute_score(deck: &Deck) -> usize {
    deck.iter()
        .fold((0, deck.len()), |(score, offset), card| {
            (score + card * offset, offset - 1)
        })
        .0
}

fn deal(input: &str) -> Game {
    let mut players = input.trim().split("\n\n").map(|player| {
        player
            .trim()
            .lines()
            .skip(1)
            .map(|c| c.parse::<Card>().unwrap())
            .collect()
    });
    let game = [players.next().unwrap(), players.next().unwrap()];
    debug_assert!(players.next().is_none());
    game
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_ONE: &str = "\
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test]
    fn test_parse() {
        let [a, b] = deal(&EXAMPLE_ONE);
        assert_eq!(vec![9, 2, 6, 3, 1], a.into_iter().collect::<Vec<_>>());
        assert_eq!(vec![5, 8, 4, 7, 10], b.into_iter().collect::<Vec<_>>());
    }

    #[test]
    fn test_compute_score() {
        let deck: Deck = vec![3, 2, 10, 6, 8, 5, 9, 4, 7, 1].into_iter().collect();
        assert_eq!(306, compute_score(&deck));
    }

    #[test]
    fn example_one() {
        let game = deal(&EXAMPLE_ONE);
        assert_eq!(306, part_one(game));
    }
}
