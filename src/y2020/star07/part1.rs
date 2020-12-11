use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn parse_line(line: &str) -> (&str, Vec<&str>) {
    let (raw_outer, raw_inner) = line[0..line.len() - 1].split_once(" contain ").unwrap();
    (
        &raw_outer[0..raw_outer.len() - 5],
        raw_inner
            .split(", ")
            .map(|bag_with_count| {
                let (_count, color_bag) = bag_with_count.split_once(" ").unwrap();
                let color = color_bag.rsplit_once(" ").unwrap().0;
                color
            })
            .collect(),
    )
}

pub fn run(input: &str) -> i32 {
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();
    let empty_set: HashSet<&str> = HashSet::new();

    for (outer_bag, inner_bags) in input.lines().map(|line| parse_line(line)) {
        for inner_bag in inner_bags {
            map.entry(inner_bag).or_default().insert(outer_bag);
        }
    }

    let mut set: HashSet<&str> = HashSet::new();
    set.insert("shiny gold");

    loop {
        let new = set
            .iter()
            .flat_map(|str| map.get(str).unwrap_or_else(|| &empty_set))
            .collect_vec();
        if !new.iter().fold(false, |acc, str| set.insert(str) || acc) {
            break;
        }
    }

    set.len() as i32 - 1
}
