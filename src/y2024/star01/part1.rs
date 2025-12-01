use anyhow::{Context, Result};

pub fn run(input: &str) -> Result<usize> {
    let mut left = Vec::<usize>::new();
    let mut right = Vec::<usize>::new();
    for line in input.lines() {
        let mut split = line.split_ascii_whitespace();
        left.push(
            split
                .next()
                .context("No data on the line")?
                .parse()
                .context("Failed to parse input number")?,
        );
        right.push(
            split
                .next()
                .context("No sencond number on the line")?
                .parse()
                .context("Failed to parse input number")?,
        );
    }
    left.sort_unstable();
    right.sort_unstable();

    Ok(left
        .into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum())
}
