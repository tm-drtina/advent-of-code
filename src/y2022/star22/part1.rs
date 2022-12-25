use anyhow::Result;

use super::common::Puzzle;

pub fn run(input: &str) -> Result<usize> {
    Ok(input.parse::<Puzzle>()?.go().password())
}
