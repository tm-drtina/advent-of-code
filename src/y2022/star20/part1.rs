use anyhow::Result;

use super::common::Puzzle;

pub fn run(input: &str) -> Result<i64> {
    let mut puzzle: Puzzle = input.parse()?;
    puzzle.mix();
    puzzle.groove()
}
