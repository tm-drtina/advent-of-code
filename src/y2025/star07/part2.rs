use std::collections::BTreeMap;

pub fn run(input: &str) -> usize {
    let mut lines = input.lines();
    let pos = lines
        .next()
        .unwrap()
        .bytes()
        .position(|b| b == b'S')
        .unwrap();

    let mut positions = BTreeMap::new();
    positions.insert(pos, 1);
    for line in lines {
        let mut new_pos = BTreeMap::new();
        let line = line.as_bytes();

        for (old_pos, count) in positions {
            match line[old_pos] {
                b'^' => {
                    if old_pos > 0 {
                        *new_pos.entry(old_pos - 1).or_default() += count;
                    }
                    if old_pos < line.len() - 1 {
                        *new_pos.entry(old_pos + 1).or_default() += count;
                    }
                }
                b'.' => {
                    *new_pos.entry(old_pos).or_default() += count;
                }
                _ => panic!("Unknown char"),
            }
        }

        positions = new_pos;
    }

    positions.values().sum()
}
