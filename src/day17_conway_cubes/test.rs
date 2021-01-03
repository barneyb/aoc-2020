use super::*;

const EXAMPLE_ONE: &str = ".#.
..#
###";

#[test]
fn example_one() {
    let s = EXAMPLE_ONE.trim();
    // assert_eq!(112, run_simulation(s));
    assert_eq!(848, run_simulation(s));
}

// #[test]
// fn example_one_part_one_cycle_by_cycle() {
//     let g = Game::new(EXAMPLE_ONE.trim());
//     println!("Before any cycles:\n\n{}", g);
//     assert_eq!(0, g.cycle_count);
//     assert_eq!(5, g.get_active_cell_count());
//     let g = g.cycle();
//     println!("After 1 cycle:\n\n{}", g);
//     assert_eq!(1, g.cycle_count);
//     assert_eq!(11, g.get_active_cell_count());
//     let g = g.cycle();
//     println!("After 2 cycles:\n\n{}", g);
//     assert_eq!(2, g.cycle_count);
//     assert_eq!(21, g.get_active_cell_count());
//     let g = g.cycle();
//     println!("After 3 cycles:\n\n{}", g);
//     assert_eq!(3, g.cycle_count);
//     assert_eq!(38, g.get_active_cell_count());
//     let g = g.cycle().cycle().cycle();
//     assert_eq!(6, g.cycle_count);
//     assert_eq!(112, g.get_active_cell_count());
// }

#[test]
fn example_one_part_two_cycle_by_cycle() {
    let g = Game::new(EXAMPLE_ONE.trim());
    println!("Before any cycles:\n\n{}", g);
    assert_eq!(0, g.cycle_count);
    assert_eq!(5, g.get_active_cell_count());
    let g = g.cycle();
    println!("After 1 cycle:\n\n{}", g);
    assert_eq!(1, g.cycle_count);
    assert_eq!(29, g.get_active_cell_count());
    let g = g.cycle();
    println!("After 2 cycles:\n\n{}", g);
    assert_eq!(2, g.cycle_count);
    assert_eq!(60, g.get_active_cell_count());
}
