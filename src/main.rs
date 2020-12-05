use aoc_2020 as aoc;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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
        if !passport.contains_key(&f) {
        println!("{:?} doesn't have {:?}", passport, f);
            return false;
        }
    }
    println!("we good");
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
        pp.insert(Field::BirthYear, "123");
        assert!(!is_valid(&pp));
        pp.insert(Field::IssueYear, "123");
        assert!(!is_valid(&pp));
        pp.insert(Field::ExpirationYear, "123");
        assert!(!is_valid(&pp));
        pp.insert(Field::Height, "123");
        assert!(!is_valid(&pp));
        pp.insert(Field::HairColor, "123");
        assert!(!is_valid(&pp));
        pp.insert(Field::EyeColor, "123");
        assert!(!is_valid(&pp));
        pp.insert(Field::PassportID, "123");
        assert!(is_valid(&pp));
        pp.insert(Field::CountryID, "123");
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
