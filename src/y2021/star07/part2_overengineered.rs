use std::collections::HashMap;

fn move_crabs(from_pos: (usize, usize), to_pos: (usize, usize), dist: usize) -> (usize, (usize, usize)) {
    debug_assert_ne!(0, dist);

    let cost_first_step = from_pos.1;
    let cost_last_step = from_pos.1 + (dist - 1) * from_pos.0;
    (
        dist * (cost_first_step + cost_last_step) / 2,
        (from_pos.0 + to_pos.0, from_pos.1 + to_pos.1 + dist * from_pos.0)
    )
}

pub fn run(input: &str) -> usize {
    let mut pos_counts = HashMap::<usize, (usize, usize)>::new();
    for s in input.split(',') {
        let entry = pos_counts.entry(s.parse().unwrap()).or_default();
        entry.0 += 1;
        entry.1 += 1;
    }
    let pos_counts = pos_counts;
    let mut steps = 0;

    let mut keys: Vec<usize> = pos_counts.keys().copied().collect();
    keys.sort_unstable();
    let mut keys = &mut keys[..];

    let mut first_pos = *keys.first().unwrap();
    let mut first = pos_counts[&first_pos];
    let mut last_pos = *keys.last().unwrap();
    let mut last = pos_counts[&last_pos];

    if keys.len() > 2 {
        let mut new_first_pos = keys[1];
        let mut new_last_pos = keys[keys.len() - 2];
        let (mut cost_first, mut new_first) = move_crabs(first, *pos_counts.get(&new_first_pos).unwrap(), new_first_pos - first_pos);
        let (mut cost_last, mut new_last) = move_crabs(last, *pos_counts.get(&new_last_pos).unwrap(), last_pos - new_last_pos);
        while keys.len() > 2 {
            if cost_first > cost_last {
                steps += cost_last;
                last = new_last;
                last_pos = new_last_pos;
                let index = keys.len() - 1;
                keys = &mut keys[..index];
                if keys.len() > 2 {
                    new_first_pos = keys[1];
                    let (tmp_cost_first, tmp_new_first) = move_crabs(first, *pos_counts.get(&new_first_pos).unwrap(), new_first_pos - first_pos);
                    cost_first = tmp_cost_first;
                    new_first = tmp_new_first;
                }
            } else {
                steps += cost_first;
                first = new_first;
                first_pos = new_first_pos;
                keys = &mut keys[1..];
                if keys.len() > 2 {
                    new_last_pos = keys[keys.len() - 2];
                    let (tmp_cost_last, tmp_new_last) = move_crabs(last, *pos_counts.get(&new_last_pos).unwrap(), last_pos - new_last_pos);
                    cost_last = tmp_cost_last;
                    new_last = tmp_new_last;
                }
            }
        }
    }
    while last_pos > first_pos {
        let mut dist = (last_pos - first_pos) / 2;
        if dist == 0 {
            dist = 1; // handle last step
        }

        let (cost_first, new_first) = move_crabs(first, (0, 0), dist);
        let (cost_last, new_last) = move_crabs(last, (0, 0), dist);
        if cost_first > cost_last {
            steps += cost_last;
            last = new_last;
            last_pos -= dist;
        } else {
            steps += cost_first;
            first = new_first;
            first_pos += dist;
        }
    }
    

    steps
}
