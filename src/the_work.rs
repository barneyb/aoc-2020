use aoc_2020::read_input;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

pub fn the_work() {
    let input = read_input();
    // 1699 3229 1433 2351 ***pq
    println!("{:?}", part_one(&input));
}

struct Tile {
    num: usize,
    grid: String,
    north: String,
    south: String,
    east: String,
    west: String,
    f_north: String,
    f_south: String,
    f_east: String,
    f_west: String,
}

impl Tile {
    fn new(num: usize, grid: String) -> Tile {
        let north = String::from(&grid[0..10]);
        let south = String::from(&grid[90..100]);
        let west: String = grid.chars().step_by(10).collect();
        let east: String = grid.chars().skip(9).step_by(10).collect();
        let f_north = north.chars().rev().collect();
        let f_south = south.chars().rev().collect();
        let f_west = west.chars().rev().collect();
        let f_east = east.chars().rev().collect();
        Tile {
            num,
            north,
            south,
            west,
            east,
            f_north,
            f_south,
            f_west,
            f_east,
            grid,
        }
    }

    fn edges(&self) -> Vec<&str> {
        vec![
            &self.north,
            &self.south,
            &self.west,
            &self.east,
            &self.f_north,
            &self.f_south,
            &self.f_west,
            &self.f_east,
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
            write!(f, "\n{}", &self.grid[(i * 10)..((i + 1) * 10)])?;
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
    for a in tiles.iter() {
        println!("{}", a.num);
        for (c, edge_a) in vec![
            ('^', &a.north),
            ('<', &a.west),
            ('>', &a.east),
            ('v', &a.south),
        ] {
            let potential_mates = tiles.iter().filter(|b| {
                if a.num == b.num {
                    return false;
                }
                b.edges().iter().any(|e| edge_a == *e)
            });
            println!(
                "  {} {:?}",
                c,
                potential_mates.map(|t| t.num).collect::<Vec<_>>()
            );
        }
    }
    0
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
        assert_eq!("..###..###", t.south);
        assert_eq!("...#.##..#", t.east);
        assert_eq!(".#####..#.", t.west);
    }

    #[test]
    fn example_one() {
        assert_eq!(20899048083289, part_one(&EXAMPLE_ONE));
    }
}
