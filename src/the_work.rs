use aoc_2020::read_input;
use std::cell::RefCell;
use std::collections::HashMap;

pub fn the_work() {
    let s = read_input();
    println!("{:?}", s.len());
}

fn part_one(input: &str) -> usize {
    let mut lines = input.lines();
    let rule_list = lines
        .by_ref()
        .take_while(|&l| !l.is_empty())
        .collect::<Vec<_>>();
    let rule = Flattener::new(&rule_list).flattened();
    let messages = lines.collect::<Vec<_>>();
    println!(
        "{:?}\n-------\n{:?}\n-------\n{:?}",
        rule, rule_list, messages
    );
    42
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
        self.get_rule("0")
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
                            _ => result.push_str(&self.get_rule(t)),
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
        assert_eq!("(a(ab|ba))", Flattener::new(&rules).flattened());
    }

    #[test]
    fn example_two() {
        let s = EXAMPLE_TWO.trim();
        assert_eq!(
            "(a((aa|bb)(ab|ba)|(ab|ba)(aa|bb))b)",
            Flattener::new(
                &r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b""#
                    .lines()
                    .collect::<Vec<_>>()
            )
            .flattened()
        );
    }
}
