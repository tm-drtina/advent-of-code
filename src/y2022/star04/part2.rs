use anyhow::Result;

use super::common::Pair;

pub fn run(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(str::parse::<Pair>)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .filter(Pair::overlaps)
        .count())
}
