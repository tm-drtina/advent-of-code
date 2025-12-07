use std::collections::BTreeSet;

pub fn run(input: &str) -> usize {
    let mut lines = input.lines();
    let pos = lines
        .next()
        .unwrap()
        .bytes()
        .position(|b| b == b'S')
        .unwrap();

    let mut positions = BTreeSet::new();
    positions.insert(pos);
    let mut count = 0;
    for line in lines {
        let mut new_pos = BTreeSet::new();
        let line = line.as_bytes();

        for old_pos in positions {
            match line[old_pos] {
                b'^' => {
                    count += 1;
                    if old_pos > 0 {
                        new_pos.insert(old_pos - 1);
                    }
                    if old_pos < line.len() - 1 {
                        new_pos.insert(old_pos + 1);
                    }
                }
                b'.' => {
                    new_pos.insert(old_pos);
                }
                _ => panic!("Unknown char"),
            }
        }

        positions = new_pos;
    }

    count
}
