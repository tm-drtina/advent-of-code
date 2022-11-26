use std::collections::HashMap;

struct Bag<'a> {
    name: &'a str,
    count: i32,
}

fn parse_line(line: &str) -> (&str, Vec<Bag<'_>>) {
    let (raw_outer, raw_inner) = line[0..line.len() - 1].split_once(" contain ").unwrap();
    (
        &raw_outer[0..raw_outer.len() - 5],
        match raw_inner {
            "no other bags" => Vec::new(),
            _ => raw_inner
                .split(", ")
                .map(|bag_with_count| {
                    let (count, color_bag) = bag_with_count.split_once(' ').unwrap();
                    let color = color_bag.rsplit_once(' ').unwrap().0;
                    Bag {
                        name: color,
                        count: count.parse().unwrap(),
                    }
                })
                .collect(),
        },
    )
}

fn count_rec(map: &HashMap<&str, Vec<Bag<'_>>>, bag: &Bag<'_>) -> i32 {
    map.get(bag.name)
        .map_or(0, |bags| {
            bags.iter()
                .map(|bag| bag.count + bag.count * count_rec(map, bag))
                .sum()
        })
}

pub fn run(input: &str) -> i32 {
    let m: HashMap<&str, Vec<Bag<'_>>> = input.lines().map(parse_line).collect();
    count_rec(
        &m,
        &Bag {
            name: "shiny gold",
            count: 1,
        },
    )
}
