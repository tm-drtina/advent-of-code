use std::collections::HashMap;

pub fn run(input: &str) -> usize {
    let mut pos_counts = HashMap::<usize, (usize, usize)>::new();
    for s in input.split(',') {
        let entry = pos_counts.entry(s.parse().unwrap()).or_default();
        entry.0 += 1;
        entry.1 += 1;
    }
    let pos_counts = pos_counts;
    let mut steps = 0;

    let mut first_pos = *pos_counts.keys().min().unwrap();
    let mut first = pos_counts[&first_pos];
    let mut last_pos = *pos_counts.keys().max().unwrap();
    let mut last = pos_counts[&last_pos];

    while first_pos != last_pos {
        if first.1 > last.1 {
            steps += last.1;
            last_pos -= 1;
            if let Some(new_last) = pos_counts.get(&last_pos) {
                last = (new_last.0 + last.0, new_last.1 + last.0 + last.1);
            } else {
                last = (last.0, last.0 + last.1);
            }
        } else {
            steps += first.1;
            first_pos += 1;
            if let Some(new_first) = pos_counts.get(&first_pos) {
                first = (new_first.0 + first.0, new_first.1 + first.1 + first.0);
            } else {
                first = (first.0, first.1 + first.0);
            }
        }
    }

    steps
}
