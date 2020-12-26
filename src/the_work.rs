use aoc_2020::geom2d::Dir;
use aoc_2020::read_input;
use std::collections::HashMap;
use std::fmt::{self, Debug, Display, Formatter};
use std::str::FromStr;

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
    north_edge: Option<String>,
    south_edge: Option<String>,
    east_edge: Option<String>,
    west_edge: Option<String>,
}

impl Tile {
    fn new(num: usize, pixels: String) -> Tile {
        let dim: usize = (pixels.len() as f64).sqrt() as usize;
        debug_assert_eq!(dim * dim, pixels.len()); // no floating point error!
        Tile {
            num,
            pixels,
            dim,
            ..Default::default()
        }
    }

    /// I flip the tile along a vertical axis as if it were sitting on a table and you were to grab
    /// the bottom edge, pick it up, roll your wrist over, and set it back down.
    fn flip(&mut self) {
        let mut next = String::with_capacity(self.pixels.len());
        let bytes = self.pixels.bytes().collect::<Vec<_>>();
        for y in 0..self.dim {
            for x in 0..self.dim {
                next.push(bytes[y * self.dim + (self.dim - x - 1)] as char)
            }
        }
        self.pixels = next;
        self.north_edge = None;
        self.south_edge = None;
        self.east_edge = None;
        self.west_edge = None;
    }

    // I rotate the tile 90 degrees clockwise without picking it up.
    fn rotate(&mut self) {
        let mut next = String::with_capacity(self.pixels.len());
        let bytes = self.pixels.bytes().collect::<Vec<_>>();
        for y in 0..self.dim {
            for x in 0..self.dim {
                next.push(bytes[(self.dim - x - 1) * self.dim + y] as char)
            }
        }
        self.pixels = next;
        // don't need to recompute these; take() will clear the RHS
        self.east_edge = self.north_edge.take();
        self.west_edge = self.south_edge.take();
    }

    fn get_edge(&mut self, d: Dir) -> &str {
        match d {
            Dir::North => {
                if let None = &self.north_edge {
                    self.north_edge = Some(String::from(&self.pixels[0..self.dim]));
                }
                if let Some(s) = &self.north_edge {
                    return s
                }
            }
            Dir::South => {
                if let None = &self.south_edge {
                    self.south_edge = Some(String::from(&self.pixels[(self.pixels.len() - self.dim)..]));
                }
                if let Some(s) = &self.south_edge {
                    return s
                }
            }
            Dir::East => {
                if let None = &self.east_edge {
                    self.east_edge = Some(self.pixels.chars().skip(self.dim - 1).step_by(self.dim).collect());
                }
                if let Some(s) = &self.east_edge {
                    return s
                }
            }
            Dir::West => {
                if let None = &self.west_edge {
                    self.west_edge = Some(self.pixels.chars().step_by(self.dim).collect());
                }
                if let Some(s) = &self.west_edge {
                    return s
                }
            }
        }
        panic!() // i cannot figure out how to satisfy the borrow checker...
    }

    // fn edges(&self) -> Vec<&str> { // todo: remove
    //     vec![
    //         &self.north,
    //         &self.south,
    //         &self.west,
    //         &self.east,
    //         &self.f_north,
    //         &self.f_south,
    //         &self.f_west,
    //         &self.f_east,
    //     ]
    // }
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

// fn lay_out_tiles(tiles: &Vec<Tile>) -> Vec<&Tile> {
//     let by_num = tiles.iter().map(|t| (t.num, t)).collect::<HashMap<_, _>>();
//
//     vec![]
// }

fn part_one(tiles: &Vec<Tile>) -> usize {
    // todo: use layout...
    // tiles
    //     .iter()
    //     .filter(|a| {
    //         println!("{}", a.num);
    //         vec![
    //             ('^', &a.north),
    //             ('<', &a.west),
    //             ('>', &a.east),
    //             ('v', &a.south),
    //         ]
    //         .iter()
    //         .filter(|(c, edge_a)| {
    //             let potential_mates = tiles
    //                 .iter()
    //                 .filter(|b| {
    //                     if a.num == b.num {
    //                         return false;
    //                     }
    //                     b.edges().iter().any(|e| edge_a == e)
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
    4
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
        let mut t = tile_2311();
        assert_eq!(2311, t.num);
        assert_eq!("..##.#..#.", t.get_edge(Dir::North));
        assert_eq!("..###..###", t.get_edge(Dir::South));
        assert_eq!("...#.##..#", t.get_edge(Dir::East));
        assert_eq!(".#####..#.", t.get_edge(Dir::West));
    }

    #[test]
    fn test_flip() {
        let mut t = tile_2311();
        t.flip();
        assert_eq!(".#..#.##..", t.get_edge(Dir::North));
        assert_eq!("###..###..", t.get_edge(Dir::South));
        assert_eq!(".#####..#.", t.get_edge(Dir::East));
        assert_eq!("...#.##..#", t.get_edge(Dir::West));
    }

    #[test]
    fn test_rotate_cw() {
        let mut t = tile_2311();
        t.rotate();
        assert_eq!(".#..#####.", t.get_edge(Dir::North));
        assert_eq!("#..##.#...", t.get_edge(Dir::South));
        assert_eq!("..##.#..#.", t.get_edge(Dir::East));
        assert_eq!("..###..###", t.get_edge(Dir::West));
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
