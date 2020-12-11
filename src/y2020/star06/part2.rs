use itertools::Itertools;
use std::collections::HashSet;

struct Input<'a, T: Iterator<Item = &'a str>> {
    data: T,
}

impl<'a, T: Iterator<Item = &'a str>> Iterator for Input<'a, T> {
    type Item = Vec<HashSet<char>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut res: Vec<HashSet<char>> = Vec::new();
        loop {
            match self.data.next() {
                None => break,
                Some(line) => {
                    if line.is_empty() {
                        break;
                    }
                    res.push(line.chars().collect());
                }
            }
        }

        if res.is_empty() {
            None
        } else {
            Some(res)
        }
    }
}

pub fn run(input: &str) -> usize {
    Input {
        data: input.lines(),
    }
    .map(|group| {
        group
            .into_iter()
            .fold1(|set1, set2| set1.intersection(&set2).copied().collect())
            .unwrap()
            .len()
    })
    .sum()
}
