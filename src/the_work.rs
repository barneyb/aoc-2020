use aoc_2020 as aoc;

pub fn the_work() {
    let groups = aoc::unwrap_paragraphs(&aoc::read_input());
    let a = 'a' as usize;
    let part_one: usize = groups
        .iter()
        .map(|g| {
            let mut map = [0; 26];
            for p in g.split(' ') {
                for c in p.chars() {
                    map[(c as usize) - a] += 1;
                }
            }
            let pc = g.split(' ').count();
            map.iter().filter(|&&c| c == pc).count()
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
