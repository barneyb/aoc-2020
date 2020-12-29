use aoc_2020::{read_input, time_block};
use std::collections::{HashSet, VecDeque};

pub fn the_work() {
    let game = time_block("Deal", || deal(&read_input()));
    println!("{:?}", time_block("Part One", || part_one(game)));
    let game = time_block("Deal Again", || deal(&read_input()));
    println!("{:?}", time_block("Part Two", || part_two(game)));
}

type Card = usize;
type Deck = VecDeque<Card>;
type Game = [Deck; 2];
enum Player {
    PlayerOne,
    PlayerTwo,
}
use Player::*;

fn part_one([mut one, mut two]: Game) -> usize {
    loop {
        // each flips their top card
        let a = one.pop_front().unwrap();
        let b = two.pop_front().unwrap();
        // the player with the larger card gets both (theirs first)
        if a > b {
            one.push_back(a);
            one.push_back(b);
            if two.is_empty() {
                // return the winner's score
                return compute_score(&one);
            }
        } else {
            two.push_back(b);
            two.push_back(a);
            if one.is_empty() {
                // return the winner's score
                return compute_score(&two);
            }
        }
    }
}

fn part_two(mut game: Game) -> usize {
    let winner = play_recursive(&mut game);
    compute_score(&game[winner as usize])
}

fn play_recursive([one, two]: &mut Game) -> Player {
    let mut seen_states = HashSet::new();
    loop {
        // if it's a repeated state player one wins the game
        if !seen_states.insert((
            one.iter().cloned().collect::<Vec<_>>(),
            two.iter().cloned().collect::<Vec<_>>(),
        )) {
            return PlayerOne;
        }
        // each flips their top card
        let a = one.pop_front().unwrap();
        let b = two.pop_front().unwrap();
        let round_winner = if one.len() >= a && two.len() >= b {
            // make new decks of first n cards of each deck
            // play a recursive game; winner wins the round
            let mut subgame = [
                one.iter().take(a).cloned().collect::<Deck>(),
                two.iter().take(b).cloned().collect::<Deck>(),
            ];
            play_recursive(&mut subgame)
        } else if a > b {
            PlayerOne
        } else {
            PlayerTwo
        };
        // the round's winner gets both cards (theirs first)
        match round_winner {
            PlayerOne => {
                one.push_back(a);
                one.push_back(b);
                if two.is_empty() {
                    return PlayerOne;
                }
            }
            PlayerTwo => {
                two.push_back(b);
                two.push_back(a);
                if one.is_empty() {
                    return PlayerTwo;
                }
            }
        }
    }
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
    fn example_one_part_one() {
        let game = deal(&EXAMPLE_ONE);
        assert_eq!(306, part_one(game));
    }

    const EXAMPLE_TWO: &str = "\
Player 1:
43
19

Player 2:
2
29
14";

    #[test]
    fn example_two() {
        let game = deal(&EXAMPLE_TWO);
        let score = compute_score(&game[PlayerOne as usize]);
        assert_eq!(score, part_two(game));
    }

    #[test]
    fn example_one_part_two() {
        let game = deal(&EXAMPLE_ONE);
        assert_eq!(291, part_two(game));
    }
}
