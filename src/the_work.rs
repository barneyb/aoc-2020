use aoc_2020 as aoc;
use std::collections::HashMap;

use petgraph::graph::DiGraph;
use petgraph::Direction;
use std::collections::{HashSet, LinkedList};
use std::str::FromStr;

pub fn the_work() {
    let bags = aoc::read_lines(|l| l.parse::<Bag>().unwrap());
    println!("{}", count_containers(&bags));
}

fn count_containers(bags: &[Bag]) -> usize {
    let mut lookup = HashMap::new();
    let mut graph = DiGraph::<&str, &usize>::new();
    for b in bags {
        lookup.insert(&b.color[..], graph.add_node(&b.color));
    }
    for b in bags {
        for (sb, n) in &b.contains {
            graph.add_edge(
                *lookup.get(&b.color[..]).unwrap(),
                *lookup.get(&sb[..]).unwrap(),
                n,
            );
        }
    }
    let mut containers = HashSet::new();
    let mut queue = LinkedList::new();
    queue.push_back("shiny gold");
    loop {
        match queue.pop_front() {
            None => break,
            Some(color) => {
                containers.insert(color);
                let ni = lookup.get(color).unwrap();
                println!("process '{}' to '{:?}'", color, ni);
                for n in graph.neighbors_directed(*ni, Direction::Incoming) {
                    println!("  neighbor '{}'", graph[n]);
                    queue.push_back(graph[n])
                }
            }
        }
    }
    containers.len() - 1 // since 'shiny gold' is in there
}

#[derive(Debug)]
struct Bag {
    color: String,
    contains: HashMap<String, usize>,
}

impl Bag {
    fn new(color: String) -> Bag {
        Bag {
            color,
            contains: HashMap::new(),
        }
    }

    fn add_contains(&mut self, color: String, num: usize) {
        self.contains.insert(color, num);
    }
}

impl FromStr for Bag {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split(',').map(|s| s.trim()).filter(|s| s.len() > 0);
        let mut bag = Bag::new(words.next().unwrap().to_string());
        loop {
            let num = match words.next() {
                None => break,
                Some(num) => num
                    .parse::<usize>()
                    .expect(&format!("failed to parse '{}'", num)),
            };
            let clr = words.next().unwrap();
            bag.add_contains(clr.to_string(), num);
        }

        Ok(bag)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = "light red , 1, bright white , 2, muted yellow ,
dark orange ,  3, bright white ,, 4, muted yellow ,
bright white ,  1, shiny gold ,
muted yellow ,  2, shiny gold ,, 9, faded blue ,
shiny gold ,  1, dark olive , 2, vibrant plum ,
dark olive ,  3, faded blue ,, 4, dotted black ,
vibrant plum ,  5, faded blue ,, 6, dotted black ,
faded blue ,  ,, ,
dotted black ,  ,, ,";

    #[test]
    fn test_wire() {
        let bags = EXAMPLE_INPUT
            .lines()
            .map(|l| l.parse::<Bag>().unwrap())
            .collect::<Vec<Bag>>();
        assert_eq!(4, count_containers(&bags))
    }

    #[test]
    fn test_parse() {
        let mut lines = EXAMPLE_INPUT.lines();
        let b = lines.next().unwrap().parse::<Bag>().unwrap();
        assert_eq!("light red", b.color);
        assert_eq!(&1, b.contains.get("bright white").unwrap());
        assert_eq!(&2, b.contains.get("muted yellow").unwrap());
        assert_eq!(2, b.contains.len());
    }
}
