use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Hash, Eq, PartialEq, Debug, EnumIter)]
pub enum Field {
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
    fn is_valid(&self, v: &str) -> bool {
        match self {
            Field::BirthYear => match v.parse::<i32>() {
                Ok(i) => i >= 1920 && i <= 2002,
                Err(_) => false,
            },
            Field::IssueYear => match v.parse::<i32>() {
                Ok(i) => i >= 2010 && i <= 2020,
                Err(_) => false,
            },
            Field::ExpirationYear => match v.parse::<i32>() {
                Ok(i) => i >= 2020 && i <= 2030,
                Err(_) => false,
            },
            Field::Height => match &v[(v.len() - 2)..v.len()] {
                "cm" => match v[0..3].parse::<i32>() {
                    Ok(i) => i >= 150 && i <= 193,
                    Err(_) => false,
                },
                "in" => match v[0..2].parse::<i32>() {
                    Ok(i) => i >= 59 && i <= 76,
                    Err(_) => false,
                },
                _ => false,
            },
            Field::HairColor => COLOR_RE.is_match(v),
            Field::EyeColor => match v {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            },
            Field::PassportID => PID_RE.is_match(v),
            Field::CountryID => true,
        }
    }
}

pub struct Passport {
    fields: HashMap<Field, String>,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            fields: HashMap::new(),
        }
    }

    fn get_field(&self, f: &Field) -> Option<&String> {
        self.fields.get(f)
    }

    #[allow(dead_code)]
    fn has_field(&self, f: &Field) -> bool {
        self.fields.contains_key(f)
    }

    fn set_field(&mut self, f: Field, value: String) -> Option<String> {
        self.fields.insert(f, value)
    }

    pub fn is_valid(&self) -> bool {
        for f in Field::iter() {
            if f == Field::CountryID {
                continue;
            }
            match self.get_field(&f) {
                None => return false,
                Some(v) => {
                    if !f.is_valid(v) {
                        return false;
                    }
                }
            }
        }
        true
    }
}

impl<'a> FromStr for Passport {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pp = Passport::new();
        s.trim()
            .split(" ")
            .map(|s| s.trim())
            .filter(|s| s.len() > 0)
            .map(|p| p.split(":").map(|s| s.trim()))
            .for_each(|mut p| {
                pp.set_field(
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
                    p.next().unwrap().to_string(),
                );
            });
        Ok(pp)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc_2020 as aoc;

    #[test]
    fn test_is_valid() {
        let mut pp = Passport::new();
        assert!(!pp.is_valid());
        pp.set_field(Field::BirthYear, "1980".to_string());
        assert!(!pp.is_valid());
        pp.set_field(Field::IssueYear, "2015".to_string());
        assert!(!pp.is_valid());
        pp.set_field(Field::ExpirationYear, "2025".to_string());
        assert!(!pp.is_valid());
        pp.set_field(Field::Height, "192cm".to_string());
        assert!(!pp.is_valid());
        pp.set_field(Field::HairColor, "#123abc".to_string());
        assert!(!pp.is_valid());
        pp.set_field(Field::EyeColor, "grn".to_string());
        assert!(!pp.is_valid());
        pp.set_field(Field::PassportID, "123456789".to_string());
        assert!(pp.is_valid());
        pp.set_field(Field::CountryID, "garbage".to_string());
        assert!(pp.is_valid());
    }

    #[test]
    fn test_parse() {
        let input = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929";
        let pp = input.parse::<Passport>().unwrap();
        assert!(pp.has_field(&Field::EyeColor));
        assert!(!pp.has_field(&Field::Height));
    }

    #[test]
    fn test_example_input() {
        let input = String::from(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
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
        let passports = aoc::unwrap_paragraphs(&input);
        assert_eq!(4, passports.len());
        for p in passports.iter() {
            println!("{:?}", p);
            assert!(!p.contains(&String::from("\n")));
        }
        assert_eq!(
            2,
            passports
                .iter()
                .map(|s| s.parse::<Passport>().unwrap())
                .filter(|p| p.is_valid())
                .count()
        );
    }
}
