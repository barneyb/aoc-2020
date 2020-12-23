use aoc_2020::read_input;

pub fn the_work() {
    let s = read_input();
    println!("{:?}", part_one(&s));
}

fn part_one(input: &str) -> usize {
    input.lines().map(|l| evaluate(&l)).sum()
}

fn do_op(c: char, terms: &mut Vec<usize>) {
    let b = terms.pop().unwrap();
    let a = terms.pop().unwrap();
    terms.push(match c {
        '+' => a + b,
        '*' => a * b,
        _ => panic!("Unrecognized operator '{}'", c),
    })
}

fn evaluate(expr: &str) -> usize {
    let mut operators = Vec::new();
    let mut terms = Vec::new();
    for c in expr.chars().filter(|c| !c.is_whitespace()) {
        match c {
            ' ' => {} // soak up spaces
            '1'..='9' => {
                terms.push(c.to_digit(10).unwrap() as usize);
            }
            '(' => operators.push(c),
            ')' => {
                while let Some(c) = operators.pop() {
                    if c == '(' {
                        break;
                    } else {
                        do_op(c, &mut terms)
                    }
                }
            }
            _ => {
                while let Some(op) = operators.pop() {
                    if op == '+' || (c == '*' && op != '(') {
                        do_op(op, &mut terms)
                    } else {
                        operators.push(op);
                        break;
                    }
                }
                operators.push(c);
            }
        }
    }
    while let Some(c) = operators.pop() {
        do_op(c, &mut terms)
    }
    println!("{} => {:?}", expr, terms);
    debug_assert_eq!(1, terms.len());
    terms[0]
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
        assert_eq!(21, evaluate("1 + 2 * 3 + 4"));
        assert_eq!(105, evaluate("1 + 2 * 3 + 4 * 5"));
        assert_eq!(231, evaluate("1 + 2 * 3 + 4 * 5 + 6"));
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
        assert_eq!(46 + 1445 + 669060 + 23340, part_one(s));
    }

    #[test]
    fn test_part_two_examples() {
        assert_eq!(46, evaluate("2 * 3 + (4 * 5)"));
        assert_eq!(1445, evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(
            669060,
            evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
        );
        assert_eq!(
            23340,
            evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );
    }
}
