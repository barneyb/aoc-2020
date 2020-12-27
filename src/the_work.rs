use aoc_2020::read_input;
use petgraph::graph::Graph;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

pub fn the_work() {
    let input = read_input();
    // 1699 3229 1433 2351 ***pq
    println!("{:?}", part_one(&input));
}

struct Tile {
    num: usize,
    pixels: String,
    north: String,
    east: String,
    south: String,
    west: String,
    f_north: String,
    f_east: String,
    f_south: String,
    f_west: String,
}

fn usize_sqrt(n: usize) -> usize {
    let r: usize = (n as f64).sqrt() as usize;
    debug_assert_eq!(r * r, n); // no floating point error!
    r
}

impl Tile {
    fn new(num: usize, pixels: String) -> Tile {
        let dim = usize_sqrt(pixels.len());
        let north = String::from(&pixels[0..dim]);
        let east: String = pixels.chars().skip(dim - 1).step_by(dim).collect();
        let f_south = String::from(&pixels[(pixels.len() - dim)..]);
        let f_west: String = pixels.chars().step_by(dim).collect();
        let f_north = north.chars().rev().collect();
        let f_east = east.chars().rev().collect();
        let south = f_south.chars().rev().collect();
        let west = f_west.chars().rev().collect();
        Tile {
            num,
            pixels,
            north,
            east,
            south,
            west,
            f_north,
            f_east,
            f_south,
            f_west,
        }
    }

    fn edges(&self) -> Vec<&str> {
        vec![
            &self.north,
            &self.east,
            &self.south,
            &self.west,
            &self.f_north,
            &self.f_east,
            &self.f_south,
            &self.f_west,
        ]
    }
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let ci = s.find(':').expect("no colon?!");
        let num = s[5..ci].parse().unwrap();
        Ok(Tile::new(num, s[(ci + 2)..].replace('\n', "")))
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Tile {}:", self.num)?;
        for i in 0..10 {
            write!(f, "\n{}", &self.pixels[(i * 10)..((i + 1) * 10)])?;
        }
        Ok(())
    }
}

fn parse(input: &str) -> Vec<Tile> {
    input
        .trim()
        .split("\n\n")
        .map(|t| t.parse().unwrap())
        .collect()
}

fn part_one(input: &str) -> usize {
    let tiles = &parse(input);
    let mut node_by_num = HashMap::new();
    let mut node_by_edge = HashMap::new();
    let mut graph = Graph::new();
    for t in tiles {
        let node = graph.add_node(&t.num);
        if let Some(_) = node_by_num.insert(t.num, node) {
            panic!("duplicate tile '{}'?!", t.num)
        }
        for e in t.edges() {
            if let Some(n) = node_by_edge.insert(e, node) {
                graph.update_edge(n, node, e);
                graph.update_edge(node, n, e);
            }
        }
    }
    node_by_num
        .iter()
        .filter(|&(_, &ni)| graph.edges(ni).count() == 2)
        .map(|(num, _)| num)
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_ONE: &str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

    fn tile_2311() -> Tile {
        EXAMPLE_ONE.split("\n\n").next().unwrap().parse().unwrap()
    }

    #[test]
    fn parse() {
        let t = tile_2311();
        assert_eq!(2311, t.num);
        assert_eq!("..##.#..#.", t.north);
        assert_eq!("...#.##..#", t.east);
        assert_eq!("..###..###", t.f_south);
        assert_eq!(".#####..#.", t.f_west);
    }

    #[test]
    fn example_one() {
        assert_eq!(20899048083289, part_one(&EXAMPLE_ONE));
    }
}
