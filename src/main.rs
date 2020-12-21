use aoc_2020 as aoc;

fn main() {
    let mut expenses = aoc::read_lines(|s| s.parse::<i32>().unwrap());
    expenses.sort();
    let mut ni = 0;
    let mut nj = 0;
    let mut nk = 0;
    'outer: for (i, &a) in expenses.iter().enumerate() {
        ni += 1;
        for b in expenses.iter().skip(i + 1) {
            nj += 1;
            let c = 2020 - a - b;
            if c <= 0 {
                break;
            }
	    nk += 7; // log base 2 of 200 (expense count)
 	    if let Ok(_) = expenses.binary_search(&c) {
                println!("{} * {} * {} = {}", a, b, c, a * b * c);
                break 'outer;
            }
        }
    }
    println!("{}, {}, {}", ni, nj, nk);

// brute force
// 401 * 1390 * 229 = 127642310
// 15, 2953, 590589

// i-skips
// 401 * 1390 * 229 = 127642310
// 15, 2833, 296143

// with remaining
// 401 * 1390 * 229 = 127642310
// 15, 2833, 15241

// sort and short circuit
// 229 * 401 * 1390 = 127642310
// 4, 519, 2118

// binary search
// 229 * 401 * 1390 = 127642310
// 4, 519, 2580

}
