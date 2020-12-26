use aoc_2020::geom2d::Dir;
use aoc_2020::read_input;
use std::fmt::{self, Debug, Display, Formatter};
use std::str::FromStr;
use std::rc::Rc;
use std::collections::HashMap;

pub fn the_work() {
    let input = read_input();
    let tiles = parse(&input);
    // 1699 3229 1433 2351 ***pq
    println!("{:?}", part_one(&tiles));
}

#[derive(Default)]
struct Tile {
    num: usize,
    pixels: String,
    dim: usize,
    edges: [usize; 4],
    neighbors: [Option<Rc<Tile>>; 4],
}

const UP: usize = 0;
const DOWN: usize = 1;
const RIGHT: usize = 2;
const LEFT: usize = 3;

fn bitter(bits: usize, c: char) -> usize {
    let mut n = bits << 1;
    if c == '#' {
        n += 1;
    }
    n
}

fn str2bits(s: &str) -> usize {
    s.chars().fold(0, bitter)
}

fn get_edges(pixels: &String, dim: usize) -> [usize; 4] {
    [
        str2bits(&pixels[0..dim]),
        str2bits(&pixels[(pixels.len() - dim)..]),
        pixels
            .chars()
            .skip(dim - 1)
            .step_by(dim)
            .fold(0, bitter),
        pixels.chars().step_by(dim).fold(0, bitter),
    ]
}

fn usize_sqrt(n: usize) -> usize {
    let r: usize = (n as f64).sqrt() as usize;
    debug_assert_eq!(r * r, n); // no floating point error!
    r
}

impl Tile {
    fn new(num: usize, pixels: String) -> Tile {
        let dim = usize_sqrt(pixels.len());
        Tile {
            num,
            dim,
            edges: get_edges(&pixels, dim),
            pixels,
            ..Default::default()
        }
    }

    fn transform<F>(&mut self, trans: F)
    where F: Fn(usize, usize, usize) -> (usize, usize)
    {
        let mut next = String::with_capacity(self.pixels.len());
        let bytes = self.pixels.bytes().collect::<Vec<_>>();
        for y in 0..self.dim {
            for x in 0..self.dim {
                let (a, b) = trans(x, y, self.dim);
                next.push(bytes[a * self.dim + b] as char)
            }
        }
        self.pixels = next;
        self.edges = get_edges(&self.pixels, self.dim);
    }

    /// I flip the tile along a vertical axis as if it were sitting on a table and you were to grab
    /// the bottom edge, pick it up, roll your wrist over, and set it back down.
    fn flip(&mut self) {
        self.transform(|x, y, dim| (y, dim - x - 1));
    }

    // I rotate the tile 90 degrees clockwise without picking it up.
    fn rotate(&mut self) {
        self.transform(|x, y, dim| ((dim - x - 1), y));
    }

    fn get_edge(&self, d: Dir) -> usize {
        self.edges[match d {
            Dir::North => 0,
            Dir::South => 1,
            Dir::East => 2,
            Dir::West => 3,
        }]
    }

    fn all_edges(&self) -> Vec<usize> { // todo: remove
        let mut es = Vec::with_capacity(8);
        fn reverse(mut it: usize, dim: usize) -> usize {
            let mut n = 0;
            for _ in 0..dim {
                n <<= 1;
                n += it & 1;
                it >>= 1;
            }
            n
        }
        for &e in self.edges.iter() {
            es.push(e);
            es.push(reverse(e, self.dim));
        }
        es
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

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.num)
    }
}

fn parse(input: &str) -> Vec<Tile> {
    input
        .trim()
        .split("\n\n")
        .map(|t| t.parse().unwrap())
        .collect()
}

fn lay_out_tiles(tiles: &Vec<Tile>) -> Vec<&Tile> {
    let by_num = tiles.iter().map(|t| (t.num, t)).collect::<HashMap<_, _>>();
    let curr = tiles.get(0).unwrap();
    if let Some(up) = tiles.iter().find(|&t| t.num != curr.num && t.edges.contains(&curr.edges[UP])) {

    } else if let Some(down) = tiles.iter().find(|&t| t.num != curr.num && t.edges.contains(&curr.edges[DOWN])) {

    } else {
        // todo: flip it and try again
    }

    vec![]
}

fn part_one(tiles: &Vec<Tile>) -> usize {
    let laid_out = lay_out_tiles(tiles);
    let dim = usize_sqrt(laid_out.len());
    laid_out[0].num * laid_out[dim - 1].num * laid_out[dim * 2].num * laid_out[laid_out.len() - 1].num
    // tiles
    //     .iter()
    //     .filter(|a| {
    //         println!("{}", a.num);
    //         vec![
    //             ('^', &a.get_edge(Dir::North)),
    //             ('<', &a.get_edge(Dir::West)),
    //             ('>', &a.get_edge(Dir::East)),
    //             ('v', &a.get_edge(Dir::South)),
    //         ]
    //         .iter()
    //         .filter(|(c, &edge_a)| {
    //             let potential_mates = tiles
    //                 .iter()
    //                 .filter(|b| {
    //                     if a.num == b.num {
    //                         return false;
    //                     }
    //                     b.all_edges().iter().any(|&e| edge_a == e)
    //                 })
    //                 .collect::<Vec<_>>();
    //             println!(
    //                 "  {} {:?}",
    //                 c,
    //                 potential_mates.iter().map(|t| t.num).collect::<Vec<_>>()
    //             );
    //             potential_mates.len() > 0
    //         })
    //         .count()
    //             == 2
    //     })
    //     .map(|c| c.num)
    //     .product()
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
    fn test_parse() {
        let t = tile_2311();
        assert_eq!(2311, t.num);
        assert_eq!(str2bits("..##.#..#."), t.get_edge(Dir::North));
        assert_eq!(str2bits("..###..###"), t.get_edge(Dir::South));
        assert_eq!(str2bits("...#.##..#"), t.get_edge(Dir::East));
        assert_eq!(str2bits(".#####..#."), t.get_edge(Dir::West));
    }

    #[test]
    fn test_flip() {
        let mut t = tile_2311();
        t.flip();
        assert_eq!(str2bits(".#..#.##.."), t.get_edge(Dir::North));
        assert_eq!(str2bits("###..###.."), t.get_edge(Dir::South));
        assert_eq!(str2bits(".#####..#."), t.get_edge(Dir::East));
        assert_eq!(str2bits("...#.##..#"), t.get_edge(Dir::West));
    }

    #[test]
    fn test_rotate_cw() {
        let mut t = tile_2311();
        t.rotate();
        assert_eq!(str2bits(".#..#####."), t.get_edge(Dir::North));
        assert_eq!(str2bits("#..##.#..."), t.get_edge(Dir::South));
        assert_eq!(str2bits("..##.#..#."), t.get_edge(Dir::East));
        assert_eq!(str2bits("..###..###"), t.get_edge(Dir::West));
    }

    #[test]
    fn example_one() {
        let tiles = parse(&EXAMPLE_ONE);
        // assert_eq!(
        //     vec![1951, 2311, 3079, 2729, 1427, 2473, 2971, 1489, 1171],
        //     lay_out_tiles(&tiles).iter().map(|t| t.num).collect::<Vec<_>>()
        // );
        assert_eq!(20899048083289, part_one(&tiles));
    }
}
