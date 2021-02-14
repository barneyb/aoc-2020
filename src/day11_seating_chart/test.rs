use super::*;

const EXAMPLE_ONE: &str = "
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

#[test]
fn test_load_map() {
    let m = load_map(EXAMPLE_ONE);
    assert_eq!(10, m.width);
    assert_eq!(10, m.height);
    assert_eq!(71, m.empty_seat_count());
    assert_eq!(0, m.occupied_seat_count());
}

#[test]
fn test_display() {
    assert_eq!(EXAMPLE_ONE.trim(), load_map(EXAMPLE_ONE).to_string())
}

#[test]
fn test_step() {
    let m = load_map(EXAMPLE_ONE);
    assert_eq!(0, m.occupied_seat_count());
    let m = m.step();
    assert_eq!(
        "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##",
        m.to_string()
    );
    assert_eq!(71, m.occupied_seat_count());
    let m = m.step();
    assert_eq!(
        "#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#",
        m.to_string()
    );
    assert_eq!(7, m.occupied_seat_count());
    let m = m.step();
    assert_eq!(
        "#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#",
        m.to_string()
    );
    assert_eq!(53, m.occupied_seat_count());
}

#[test]
fn test_stabilize() {
    let m = load_map(EXAMPLE_ONE);
    let m = stabilize_map(&m);
    assert_eq!(26, m.occupied_seat_count());
}

#[test]
fn example_two_1() {
    let m = load_map(
        "
.......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....",
    );
    assert_eq!(8, m.occupied_neighbor_count(4 * 9 + 3));
}

#[test]
fn test_directions() {
    let m = load_map(
        "
...
.L.
...",
    );
    assert_eq!(0, m.occupied_neighbor_count(1 * 3 + 1));
    let m = load_map(
        "
#..
.L.
...",
    );
    assert_eq!(1, m.occupied_neighbor_count(1 * 3 + 1));
    let m = load_map(
        "
.#.
.L.
...",
    );
    assert_eq!(1, m.occupied_neighbor_count(1 * 3 + 1));
    let m = load_map(
        "
..#
.L.
...",
    );
    assert_eq!(1, m.occupied_neighbor_count(1 * 3 + 1));
    let m = load_map(
        "
...
.L#
...",
    );
    assert_eq!(1, m.occupied_neighbor_count(1 * 3 + 1));
    let m = load_map(
        "
...
.L.
..#",
    );
    assert_eq!(1, m.occupied_neighbor_count(1 * 3 + 1));
    let m = load_map(
        "
...
.L.
.#.",
    );
    assert_eq!(1, m.occupied_neighbor_count(1 * 3 + 1));
    let m = load_map(
        "
...
.L.
#..",
    );
    assert_eq!(1, m.occupied_neighbor_count(1 * 3 + 1));
    let m = load_map(
        "
...
#L.
...",
    );
    assert_eq!(1, m.occupied_neighbor_count(1 * 3 + 1));

    let m = load_map("L");
    assert_eq!(0, m.occupied_neighbor_count(0));
    let m = load_map("L#");
    assert_eq!(1, m.occupied_neighbor_count(0));
    let m = load_map("#L");
    assert_eq!(1, m.occupied_neighbor_count(1));
    let m = load_map("#\nL");
    assert_eq!(1, m.occupied_neighbor_count(1));
    let m = load_map("L\n#");
    assert_eq!(1, m.occupied_neighbor_count(0));
    let m = load_map("#.\n.L");
    assert_eq!(1, m.occupied_neighbor_count(3));
    let m = load_map(".#\nL.");
    assert_eq!(1, m.occupied_neighbor_count(2));
    let m = load_map("L.\n.#");
    assert_eq!(1, m.occupied_neighbor_count(0));
    let m = load_map(".L\n#.");
    assert_eq!(1, m.occupied_neighbor_count(1));
}

#[test]
fn example_two_2() {
    let m = load_map(
        "
.............
.L.L.#.#.#.#.
.............",
    );
    assert_eq!(0, m.occupied_neighbor_count(1 * 13 + 1));
    assert_eq!(1, m.occupied_neighbor_count(1 * 13 + 3));
}

#[test]
fn example_two_3() {
    let m = load_map(
        "
.##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.",
    );
    assert_eq!(0, m.occupied_neighbor_count(3 * 7 + 3));
}
