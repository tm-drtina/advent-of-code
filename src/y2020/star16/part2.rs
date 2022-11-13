use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::ops::Range;

pub fn run(input: &str) -> i64 {
    let mut lines = input.lines();

    let mut fields: HashMap<&str, Vec<Range<i32>>> = HashMap::new();

    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let (name, rules) = line.split_once(": ").unwrap();
        let rules = rules
            .split(" or ")
            .map(|rule| {
                let (from_str, to_str) = rule.split_once('-').unwrap();
                let from = from_str.parse().unwrap();
                let to = to_str.parse::<i32>().unwrap() + 1;
                from..to
            })
            .collect();
        fields.insert(name, rules);
    }
    // My ticket
    lines.next(); // title
    let my_ticket: Vec<i32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect(); // values

    lines.next(); // empty line

    let fields_valid: Vec<HashMap<&str, Vec<Range<i32>>>> = my_ticket
        .iter()
        .map(|x| {
            fields
                .iter()
                .filter(|(_key, val)| val.iter().any(|range| range.contains(x)))
                .map(|(key, val)| (*key, val.clone()))
                .collect()
        })
        .collect();

    // Nearby tickets
    lines.next(); //title
    let mut res_fields = lines
        .map(|line| {
            line.split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|ticket| {
            ticket.iter().all(|tf| {
                fields
                    .values()
                    .any(|field| field.iter().any(|range| range.contains(tf)))
            })
        })
        .fold(fields_valid, |acc, ticket| {
            ticket
                .iter()
                .zip(acc)
                .map(|(tf, fields)| {
                    fields
                        .iter()
                        .filter(|(_key, val)| val.iter().any(|range| range.contains(tf)))
                        .map(|(key, val)| (*key, val.clone()))
                        .collect()
                })
                .collect()
        });

    let mut used: HashSet<&str> = HashSet::new();
    let mut indices: HashSet<usize> = HashSet::from_iter(0..res_fields.len());

    let mut res = vec![""; res_fields.len()];

    loop {
        if indices.is_empty() {
            break;
        }
        for i in indices
            .clone()
            .iter()
            .filter(|i| res_fields[**i].len() == 1)
        {
            let val = res_fields[*i].iter().next().unwrap().0;
            used.insert(val);
            indices.remove(i);
            res[*i] = val;
        }
        for i in indices.iter() {
            for x in &used {
                res_fields[*i].remove(x);
            }
        }
    }

    res.iter()
        .zip(my_ticket)
        .filter(|(key, _val)| key.len() >= 9 && &key[0..9] == "departure")
        .map(|(_key, val)| i64::from(val))
        .product()
}
