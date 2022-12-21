use anyhow::Result;

use super::common::Puzzle;

pub fn run(input: &str) -> Result<usize> {
    Ok(Puzzle::compute_periodicity(input.parse()?).drop_n(1_000_000_000_000))
}
