use std::collections::HashSet;

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

pub fn run(input: &str) -> usize {
    input
        .lines()
        .map(to_coords)
        .fold(HashSet::new(), |mut acc, val| {
            if acc.contains(&val) {
                acc.remove(&val);
            } else {
                acc.insert(val);
            }
            acc
        })
        .len()
}
