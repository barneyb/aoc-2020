use aoc_2020::read_input;
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;

pub fn the_work() {
    let s = read_input();
    println!("{:?}", both_parts(&s));
}

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut blocks = input.split("\n\n").map(|b| b.lines().collect::<Vec<_>>());
    (blocks.next().unwrap(), blocks.next().unwrap())
}

fn get_rules(input: &str) -> Vec<&str> {
    parse(input).0
}

fn part_one(input: &str) -> usize {
    let (rule_list, strings) = parse(input);
    let mut rule = Flattener::new(&rule_list).flattened();
    rule.insert(0, '^');
    rule.push('$');
    let re = Regex::new(&rule).unwrap();
    strings.iter().filter(|l| re.is_match(l)).count()
}

fn both_parts(input: &str) -> (usize, usize) {
    let (rule_list, strings) = parse(input);
    let flattener = Flattener::new(&rule_list);
    /*
    0: 8 11
    8: 42
    11: 42 31

    0: 42 42 31
     */
    let rule_42 = flattener.get_rule("42");
    let rule_31 = flattener.get_rule("31");
    let rule_0 = format!("^{}{}{}$", rule_42, rule_42, rule_31);
    let re = Regex::new(&rule_0).unwrap();
    let one = strings.iter().filter(|l| re.is_match(l)).count();

    /*
    0: 8 11
    8: 42 | 42 8
    11: 42 31 | 42 11 31

    0: 42+ [42 <nest> 31]+
    0: 42{m} 31{n} where m > n
     */

    let re_42 = Regex::new(&format!("^{}", rule_42)).unwrap();
    let re_31 = Regex::new(&format!("^{}", rule_31)).unwrap();
    let two = strings
        .iter()
        .filter(|&&l| {
            let mut c42 = 0;
            let mut c31 = 0;
            let mut start = 0;
            while let Some(m) = re_42.find(&l[start..]) {
                start += m.end();
                c42 += 1;
            }
            if c42 < 2 {
                // need at least two 42s
                return false;
            }
            while let Some(m) = re_31.find(&l[start..]) {
                start += m.end();
                c31 += 1;
            }
            if c31 < 1 {
                // need at least one 31
                return false;
            }
            c42 > c31 && l[start..].len() == 0
        })
        .count();

    (one, two)
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

    const EXAMPLE_THREE: &str = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

    #[test]
    fn example_one() {
        let rules = vec!["0: 1 2", r#"1: "a""#, "2: 1 3 | 3 1", r#"3: "b""#];
        let flattener = Flattener::new(&rules);
        let re = flattener.flattened();
        println!("{}", re);
        assert_eq!("(a(ab|ba))", re);
    }

    #[test]
    fn example_two() {
        let rules = get_rules(EXAMPLE_TWO);
        let flattener = Flattener::new(&rules);
        let re = flattener.flattened();
        println!("{}", re);
        assert_eq!("(a((aa|bb)(ab|ba)|(ab|ba)(aa|bb))b)", re);
        assert_eq!(2, part_one(EXAMPLE_TWO));
    }

    #[test]
    fn example_three() {
        assert_eq!((3, 12), both_parts(EXAMPLE_THREE));
    }
}
