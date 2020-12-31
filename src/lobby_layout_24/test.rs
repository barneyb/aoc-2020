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
fn example_four() {
    assert_eq!(10, part_one(&EXAMPLE_FOUR));
}
