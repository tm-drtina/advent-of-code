use std::collections::HashMap;
use std::str::FromStr;

struct Bag<'a> {
    name: &'a str,
    count: i32,
}

fn parse_line(line: &str) -> (&str, Vec<Bag>) {
    let (raw_outer, raw_inner) = line[0..line.len() - 1].split_once(" contain ").unwrap();
    (
        &raw_outer[0..raw_outer.len() - 5],
        match raw_inner {
            "no other bags" => Vec::new(),
            _ => raw_inner
                .split(", ")
                .map(|bag_with_count| {
                    let (count, color_bag) = bag_with_count.split_once(" ").unwrap();
                    let color = color_bag.rsplit_once(" ").unwrap().0;
                    Bag {
                        name: color,
                        count: i32::from_str(count).unwrap(),
                    }
                })
                .collect(),
        },
    )
}

fn count_rec(map: &HashMap<&str, Vec<Bag>>, bag: &Bag) -> i32 {
    map.get(bag.name)
        .map(|bags| {
            bags.iter()
                .map(|bag| bag.count + bag.count * count_rec(map, bag))
                .sum()
        })
        .unwrap_or(0)
}

pub fn run(input: &str) -> i32 {
    let m: HashMap<&str, Vec<Bag>> = input.lines().map(|line| parse_line(line)).collect();
    count_rec(
        &m,
        &Bag {
            name: "shiny gold",
            count: 1,
        },
    )
}
