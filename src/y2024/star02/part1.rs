use std::cmp::Ordering;
use std::num::ParseIntError;
use std::str::FromStr;

use anyhow::Result;

pub(super) struct Report(pub(super) Vec<usize>);

impl FromStr for Report {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(
            s.split_ascii_whitespace()
                .map(usize::from_str)
                .collect::<Result<Vec<_>, ParseIntError>>()?,
        ))
    }
}

impl Report {
    fn is_safe(&self) -> bool {
        let mut ord = Ordering::Equal;
        for (prev, next) in self.0.iter().zip(self.0.iter().skip(1)) {
            let diff = prev.abs_diff(*next);
            if diff == 0 || diff > 3 {
                return false;
            }
            if ord == Ordering::Equal {
                ord = prev.cmp(next);
            } else if ord != prev.cmp(next) {
                return false;
            }
        }
        true
    }
}

pub fn run(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(Report::from_str)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .filter(Report::is_safe)
        .count())
}
