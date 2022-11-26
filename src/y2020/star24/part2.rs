use std::collections::{HashMap, HashSet};

fn to_coords(line: &str) -> (i32, i32) {
    let mut point = (0, 0);
    let mut chars = line.chars();

    while let Some(ch) = chars.next() {
        match ch {
            'e' => point.1 += 1,
            'w' => point.1 -= 1,
            's' => {
                point.0 += 1;
                if chars.next().unwrap() == 'w' {
                    point.1 -= 1;
                }
            }
            'n' => {
                point.0 -= 1;
                if chars.next().unwrap() == 'e' {
                    point.1 += 1;
                }
            }
            _ => {}
        }
    }

    point
}

fn neighbors(p: (i32, i32)) -> Vec<(i32, i32)> {
    vec![
        (p.0, p.1 + 1),
        (p.0, p.1 - 1),
        (p.0 + 1, p.1),
        (p.0 + 1, p.1 - 1),
        (p.0 - 1, p.1 + 1),
        (p.0 - 1, p.1),
    ]
}

fn step(black: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut new_black = HashSet::new();
    let mut white_neighbors: HashMap<(i32, i32), usize> = HashMap::new();
    for &x in black {
        let mut black_neighbors = 0;
        for neighbor in neighbors(x) {
            if black.contains(&neighbor) {
                black_neighbors += 1;
            } else {
                *white_neighbors.entry(neighbor).or_default() += 1;
            }
        }
        if black_neighbors == 1 || black_neighbors == 2 {
            new_black.insert(x);
        }
    }
    white_neighbors
        .iter()
        .filter(|(_, val)| **val == 2)
        .for_each(|(key, _)| {
            new_black.insert(*key);
        });

    new_black
}

pub fn run(input: &str) -> usize {
    let mut black = input
        .lines()
        .map(to_coords)
        .fold(HashSet::new(), |mut acc, val| {
            if acc.contains(&val) {
                acc.remove(&val);
            } else {
                acc.insert(val);
            }
            acc
        });

    for _ in 0..100 {
        black = step(&black);
    }

    black.len()
}
