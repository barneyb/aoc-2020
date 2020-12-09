use aoc_2020 as aoc;
use std::str::FromStr;

pub fn the_work() {
    let instructions = aoc::read_lines(|l| l.parse::<Ins>().unwrap());
    println!("{:?}", evaluate(&instructions));
}

#[derive(Eq, PartialEq, Debug)]
enum Res<T> {
    Loop(T),
    Exit(T),
}

fn evaluate(instructions: &Vec<Ins>) -> Res<i32> {
    let mut ip = 0;
    let mut accumulator = 0;
    let mut map = Vec::with_capacity(instructions.len());
    for _ in 0..instructions.len() {
        map.push(false);
    }
    loop {
        if ip >= instructions.len() {
            return Res::Exit(accumulator);
        }
        if let Some(true) = map.get(ip) {
            return Res::Loop(accumulator);
        }
        map[ip] = true;
        let ins = &instructions[ip];
        match ins.code {
            Nop => ip += 1,
            Acc => {
                accumulator += ins.param;
                ip += 1;
            }
            Jmp => ip = ((ip as i32) + ins.param) as usize,
        }
    }
}

enum OpCode {
    Nop,
    Acc,
    Jmp,
}
use OpCode::*;

impl FromStr for OpCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nop" => Ok(Nop),
            "acc" => Ok(Acc),
            "jmp" => Ok(Jmp),
            _ => Err(format!("Unrecognized '{}' opcode", s)),
        }
    }
}

struct Ins {
    code: OpCode,
    param: i32,
}

impl FromStr for Ins {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(' ');
        Ok(Ins {
            code: s.next().unwrap().parse().unwrap(),
            param: s.next().unwrap().parse().unwrap(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = "
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    const EXAMPLE_INPUT_WITH_EXIT: &str = "
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
nop -4
acc +6";

    #[test]
    fn example_one() {
        let instructions = EXAMPLE_INPUT
            .trim()
            .lines()
            .map(|l| l.parse().unwrap())
            .collect::<Vec<Ins>>();
        assert_eq!(Res::Loop(5), evaluate(&instructions));
    }

    #[test]
    fn example_two_exit() {
        let instructions = EXAMPLE_INPUT_WITH_EXIT
            .trim()
            .lines()
            .map(|l| l.parse().unwrap())
            .collect::<Vec<Ins>>();
        assert_eq!(Res::Exit(8), evaluate(&instructions));
    }
}
