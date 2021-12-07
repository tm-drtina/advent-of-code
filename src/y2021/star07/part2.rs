use std::collections::{HashMap, hash_map::Entry};

pub fn run2(input: &str) -> i64 {
    let pos: Vec<i64> = input.split(',').map(|s| s.parse().unwrap()).collect();
    let sum: i64 = pos.iter().copied().map(|v| v*v).sum();
    let avg = (sum as f64/pos.len() as f64/pos.len() as f64).round() as i64;

    pos.into_iter().map(|n| {
        let dist = (n - avg).abs();
        dist * (dist + 1) / 2
    }).sum()
}

pub fn run(input: &str) -> usize {
    let mut pos_counts = HashMap::<usize, (usize, usize)>::new();
    for s in input.split(',') {
        let entry = pos_counts.entry(s.parse().unwrap()).or_default();
        entry.0 += 1;
        entry.1 += 1;
    }
    let mut keys: Vec<usize> = pos_counts.keys().copied().collect();
    keys.sort();
    let mut keys = &mut keys[..];
    let mut steps = 0;


    while keys.len() > 1 {
        let first_pos = *keys.first().unwrap();
        let first = pos_counts[&first_pos];
        let last_pos = *keys.last().unwrap();
        let last = pos_counts[&last_pos];

        if first.1 > last.1 {
            steps += last.1;
            match pos_counts.entry(last_pos - 1) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().0 += last.0;
                    entry.get_mut().1 += last.0 + last.1;
                    let index = keys.len() - 1;
                    keys = &mut keys[..index];
                },
                Entry::Vacant(entry) => {
                    entry.insert((last.0, last.1 + last.0));
                    *keys.last_mut().unwrap() -= 1;
                }
            };
            pos_counts.remove(&last_pos);
        } else {
            steps += first.1;
            match pos_counts.entry(first_pos + 1) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().0 += first.0;
                    entry.get_mut().1 += first.0 + first.1;
                    keys = &mut keys[1..];
                },
                Entry::Vacant(entry) => {
                    entry.insert((first.0, first.1 + first.0));
                    *keys.first_mut().unwrap() += 1;
                }
            };
            pos_counts.remove(&first_pos);
        }
    }

    steps
}