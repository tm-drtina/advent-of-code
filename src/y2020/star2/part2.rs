use std::str::FromStr;

struct Entry<'a> {
    min: usize,
    max: usize,
    char: char,
    pwd: &'a str,
}

fn parse_line(line: &str) -> Entry {
    let mut parts = line.split(' ');
    let (min, max) = parts.next().unwrap().split_once('-').unwrap();
    Entry {
        min: usize::from_str(min).unwrap(),
        max: usize::from_str(max).unwrap(),
        char: parts.next().unwrap().chars().next().unwrap(),
        pwd: parts.next().unwrap(),
    }
}

pub fn run(input: &str) -> usize {
    input
        .lines()
        .map(|line| parse_line(line))
        .filter(|entry| {
            let match1 = entry.pwd.chars().nth(entry.min - 1).unwrap() == entry.char;
            let match2 = entry.pwd.chars().nth(entry.max - 1).unwrap() == entry.char;
            (match1 || match2) && !(match1 && match2)
        })
        .count()
}
