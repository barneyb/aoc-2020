use super::*;

#[test]
fn test_tile_motion() {
    let t = Tile::origin();
    assert_eq!(t.step(NorthWest), t.step(NorthEast).step(West));
    assert_eq!(t.step(NorthWest), t.step(West).step(NorthEast));

    assert_eq!(t.step(NorthEast), t.step(NorthWest).step(East));
    assert_eq!(t.step(NorthEast), t.step(East).step(NorthWest));

    assert_eq!(t.step(East), t.step(NorthEast).step(SouthEast));
    assert_eq!(t.step(East), t.step(SouthEast).step(NorthEast));

    assert_eq!(t.step(SouthEast), t.step(East).step(SouthWest));
    assert_eq!(t.step(SouthEast), t.step(SouthWest).step(East));

    assert_eq!(t.step(SouthWest), t.step(West).step(SouthEast));
    assert_eq!(t.step(SouthWest), t.step(SouthEast).step(West));

    assert_eq!(t.step(West), t.step(NorthWest).step(SouthWest));
    assert_eq!(t.step(West), t.step(SouthWest).step(NorthWest));
}

#[test]
fn test_parse_single() {
    assert_eq!(vec![NorthWest], parse_path("nw"));
    assert_eq!(vec![NorthEast], parse_path("ne"));
    assert_eq!(vec![East], parse_path("e"));
    assert_eq!(vec![SouthEast], parse_path("se"));
    assert_eq!(vec![SouthWest], parse_path("sw"));
    assert_eq!(vec![West], parse_path("w"));
}

#[test]
fn test_neighbors() {
    let t = Tile::origin();
    let ns = vec![
        t.step(NorthWest),
        t.step(NorthEast),
        t.step(East),
        t.step(SouthEast),
        t.step(SouthWest),
        t.step(West),
    ];
    assert_eq!(ns, t.neighbors().collect::<Vec<_>>())
}

const EXAMPLE_ONE: &str = "esenee"; // move one tile east, one tile southeast, one tile northeast, and one tile east

#[test]
fn example_one() {
    assert_eq!(
        vec![East, SouthEast, NorthEast, East],
        parse_path(&EXAMPLE_ONE)
    );
}

const EXAMPLE_TWO: &str = "esew"; // immediately adjacent to the reference tile

#[test]
fn example_two() {
    assert_eq!(vec![East, SouthEast, West], parse_path(&EXAMPLE_TWO));
}

const EXAMPLE_THREE: &str = "nwwswee"; // the reference tile

#[test]
fn example_three() {
    let path = parse_path(&EXAMPLE_THREE);
    assert_eq!(vec![NorthWest, West, SouthWest, East, East], path);
    assert_eq!(Tile::origin(), Tile::origin().walk(&path));
}

const EXAMPLE_FOUR: &str = "\
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

#[test]
fn example_four_part_one() {
    let layout = initial_layout(&EXAMPLE_FOUR);
    assert_eq!(10, layout.len());
}

#[test]
fn example_four_part_two() {
    let mut layout = initial_layout(&EXAMPLE_FOUR);
    for day in 1..=100 {
        layout = do_step(&layout);
        if let Some(e) = match day {
            1 => Some(15),
            2 => Some(12),
            3 => Some(25),
            4 => Some(14),
            5 => Some(23),
            6 => Some(28),
            7 => Some(41),
            8 => Some(37),
            9 => Some(49),
            10 => Some(37),
            20 => Some(132),
            30 => Some(259),
            40 => Some(406),
            50 => Some(566),
            60 => Some(788),
            70 => Some(1106),
            80 => Some(1373),
            90 => Some(1844),
            100 => Some(2208),
            _ => None,
        } {
            assert_eq!(
                e,
                layout.len(),
                "Day {} should have {} black tiles, but has {}",
                day,
                e,
                layout.len()
            );
        }
    }

    let layout = initial_layout(&EXAMPLE_FOUR);
    assert_eq!(2208, part_two(&layout));
}
