#[macro_use]
extern crate lazy_static;

use aoc_2020 as aoc;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use regex::Regex;

fn main() {
    let input = aoc::read_input();
    let passports = break_on_blank_lines(&input);
    // 257 is high
    println!("{}", passports.iter().map(|s| parse(s)).filter(|p| is_valid(&p)).count());
}

#[derive(Hash, Eq, PartialEq, Debug, EnumIter)]
enum Field {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportID,
    CountryID,
}

lazy_static! {
    static ref COLOR_RE: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
    static ref PID_RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
}

impl Field {

    fn is_valid(&self, v:&str) -> bool {
        match self {
            Field::BirthYear => {
                match v.parse::<i32>() {
                    Ok(i) => i >= 1920 && i <= 2002,
                    Err(_) => false,
                }
            },
            Field::IssueYear => {
                match v.parse::<i32>() {
                    Ok(i) => i >= 2010 && i <= 2020,
                    Err(_) => false,
                }
            },
            Field::ExpirationYear => {
                match v.parse::<i32>() {
                    Ok(i) => i >= 2020 && i <= 2030,
                    Err(_) => false,
                }
            },
            Field::Height => {
                match &v[(v.len() - 2)..v.len()] {
                    "cm" => match v[0..3].parse::<i32>() {
                        Ok(i) => i >= 150 && i <= 193,
                        Err(_) => false,
                    },
                    "in" => match v[0..2].parse::<i32>() {
                        Ok(i) => i >= 59 && i <= 76,
                        Err(_) => false,
                    },
                    _ => false,
                }
            },
            Field::HairColor => {
                COLOR_RE.is_match(v)
            },
            Field::EyeColor => {
                match v {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                    _ => false,
                }
            },
            Field::PassportID => {
                PID_RE.is_match(v)
            },
            Field::CountryID => true,
        }
    }

}

fn break_on_blank_lines(input: &str) -> Vec<String> {
    input.split("\n\n").map(|s| s.replace('\n', " ")).collect()
}

type Passport<'a> = HashMap<Field, &'a str>;

fn parse(input: &str) -> Passport {
    let mut passport = HashMap::new();
    input.trim()
        .split(" ")
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .map(|p| p.split(":").map(|s| s.trim()))
        .for_each(|mut p| {
            passport.insert(
                match p.next().unwrap() {
                    "byr" => Field::BirthYear,
                    "iyr" => Field::IssueYear,
                    "eyr" => Field::ExpirationYear,
                    "hgt" => Field::Height,
                    "hcl" => Field::HairColor,
                    "ecl" => Field::EyeColor,
                    "pid" => Field::PassportID,
                    "cid" => Field::CountryID,
                    k => panic!("Unrecognized '{}' key!", k),
                },
                p.next().unwrap()
            );
        });
    passport
}

fn is_valid(passport: &Passport) -> bool {
    for f in Field::iter() {
        if f == Field::CountryID {
            continue;
        }
        match passport.get(&f) {
            None => return false,
            Some(&v) => if !f.is_valid(v) { return false },
        }
    }
    true
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_is_valid() {
        let mut pp = HashMap::new();
        assert!(!is_valid(&pp));
        pp.insert(Field::BirthYear, "1980");
        assert!(!is_valid(&pp));
        pp.insert(Field::IssueYear, "2015");
        assert!(!is_valid(&pp));
        pp.insert(Field::ExpirationYear, "2025");
        assert!(!is_valid(&pp));
        pp.insert(Field::Height, "192cm");
        assert!(!is_valid(&pp));
        pp.insert(Field::HairColor, "#123abc");
        assert!(!is_valid(&pp));
        pp.insert(Field::EyeColor, "grn");
        assert!(!is_valid(&pp));
        pp.insert(Field::PassportID, "123456789");
        assert!(is_valid(&pp));
        pp.insert(Field::CountryID, "garbage");
        assert!(is_valid(&pp));
    }

    #[test]
    fn test_parse() {
        let input = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929";
        let pp = parse(input);
        assert_eq!(7, pp.len());
        assert!(pp.contains_key(&Field::EyeColor));
        assert!(!pp.contains_key(&Field::Height));
    }

    #[test]
    fn test_example_input() {
        let input = String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
",
        );
        let passports = break_on_blank_lines(&input);
        assert_eq!(4, passports.len());
        for p in passports.iter() {
            println!("{:?}", p);
            assert!(!p.contains(&String::from("\n")));
        }
        assert_eq!(2, passports.iter().map(|s| parse(s)).filter(|p| is_valid(&p)).count());
    }
}
