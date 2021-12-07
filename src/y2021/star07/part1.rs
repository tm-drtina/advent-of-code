use std::collections::HashMap;

pub fn run(input: &str) -> usize {
    let mut pos_counts = HashMap::<usize, usize>::new();
    for s in input.split(',') {
        *pos_counts.entry(s.parse().unwrap()).or_default() += 1;
    }
    let mut keys: Vec<usize> = pos_counts.keys().copied().collect();
    keys.sort();
    let mut keys = &keys[..];
    let mut steps = 0;

    while keys.len() > 1 {
        let first_pos = keys.first().unwrap();
        let first = pos_counts[first_pos];
        let last_pos = keys.last().unwrap();
        let last = pos_counts[last_pos];

        if first > last {
            let new_last_pos = keys[keys.len() - 2];
            *pos_counts.get_mut(&new_last_pos).unwrap() += last;
            keys = &keys[..keys.len()-1];
            steps += (last_pos - new_last_pos) * last;
        } else {
            let new_first_pos = keys[1];
            *pos_counts.get_mut(&new_first_pos).unwrap() += first;
            keys = &keys[1..];
            steps += (new_first_pos - first_pos) * first;
        }
    }

    steps
}
