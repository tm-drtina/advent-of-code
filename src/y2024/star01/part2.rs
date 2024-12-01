use std::collections::BTreeMap;

use anyhow::{Context, Result};

pub fn run(input: &str) -> Result<usize> {
    let mut left = Vec::<usize>::new();
    let mut right = BTreeMap::<usize, usize>::new();
    for line in input.lines() {
        let mut split = line.split_ascii_whitespace();
        left.push(
            split
                .next()
                .context("No data on the line")?
                .parse()
                .context("Failed to parse input number")?,
        );
        let right_num = split
            .next()
            .context("No sencond number on the line")?
            .parse()
            .context("Failed to parse input number")?;

        *right.entry(right_num).or_default() += 1;
    }

    Ok(left.into_iter().map(|l| right.get(&l).copied().unwrap_or_default() * l).sum())
}
