use aoc_2020::read_input;
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;

pub fn the_work() {
    let s = read_input();
    println!("{:?}", part_one(&s));
}

fn part_one(input: &str) -> usize {
    let mut lines = input.trim().lines();
    let rule_list = lines
        .by_ref()
        .take_while(|&l| !l.is_empty())
        .collect::<Vec<_>>();
    let mut rule = Flattener::new(&rule_list).flattened();
    rule.insert(0, '^');
    rule.push('$');
    let re = Regex::new(&rule).unwrap();
    lines.filter(|l| re.is_match(l)).count()
}

struct Flattener<'a> {
    unparsed: HashMap<&'a str, &'a str>,
    parsed: RefCell<HashMap<&'a str, String>>,
}

impl<'a> Flattener<'a> {
    fn new(rules: &[&'a str]) -> Flattener<'a> {
        let mut unparsed = HashMap::new();
        for &r in rules {
            let ci = match r.find(':') {
                Some(idx) => idx,
                None => panic!("Rule '{}' has no colon!?", r),
            };
            unparsed.insert(&r[0..ci], r[(ci + 1)..].trim());
        }
        Flattener {
            unparsed,
            parsed: RefCell::new(HashMap::new()),
        }
    }

    fn flattened(&self) -> String {
        self.get_rule("0").replace('.', "")
    }

    fn get_rule(&self, num: &'a str) -> String {
        if let Some(s) = self.parsed.borrow().get(num) {
            return s.to_owned();
        }
        let result = match self.unparsed.get(num) {
            Some(s) => match s.chars().next() {
                Some('"') => String::from(&s[1..(s.len() - 1)]),
                _ => {
                    let mut result = String::from("(");
                    for t in s.split(' ') {
                        match t {
                            "|" => result.push_str("|"),
                            _ => {
                                match result.chars().last().unwrap() {
                                    ')' | 'a'..='z' => result.push('.'),
                                    _ => {}
                                }
                                result.push_str(&self.get_rule(t))
                            }
                        };
                    }
                    result.push(')');
                    result
                }
            },
            None => panic!("There's no rule '{}'?!", num),
        };
        self.parsed
            .borrow_mut()
            .entry(num)
            .or_insert(result)
            .to_owned()
    }
}

fn to_postfix(expr: &str) -> String {
    let mut operators = Vec::new();
    let mut postfix = String::new();
    for c in expr.chars().filter(|c| !c.is_whitespace()) {
        match c {
            'a'..='z' => postfix.push(c),
            '(' => operators.push(c),
            ')' => {
                while let Some(c) = operators.pop() {
                    if c == '(' {
                        break;
                    } else {
                        postfix.push(c);
                    }
                }
            }
            _ => {
                while let Some(op) = operators.pop() {
                    // concat has higher priority than alternation
                    if op == '.' || (c == '|' && op != '(') {
                        postfix.push(op);
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
        postfix.push(c);
    }
    println!("{} => {:?}", expr, postfix);
    postfix
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_TWO: &str = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

    #[test]
    fn example_one() {
        let rules = vec!["0: 1 2", r#"1: "a""#, "2: 1 3 | 3 1", r#"3: "b""#];
        let flattener = Flattener::new(&rules);
        let re = flattener.get_rule("0");
        println!("{}", re);
        assert_eq!("(a.(a.b|b.a))", re);
        let pf = to_postfix(&re);
        println!("{}", pf);
        assert_eq!("aab.ba.|.", pf);
        let re = flattener.flattened();
        println!("{}", re);
        assert_eq!("(a(ab|ba))", re);
    }

    #[test]
    fn example_two() {
        let rules = &r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b""#
            .lines()
            .collect::<Vec<_>>();
        let flattener = Flattener::new(rules);
        let re = flattener.get_rule("0");
        println!("{}", re);
        assert_eq!("(a.((a.a|b.b).(a.b|b.a)|(a.b|b.a).(a.a|b.b)).b)", re);
        let pf = to_postfix(&re);
        println!("{}", pf);
        assert_eq!("aaa.bb.|ab.ba.|.ab.ba.|aa.bb.|.|.b.", pf);
        let re = flattener.flattened();
        println!("{}", re);
        assert_eq!("(a((aa|bb)(ab|ba)|(ab|ba)(aa|bb))b)", re);
        assert_eq!(2, part_one(EXAMPLE_TWO));
    }
}
