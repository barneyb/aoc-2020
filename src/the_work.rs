use std::collections::HashMap;

pub fn the_work() {
    println!("{:?}", get_2020th(&vec![0, 14, 6, 20, 1, 4]));
}

struct Game {
    map: HashMap<usize, (usize, usize)>,
    last: usize,
    turns: usize,
}

impl Game {
    fn new() -> Game {
        Game {
            map: HashMap::new(),
            last: 0,
            turns: 0,
        }
    }

    fn say(&mut self, n: usize) {
        self.turns += 1;
        self.last = n;
        if let Some(&(a, _)) = self.map.get(&n) {
            self.map.insert(n, (self.turns, a));
        } else {
            self.map.insert(n, (self.turns, 0));
        }
    }

    fn compute(&mut self) {
        match self.map.get(&self.last) {
            Some(&(_, 0)) => self.say(0),
            Some(&(a, b)) => self.say(a - b),
            None => panic!("what? {} was said, but hasn't been recorded?!", self.last),
        }
    }
}

fn get_2020th(numbers: &[usize]) -> usize {
    let mut game = Game::new();
    for &n in numbers {
        game.say(n)
    }
    while game.turns < 2020 {
        game.compute();
    }
    game.last
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(436, get_2020th(&vec![0, 3, 6]));
    }

    #[test]
    fn example_2() {
        assert_eq!(1, get_2020th(&vec![1, 3, 2]));
    }

    #[test]
    fn example_3() {
        assert_eq!(10, get_2020th(&vec![2, 1, 3]));
    }

    #[test]
    fn example_4() {
        assert_eq!(27, get_2020th(&vec![1, 2, 3]));
    }

    #[test]
    fn example_5() {
        assert_eq!(78, get_2020th(&vec![2, 3, 1]));
    }

    #[test]
    fn example_6() {
        assert_eq!(438, get_2020th(&vec![3, 2, 1]));
    }

    #[test]
    fn example_7() {
        assert_eq!(1836, get_2020th(&vec![3, 1, 2]));
    }
}
