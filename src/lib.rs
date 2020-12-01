use std::fs;

pub fn read_input() -> String {
    fs::read_to_string("input.txt").unwrap().trim().to_string()
}

pub fn read_lines<T, F>(f: F) -> Vec<T>
where
    F: Fn(&str) -> T,
{
    read_input().lines().map(f).collect::<Vec<T>>()
}
