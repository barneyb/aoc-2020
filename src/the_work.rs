use std::collections::HashMap;

pub fn the_work() {
    let numbers = vec![0, 14, 6, 20, 1, 4];
    println!("{:?}", play_game(2020, &numbers));
    println!("{:?}", play_game(30_000_000, &numbers));
}

struct Game {
    history: HashMap<usize, usize>,
    last: usize,
    turns: usize,
}

impl Game {
    fn new() -> Game {
        Game {
            history: HashMap::new(),
            last: 0,
            turns: 0,
        }
    }

    fn say(&mut self, n: usize) {
        if self.turns > 0 {
            self.history.insert(self.last, self.turns);
        }
        self.turns += 1;
        self.last = n;
    }

    fn compute(&mut self) {
        match self.history.get(&self.last) {
            Some(&n) => self.say(self.turns - n),
            None => self.say(0),
        }
    }
}

fn play_game(turns: usize, numbers: &[usize]) -> usize {
    let mut game = Game::new();
    for &n in numbers {
        game.say(n)
    }
    while game.turns < turns {
        game.compute();
    }
    game.last
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc_2020::benchmark;

    #[test]
    fn example_1() {
        let nums = vec![0, 3, 6];
        assert_eq!(436, play_game(2020, &nums));
        assert_eq!(340, play_game(100_000, &nums));
        benchmark(|| play_game(300_000, &nums));
        // assert_eq!(175594, play_game(30_000_000, &nums));
    }

    #[test]
    fn example_2() {
        let nums = vec![1, 3, 2];
        assert_eq!(1, play_game(2020, &nums));
        // assert_eq!(2578, play_game(30_000_000, &nums));
    }

    #[test]
    fn example_3() {
        let nums = vec![2, 1, 3];
        assert_eq!(10, play_game(2020, &nums));
        // assert_eq!(3544142, play_game(30_000_000, &nums));
    }

    #[test]
    fn example_4() {
        let nums = vec![1, 2, 3];
        assert_eq!(27, play_game(2020, &nums));
        // assert_eq!(261214, play_game(30_000_000, &nums));
    }

    #[test]
    fn example_5() {
        let nums = vec![2, 3, 1];
        assert_eq!(78, play_game(2020, &nums));
        // assert_eq!(6895259, play_game(30_000_000, &nums));
    }

    #[test]
    fn example_6() {
        let nums = vec![3, 2, 1];
        assert_eq!(438, play_game(2020, &nums));
        // assert_eq!(18, play_game(30_000_000, &nums));
    }

    #[test]
    fn example_7() {
        let nums = vec![3, 1, 2];
        assert_eq!(1836, play_game(2020, &nums));
        // assert_eq!(362, play_game(30_000_000, &nums));
    }
}
