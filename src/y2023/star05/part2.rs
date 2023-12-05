use anyhow::{anyhow, Result};

use super::part1::Puzzle;

pub fn run(input: &str) -> Result<u64> {
    let puzzle: Puzzle = input.parse()?;
    puzzle
        .initial_seeds
        .chunks_exact(2)
        .flat_map(|a| a[0]..a[0] + a[1])
        .map(|s| puzzle.map(s))
        .min()
        .ok_or(anyhow!("No initial seeds"))
}
