use aoc_2020::read_input;
use std::ops::RangeInclusive;

pub fn the_work() {
    let s = read_input();
    let notes = load_notes(&s);
    println!("{}", part_one(&notes));
}

enum ParseMode {
    Fields,
    MyTicket,
    NearbyTickets,
}
use ParseMode::*;

fn load_notes(input: &str) -> Notes {
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
    'num_loop: for n in notes.nearby_tickets.iter().flatten() {
        for r in notes.fields.iter().flat_map(|f| f.ranges.iter()) {
            if r.contains(n) {
                continue 'num_loop;
            }
        }
        sum += n;
    }
    sum
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
        let notes = load_example_one();
        assert_eq!(71, part_one(&notes));
    }
}
