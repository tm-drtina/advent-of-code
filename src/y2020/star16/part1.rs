use std::ops::Range;
use std::str::FromStr;

pub fn run(input: &str) -> i32 {
    let mut lines = input.lines();

    let mut ranges: Vec<Range<i32>> = Vec::new();

    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let (_field, rules) = line.split_once(": ").unwrap();
        for rule in rules.split(" or ") {
            let (from_str, to_str) = rule.split_once('-').unwrap();
            let from = i32::from_str(from_str).unwrap();
            let to = i32::from_str(to_str).unwrap() + 1;
            ranges.push(from..to);
        }
    }
    // My ticket
    lines.next(); // title
    lines.next(); // values

    lines.next(); // empty line

    // Nearby tickets
    lines.next(); //title
    lines
        .flat_map(|line| line.split(',').collect::<Vec<&str>>())
        .map(|field| i32::from_str(field).unwrap())
        .filter(|field| ranges.iter().all(|range| !range.contains(field)))
        .sum()
}
