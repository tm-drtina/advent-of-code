use std::cmp::Ordering;
use std::str::FromStr;

use anyhow::Result;

use super::part1::Report;

impl Report {
    fn is_safe_with_removal(&self) -> bool {
        'outer: for i in 0..(self.0.len() - 1) {
            let mut data = self.0.clone();
            data.remove(i);
            let mut ord = Ordering::Equal;
            for (prev, next) in data.iter().zip(data.iter().skip(1)) {
                let diff = prev.abs_diff(*next);
                if diff == 0 || diff > 3 {
                    continue 'outer;
                }
                if ord == Ordering::Equal {
                    ord = prev.cmp(next);
                } else if ord != prev.cmp(next) {
                    continue 'outer;
                }
            }
            return true;
        }
        return false;
    }
}

pub fn run(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(Report::from_str)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .filter(Report::is_safe_with_removal)
        .count())
}
