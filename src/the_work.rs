use aoc_2020::read_input;
use petgraph::graph::DiGraph;
use petgraph::prelude::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::rc::Rc;
use std::str::FromStr;

pub fn the_work() {
    let tiles = parse(&read_input());
    let graph = build_graph(&tiles);
    // 1699 3229 1433 2351 ***pq
    println!("{:?}", part_one(&graph));
    println!("{:?}", part_two(&graph));
}

const NORTH: usize = 0;
const EAST: usize = 1;
const SOUTH: usize = 2;
const WEST: usize = 3;

#[derive(Debug, Clone)]
struct Tile {
    num: usize,
    pixels: String,
    dim: usize,
    borders: Vec<String>,
}

fn sqrtusize(n: usize) -> usize {
    let r = (n as f64).sqrt() as usize;
    debug_assert_eq!(r * r, n); // no floating point error!
    r
}

impl Tile {
    fn new(num: usize, pixels: &str) -> Tile {
        let pixels = String::from(pixels);
        let dim = sqrtusize(pixels.len());
        Tile {
            num,
            dim,
            borders: Tile::extract_borders(&pixels, dim),
            pixels,
        }
    }

    fn extract_borders(pixels: &String, dim: usize) -> Vec<String> {
        let north = String::from(&pixels[0..dim]);
        let east: String = pixels.chars().skip(dim - 1).step_by(dim).collect();
        let f_south = String::from(&pixels[(pixels.len() - dim)..]);
        let f_west: String = pixels.chars().step_by(dim).collect();
        let f_north = north.chars().rev().collect();
        let f_east = east.chars().rev().collect();
        vec![
            north,
            east,
            f_south.chars().rev().collect(),
            f_west.chars().rev().collect(),
            f_north,
            f_east,
            f_south,
            f_west,
        ]
    }

    fn transform<F>(&mut self, src: F)
    where
        F: Fn(usize, usize) -> (usize, usize),
    {
        let mut next = String::with_capacity(self.pixels.len());
        let bytes = self.pixels.bytes().collect::<Vec<_>>();
        for y in 0..self.dim {
            for x in 0..self.dim {
                let (a, b) = src(x, y);
                next.push(bytes[a * self.dim + b] as char)
            }
        }
        self.pixels = next;
    }

    /// I flip the tile along a vertical axis as if it were sitting on a table and you were to grab
    /// the bottom edge, pick it up, roll your wrist over, and set it back down.
    fn flip(&mut self) {
        let dim = self.dim;
        self.transform(|x, y| (y, dim - x - 1));
        self.borders = Tile::extract_borders(&self.pixels, self.dim);
    }

    // I rotate the tile 90 degrees clockwise without picking it up.
    fn rotate(&mut self, times: usize) {
        if times % 4 == 0 {
            return;
        }
        let dim = self.dim;
        for _ in 0..(times % 4) {
            // This is stupid and inefficient. But not wrong!
            self.transform(|x, y| (dim - x - 1, y));
        }
        self.borders = Tile::extract_borders(&self.pixels, self.dim);
    }

    fn get_border(&self, dir: usize) -> &String {
        &self.borders[dir]
    }

    fn get_flipped_border(&self, dir: usize) -> &String {
        &self.borders[dir + 4]
    }

    // fn face_up_borders(&self) -> &[String] {
    //     &self.borders[..4]
    // }
    //
    // fn face_down_borders(&self) -> &[String] {
    //     &self.borders[4..]
    // }

    fn all_borders(&self) -> &[String] {
        &self.borders
    }

    fn which_border(&self, border: &String) -> Option<usize> {
        self.borders.iter().position(|e| e == border)
    }
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let ci = s.find(':').expect("no colon?!");
        let num = s[5..ci].parse().unwrap();
        Ok(Tile::new(num, &s[(ci + 2)..].replace('\n', "")))
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Tile {}:", self.num)?;
        for i in 0..self.dim {
            write!(f, "\n{}", &self.pixels[(i * self.dim)..((i + 1) * self.dim)])?;
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

fn part_one(graph: Puzzle) -> usize {
    find_corners(&graph)
        .iter()
        .map(|&ni| graph.node_weight(ni).unwrap().num)
        .product()
}

fn part_two(graph: Puzzle) -> usize {
    let grid = assemble_grid(&graph);

    println!("{:?}", grid.iter().map(|t| t.num).collect::<Vec<_>>());
    graph.node_count()
}

fn assemble_grid(graph: Puzzle) -> Vec<Tile> {
    let dim = sqrtusize(graph.node_count());
    let mut grid: Vec<Rc<_>> = Vec::with_capacity(graph.node_count());

    // for each row...
    for y in 0..dim {
        let mut curr = if y == 0 {
            // on the first row, we need to find the top-left corner (an arbitrary choice).
            Rc::new(top_left_corner(&graph))
        } else {
            // on subsequent rows, we need to find the tile which is below the prior row's
            // first tile.
            Rc::new(get_neighbor(graph, &grid[(y - 1) * dim], SOUTH))
        };
        grid.push(Rc::clone(&curr));
        // for each subsequent slot in the row...
        for _ in 1..dim {
            curr = Rc::new(get_neighbor(graph, &curr, EAST));
            // write it down!
            grid.push(curr.clone());
        }
    }
    grid.into_iter()
        .map(|rc| Rc::try_unwrap(rc).unwrap().1)
        .collect::<Vec<_>>()
}

fn get_neighbor(graph: Puzzle, curr: &Rc<(NodeIndex, Tile)>, dir: usize) -> (NodeIndex<u32>, Tile) {
    // For each edge leaving curr (the node to the left), check and see if it's
    // the edge for curr's EAST border (flipped or not), and get the node at
    // the other end. That'll be the next node in the row. We can't use the
    // border directions in the graph edge directly, as the curr node's tile may
    // have been flipped or rotated since the edges were wired up.
    let ni = graph
        .edges_directed(curr.0, Direction::Outgoing)
        .find(|er| {
            if let Some(b) = curr.1.which_border(er.weight().1) {
                b % 4 == dir
            } else {
                false
            }
        })
        .unwrap()
        .target();
    // create a mungible Tile to throw in the grid
    let mut tile = mungible_tile(&graph, ni);
    // find which border will butt up against curr's EAST border (which is mirrored)
    let mut border = tile.which_border(curr.1.get_flipped_border(dir)).unwrap();
    if border >= 4 {
        // if the border is flipped, we need to flip the tile over.
        tile.flip();
        // and exchange EAST/WEST
        if border % 2 == 1 {
            border += 2;
        }
    }
    // Rotate it so the target border is facing curr. We might be rotating
    // backwards on a flipped border, so add a couple extra spins to avoid
    // overflow (they'll get mod-ed away).
    tile.rotate((8 + (dir + 2) - border) % 4);
    (ni, tile)
}

fn top_left_corner(graph: Puzzle) -> (NodeIndex, Tile) {
    let ni = find_corners(&graph)[0];
    let mut tile = mungible_tile(graph, ni);
    // flip/rotate it so that it's truly top-left
    let directions = graph.edges(ni).map(|e| e.weight().0).collect::<Vec<_>>();
    let mut min = directions[0] % 4;
    let mut max = directions[1] % 4;
    if max < min {
        let t = min;
        min = max;
        max = t;
    }
    match min {
        0 if max == 1 => tile.rotate(1),
        0 if max == 3 => tile.rotate(2),
        1 => {} // south east
        2 => tile.rotate(3),
        _ => panic!("{} can't be the min direction?!", min),
    }

    // allows visual sanity checking of the demo
    tile.flip();
    tile.rotate(3);
    // println!("{}", tile);

    (ni, tile)
}

fn mungible_tile(graph: Puzzle, ni: NodeIndex) -> Tile {
    let &gt = graph.node_weight(ni).unwrap();
    Tile::new(gt.num, &gt.pixels)
}

fn find_corners(graph: Puzzle) -> Vec<NodeIndex> {
    graph
        .node_indices()
        .filter(|ni| graph.edges(*ni).count() == 2)
        .collect()
}

type Puzzle<'a> = &'a DiGraph<&'a Tile, (usize, &'a String)>;

fn build_graph(tiles: &[Tile]) -> DiGraph<&Tile, (usize, &String)> {
    let mut node_by_edge = HashMap::new();
    let mut graph = DiGraph::new();
    for t in tiles {
        let node = graph.add_node(t);
        for (i, e) in t.all_borders().into_iter().enumerate() {
            if let Some(existing) = node_by_edge.insert(e, node) {
                if i >= 4 {
                    // it's a flipped edge, which we needed to insert into the map,
                    // but don't want in the graph.
                    continue;
                }
                graph.add_edge(node, existing, (i, e));
                let et = *graph.node_weight(existing).unwrap();
                let ei = et.all_borders().iter().position(|oe| e == oe).unwrap();
                graph.add_edge(existing, node, (ei, e));
            }
        }
    }
    if cfg!(debug_assertions) {
        let dim = sqrtusize(tiles.len());
        debug_assert_eq!(tiles.len(), graph.node_count());
        // The puzzle has dim-1 rows of dim overlapping vertical borders, and
        // dim-1 columns of overlapping horizontal borders, each represented
        // as two directed edges in the graph.
        debug_assert_eq!(2 * 2 * dim * (dim - 1), graph.edge_count());
    }
    graph
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
        assert_eq!("..##.#..#.", t.get_border(NORTH));
        assert_eq!("...#.##..#", t.get_border(EAST));
        assert_eq!("###..###..", t.get_border(SOUTH));
        assert_eq!(".#..#####.", t.get_border(WEST));
    }

    #[test]
    fn test_flip() {
        let mut t = tile_2311();
        t.flip();
        assert_eq!(".#..#.##..", t.get_border(NORTH));
        assert_eq!(".#####..#.", t.get_border(EAST));
        assert_eq!("..###..###", t.get_border(SOUTH));
        assert_eq!("#..##.#...", t.get_border(WEST));
    }

    #[test]
    fn test_rotate_cw() {
        let mut t = tile_2311();
        t.rotate(1);
        assert_eq!(".#..#####.", t.get_border(NORTH));
        assert_eq!("..##.#..#.", t.get_border(EAST));
        assert_eq!("...#.##..#", t.get_border(SOUTH));
        assert_eq!("###..###..", t.get_border(WEST));

        t = Tile::new(42, "abcdefghijklmnop");
        t.rotate(1);
        assert_eq!("mieanjfbokgcplhd", t.pixels);
        t.rotate(1);
        assert_eq!("ponmlkjihgfedcba", t.pixels);
        t.rotate(1);
        assert_eq!("dhlpcgkobfjnaeim", t.pixels);
        t.rotate(1);
        assert_eq!("abcdefghijklmnop", t.pixels);

        t = Tile::new(42, "abcdefghijklmnop");
        t.rotate(2);
        assert_eq!("ponmlkjihgfedcba", t.pixels);
        t.rotate(2);
        assert_eq!("abcdefghijklmnop", t.pixels);

        t = Tile::new(42, "abcdefghijklmnop");
        t.rotate(3);
        assert_eq!("dhlpcgkobfjnaeim", t.pixels);
        t.rotate(3);
        assert_eq!("ponmlkjihgfedcba", t.pixels);
        t.rotate(3);
        assert_eq!("mieanjfbokgcplhd", t.pixels);
        t.rotate(3);
        assert_eq!("abcdefghijklmnop", t.pixels);

        t = Tile::new(42, "abcdefghijklmnop");
        t.rotate(3);
        t.rotate(5);
        assert_eq!("abcdefghijklmnop", t.pixels);
    }

    #[test]
    fn example_one() {
        let tiles = parse(&EXAMPLE_ONE);
        let graph = build_graph(&tiles);
        assert_eq!(20899048083289, part_one(&graph));
        assert_eq!(
            vec![1951, 2311, 3079, 2729, 1427, 2473, 2971, 1489, 1171],
            assemble_grid(&graph)
                .iter()
                .map(|t| t.num)
                .collect::<Vec<_>>()
        );
        assert_eq!(273, part_two(&graph));
    }
}
