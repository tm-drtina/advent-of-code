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
        min: min.parse().unwrap(),
        max: max.parse().unwrap(),
        char: parts.next().unwrap().chars().next().unwrap(),
        pwd: parts.next().unwrap(),
    }
}

pub fn run(input: &str) -> usize {
    input
        .lines()
        .map(parse_line)
        .filter(|entry| {
            let count = entry.pwd.chars().filter(|c| *c == entry.char).count();
            count >= entry.min && count <= entry.max
        })
        .count()
}
