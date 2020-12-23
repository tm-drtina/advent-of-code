use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

fn play_game(mut p: (VecDeque<i32>, VecDeque<i32>)) -> (bool, VecDeque<i32>) {
    let mut h: HashSet<(VecDeque<i32>, VecDeque<i32>)> = HashSet::new();

    while !p.0.is_empty() && !p.1.is_empty() {
        if h.contains(&p) {
            return (true, p.0);
        }
        h.insert(p.clone());

        let c1 = p.0.pop_front().unwrap();
        let c2 = p.1.pop_front().unwrap();

        let p1_wins: bool;

        if c1 as usize <= p.0.len() && c2 as usize <= p.1.len() {
            p1_wins = play_game((
                p.0.iter().take(c1 as usize).copied().collect(),
                p.1.iter().take(c2 as usize).copied().collect(),
            ))
            .0
        } else {
            p1_wins = c1 > c2;
        }

        if p1_wins {
            p.0.push_back(c1);
            p.0.push_back(c2);
        } else {
            p.1.push_back(c2);
            p.1.push_back(c1);
        }
    }
    if p.0.is_empty() {
        (false, p.1)
    } else {
        (true, p.0)
    }
}

fn score(p: VecDeque<i32>) -> i64 {
    let size = p.len();
    p.into_iter()
        .enumerate()
        .map(|(index, value)| (size - index) as i64 * value as i64)
        .sum()
}

pub fn run(input: &str) -> i64 {
    let mut lines = input.lines();

    let mut p1: VecDeque<_> = VecDeque::new();
    let mut p2: VecDeque<_> = VecDeque::new();

    lines.next(); // title
    loop {
        match lines.next() {
            Some("") | None => break,
            Some(line) => p1.push_back(i32::from_str(line).unwrap()),
        }
    }
    lines.next(); // title
    loop {
        match lines.next() {
            Some("") | None => break,
            Some(line) => p2.push_back(i32::from_str(line).unwrap()),
        }
    }

    score(play_game((p1, p2)).1)
}
