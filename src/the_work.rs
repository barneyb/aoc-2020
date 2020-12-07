use aoc_2020 as aoc;
use std::collections::HashMap;

use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use std::collections::{HashSet, LinkedList};
use std::str::FromStr;

pub fn the_work() {
    let bags = aoc::read_lines(|l| l.parse::<Bag>().unwrap());
    let (one, two) = count_containers(&bags);
    println!("{}\n{}", one, two);
}

fn count_containers(bags: &[Bag]) -> (usize, usize) {
    let mut lookup = HashMap::new();
    let mut graph = DiGraph::<&str, usize>::new();
    for b in bags {
        lookup.insert(&b.color[..], graph.add_node(&b.color));
    }
    for b in bags {
        for (sb, n) in &b.contains {
            graph.add_edge(
                *lookup.get(&b.color[..]).unwrap(),
                *lookup.get(&sb[..]).unwrap(),
                *n,
            );
        }
    }
    let shiny_gold = lookup.get("shiny gold").unwrap();
    (
        count_distinct_containers(&graph, &shiny_gold),
        compute_downstreams(&graph, &shiny_gold),
    )
}

fn count_distinct_containers(graph: &DiGraph<&str, usize>, start: &NodeIndex) -> usize {
    let mut containers = HashSet::new();
    let mut queue = LinkedList::new();
    queue.push_back(*start);
    loop {
        match queue.pop_front() {
            None => break,
            Some(ni) => {
                containers.insert(ni);
                for e in graph.edges_directed(ni, Direction::Incoming) {
                    queue.push_back(e.source())
                }
            }
        }
    }
    containers.len() - 1 // the start node doesn't count
}

fn compute_downstreams(graph: &DiGraph<&str, usize>, start: &NodeIndex) -> usize {
    let mut count = 0;
    let mut queue = LinkedList::new();
    queue.push_back((*start, 1));
    loop {
        match queue.pop_front() {
            None => break,
            Some((ni, factor)) => {
                count += factor;
                for e in graph.edges_directed(ni, Direction::Outgoing) {
                    queue.push_back((e.target(), factor * e.weight()))
                }
            }
        }
    }
    count - 1 // the start node one doesn't count
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
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let mut words = s
            .split(',')
            .map(|s| s.trim())
            .filter(|s| s.len() > 0);
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

    const EXAMPLE_INPUT: &str = "
light red , 1, bright white , 2, muted yellow ,
dark orange ,  3, bright white ,, 4, muted yellow ,
bright white ,  1, shiny gold ,
muted yellow ,  2, shiny gold ,, 9, faded blue ,
shiny gold ,  1, dark olive , 2, vibrant plum ,
dark olive ,  3, faded blue ,, 4, dotted black ,
vibrant plum ,  5, faded blue ,, 6, dotted black ,
faded blue ,  ,, ,
dotted black ,  ,, ,";

    const EXAMPLE_INPUT_TWO: &str = "
shiny gold , 2 ,dark red ,
dark red , 2 ,dark orange ,
dark orange , 2 ,dark yellow ,
dark yellow , 2 ,dark green ,
dark green , 2 ,dark blue ,
dark blue , 2 ,dark violet ,
dark violet ,,";

    #[test]
    fn example_one() {
        let bags = EXAMPLE_INPUT
            .trim()
            .lines()
            .map(|l| l.parse::<Bag>().unwrap())
            .collect::<Vec<Bag>>();
        assert_eq!((4, 32), count_containers(&bags));
    }

    #[test]
    fn example_two() {
        let bags = EXAMPLE_INPUT_TWO
            .trim()
            .lines()
            .map(|l| l.parse::<Bag>().unwrap())
            .collect::<Vec<Bag>>();
        // this one's degenerate, shiny is the sole root
        assert_eq!((0, 126), count_containers(&bags));
    }

    #[test]
    fn test_parse() {
        let mut lines = EXAMPLE_INPUT.trim().lines();
        let b = lines.next().unwrap().parse::<Bag>().unwrap();
        assert_eq!("light red", b.color);
        assert_eq!(&1, b.contains.get("bright white").unwrap());
        assert_eq!(&2, b.contains.get("muted yellow").unwrap());
        assert_eq!(2, b.contains.len());
    }
}
