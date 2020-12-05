use std::str::FromStr;

#[derive(Debug)]
struct Policy {
    min: usize,
    max: usize,
    char: char,
}

impl Policy {
    fn is_valid(&self, pw: &str) -> bool {
        let first = self.test_char(pw, self.min - 1);
        let last = self.test_char(pw, self.max - 1);
        first ^ last
    }

    fn test_char(&self, pw: &str, i: usize) -> bool {
        pw.len() > i && pw.chars().nth(i).expect("failed to get char") == self.char
    }
}

impl FromStr for Policy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let di = s.find('-').expect("failed to find dash");
        let si = s.find(' ').expect("failed to find space");
        Ok(Policy {
            min: s[0..di].parse().expect("failed to parse min"),
            max: s[(di + 1)..si].parse().expect("failed to parse max"),
            char: s[(si + 1)..s.len()]
                .chars()
                .next()
                .expect("failed to get policy char"),
        })
    }
}

#[derive(Debug)]
pub struct Record {
    policy: Policy,
    password: String,
}

impl Record {
    pub fn is_valid(&self) -> bool {
        self.policy.is_valid(&self.password)
    }
}

impl FromStr for Record {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ci = s.find(':').expect("failed to find colon");
        Ok(Record {
            policy: s[0..ci].parse().expect("failed to parse policy"),
            password: String::from(s[(ci + 1)..s.len()].trim()),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_policy_parse() {
        let p = "3-4 q".parse::<Policy>().unwrap();
        assert_eq!(p.min, 3);
        assert_eq!(p.max, 4);
        assert_eq!(p.char, 'q');
    }

    #[test]
    fn test_record_parse() {
        let r = "3-4 q: qqqn".parse::<Record>().unwrap();
        assert_eq!(r.password, "qqqn");
        let p = r.policy;
        assert_eq!(p.min, 3);
        assert_eq!(p.max, 4);
        assert_eq!(p.char, 'q');
    }
}
