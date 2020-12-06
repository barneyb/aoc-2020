use aoc_2020 as aoc;

pub fn the_work() {
    let groups = aoc::unwrap_paragraphs(&aoc::read_input());
    let a = 'a' as usize;
    let part_one: usize = groups
        .iter()
        .map(|g| {
            let mut map = [false; 26];
            for c in g.chars() {
                if c == ' ' {
                    continue;
                }
                map[(c as usize) - a] = true;
            }
            map.iter().filter(|&&b| b).count()
        })
        .sum();
    println!("{}", part_one)
}

#[cfg(test)]
mod test {
    #[test]
    fn lakjdsf() {
        println!("{}", ('c' as usize) - ('a' as usize))
    }
}
