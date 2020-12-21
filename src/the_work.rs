use aoc_2020::read_input;
use regex::Regex;

pub fn the_work() {
    let s = read_input();
    println!("{:?}", part_one(&s));
}

fn part_one(input: &str) -> isize {
    input.lines().map(|l| evaluate(&l)).sum()
}

lazy_static! {
    static ref RE_EXPR: Regex = Regex::new("([0-9]+) *([+*]) *([0-9]+)").unwrap();
    static ref RE_PARENS: Regex = Regex::new("\\(([0-9]+)\\)").unwrap();
}

fn evaluate(expr: &str) -> isize {
    let mut expr = String::from(expr);
    println!("evaluate: {}", expr);
    loop {
        if let Some(m) = RE_PARENS.captures(&expr) {
            let n = m.get(1).unwrap().as_str().to_string();
            expr.replace_range(m.get(0).unwrap().range(), &n);
            println!("  parens: {}", expr);
            continue;
        }
        if let Some(m) = RE_EXPR.captures(&expr) {
            let a = m.get(1).unwrap().as_str().parse::<isize>().unwrap();
            let op = m.get(2).unwrap().as_str().chars().next().unwrap();
            let b = m.get(3).unwrap().as_str().parse::<isize>().unwrap();
            let v = match op {
                '+' => a + b,
                '*' => a * b,
                _ => panic!("Unrecognized '{}' operator", op),
            };
            expr.replace_range(m.get(0).unwrap().range(), &v.to_string());
            println!("    math: {}", expr);
            continue;
        }
        println!("done    : {}", expr);
        break;
    }
    expr.parse().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const PART_ONE_EXAMPLES: &str = "2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

    #[test]
    fn example_one() {
        assert_eq!(3, evaluate("1 + 2"));
        assert_eq!(9, evaluate("1 + 2 * 3"));
        assert_eq!(13, evaluate("1 + 2 * 3 + 4"));
        assert_eq!(65, evaluate("1 + 2 * 3 + 4 * 5"));
        assert_eq!(71, evaluate("1 + 2 * 3 + 4 * 5 + 6"));
    }

    #[test]
    fn example_two() {
        assert_eq!(1, evaluate("1"));
        assert_eq!(7, evaluate("1 + (2 * 3)"));
        assert_eq!(51, evaluate("1 + (2 * 3) + (4 * (5 + 6))"));
    }

    #[test]
    fn test_part_one() {
        let s = PART_ONE_EXAMPLES.trim();
        assert_eq!(26 + 437 + 12240 + 13632, part_one(s));
    }
}
