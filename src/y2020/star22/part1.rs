use std::collections::VecDeque;

fn play_game(mut p1: VecDeque<i32>, mut p2: VecDeque<i32>) -> VecDeque<i32> {
    while !p1.is_empty() && !p2.is_empty() {
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();
        if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }
    if p1.is_empty() {
        p2
    } else {
        p1
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
    lines.next(); // title

    let mut p1: VecDeque<_> = VecDeque::new();
    let mut p2: VecDeque<_> = VecDeque::new();
    loop {
        match lines.next() {
            Some("") | None => break,
            Some(line) => p1.push_back(line.parse().unwrap()),
        }
    }
    lines.next(); // title
    loop {
        match lines.next() {
            Some("") | None => break,
            Some(line) => p2.push_back(line.parse().unwrap()),
        }
    }

    score(play_game(p1, p2))
}
