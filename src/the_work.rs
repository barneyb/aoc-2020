use aoc_2020::read_input;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ops::RangeInclusive;

pub fn the_work() {
    let s = read_input();
    let notes = load_notes(&s);
    println!("{}", part_one(&notes));
    println!("{}", part_two(&notes)); // 603657501217 is low
}

fn load_notes(input: &str) -> Notes {
    enum ParseMode {
        Fields,
        MyTicket,
        NearbyTickets,
    }
    use ParseMode::*;

    let mut fields = Vec::new();
    let mut my_ticket = Vec::new();
    let mut nearby_tickets = Vec::new();
    let mut mode = Fields;
    for line in input.lines().map(|l| l.trim()) {
        if line.is_empty() {
            continue;
        } else if line == "your ticket:" {
            mode = MyTicket;
            continue;
        } else if line == "nearby tickets:" {
            mode = NearbyTickets;
            continue;
        }
        match mode {
            Fields => {
                let ci = line.find(':').expect("no colon?!");
                let mut f = Field::new(&line[0..ci].trim());
                for spec in line[(ci + 1)..].trim().split("or").map(|r| r.trim()) {
                    let parts = spec
                        .split("-")
                        .map(|n| n.parse().unwrap())
                        .collect::<Vec<_>>();
                    f.ranges.push(RangeInclusive::new(parts[0], parts[1]))
                }
                fields.push(f);
            }
            MyTicket => {
                my_ticket = line.split(",").map(|s| s.parse().unwrap()).collect();
            }
            NearbyTickets => {
                nearby_tickets.push(line.split(",").map(|s| s.parse().unwrap()).collect());
            }
        }
    }
    Notes {
        fields,
        my_ticket,
        nearby_tickets,
    }
}

fn part_one(notes: &Notes) -> usize {
    let mut sum = 0;
    for n in notes.nearby_tickets.iter().flatten() {
        if notes.fields.iter().any(|f| f.is_allowed(n)) {
            continue;
        }
        sum += n;
    }
    sum
}

fn part_two(notes: &Notes) -> usize {
    let fields = find_field_order(&notes);
    fields
        .iter()
        .zip(&notes.my_ticket)
        .filter(|(&f, _)| f.label.starts_with("departure"))
        .fold(1, |product, (&f, &n)| {
            println!("{} => {} * {} = {}", f.label, n, product, product * n);
            product * n
        })
}

fn find_field_order<'a>(notes: &'a Notes) -> Vec<&'a Field<'a>> {
    let mut field_map = Vec::with_capacity(notes.fields.len());
    for _ in 0..notes.fields.len() {
        let mut v = HashSet::with_capacity(notes.fields.len());
        for f in notes.fields.iter() {
            v.insert(f);
        }
        field_map.push(v);
    }

    let valid_tickets = notes
        .nearby_tickets
        .iter()
        .filter(|t| {
            t.iter()
                .all(|n| notes.fields.iter().any(|f| f.is_allowed(n)))
        })
        .collect::<Vec<_>>();

    for t in valid_tickets {
        for (i, n) in t.iter().enumerate() {
            field_map[i].retain(|f| f.is_allowed(n));
        }
    }

    let mut field_map = field_map.into_iter().enumerate().collect::<Vec<_>>();
    field_map.sort_by_key(|(_, fs)| fs.len());

    let mut result: Vec<(usize, &Field)> = Vec::with_capacity(notes.fields.len());
    for (idx, mut fs) in field_map {
        for &(_, f) in result.iter() {
            fs.remove(f);
        }
        assert_eq!(1, fs.len());
        result.push((idx, fs.iter().next().unwrap()));
    }

    println!(
        "{:?}",
        result.iter().map(|&(_, f)| f.label).collect::<Vec<_>>()
    );
    result.sort_by_key(|&(idx, _)| idx);

    result.iter().map(|(_, f)| *f).collect()
}

#[derive(Debug, Eq, PartialEq)]
struct Notes<'a> {
    fields: Vec<Field<'a>>,
    my_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

#[derive(Debug, Eq, PartialEq)]
struct Field<'a> {
    label: &'a str,
    ranges: Vec<RangeInclusive<usize>>,
}

impl Field<'_> {
    fn new(label: &str) -> Field {
        Field {
            label,
            ranges: Vec::new(),
        }
    }

    fn is_allowed(&self, n: &usize) -> bool {
        self.ranges.iter().any(|r| r.contains(n))
    }
}

impl Hash for Field<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.label.hash(state);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_ONE: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    fn load_example_one() -> Notes<'static> {
        let mut fields = Vec::new();
        let mut f = Field::new("class");
        f.ranges.push(1..=3);
        f.ranges.push(5..=7);
        fields.push(f);
        let mut f = Field::new("row");
        f.ranges.push(6..=11);
        f.ranges.push(33..=44);
        fields.push(f);
        let mut f = Field::new("seat");
        f.ranges.push(13..=40);
        f.ranges.push(45..=50);
        fields.push(f);

        let my_ticket = vec![7, 1, 14];

        let nearby_tickets = vec![
            vec![7, 3, 47],
            vec![40, 4, 50],
            vec![55, 2, 20],
            vec![38, 6, 12],
        ];

        Notes {
            fields,
            my_ticket,
            nearby_tickets,
        }
    }

    #[test]
    fn test_load_notes() {
        assert_eq!(load_example_one(), load_notes(EXAMPLE_ONE))
    }

    #[test]
    fn example_one() {
        let notes = load_example_one();
        assert_eq!(71, part_one(&notes));
        let notes = load_notes(&EXAMPLE_ONE);
        assert_eq!(71, part_one(&notes));

        assert_eq!(
            1 * 14,
            find_field_order(&notes)
                .iter()
                .zip(&notes.my_ticket)
                .filter(|(&f, _)| f.label.contains('a'))
                .fold(1, |product, (_, n)| product * n)
        );
    }

    const EXAMPLE_TWO: &str = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
13,12,11

nearby tickets:
18,9,3
5,1,15
9,14,5
";

    #[test]
    fn example_two() {
        let notes = load_notes(&EXAMPLE_TWO);
        let fields = find_field_order(&notes);
        assert_eq!(
            vec!["seat", "class", "row"],
            fields.iter().map(|f| f.label).collect::<Vec<_>>()
        );

        assert_eq!(
            12 * 13,
            fields
                .iter()
                .zip(&notes.my_ticket)
                .filter(|(&f, _)| f.label.contains('a'))
                .fold(1, |product, (_, n)| product * n)
        );

        assert_eq!(
            11,
            fields
                .iter()
                .zip(&notes.my_ticket)
                .filter(|(&f, _)| f.label.contains('o'))
                .fold(1, |product, (_, n)| product * n)
        );
    }
}
