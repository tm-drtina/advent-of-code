use std::cmp::Ordering;

use anyhow::Result;

use super::part1::{Book, Puzzle, Rules};

impl Book {
    fn reorder(&mut self, rules: &Rules) {
        self.0.sort_unstable_by(|a, b| {
            if rules.get_previous(*a).contains(b) {
                Ordering::Greater
            } else if rules.get_previous(*b).contains(a) {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
    }
}

pub fn run(input: &str) -> Result<usize> {
    let puzzle = input.parse::<Puzzle>()?;

    Ok(puzzle
        .books
        .into_iter()
        .filter(|b| !b.is_valid(&puzzle.rules))
        .map(|mut b| {
            b.reorder(&puzzle.rules);
            b
        })
        .map(Book::middle_page)
        .sum())
}
