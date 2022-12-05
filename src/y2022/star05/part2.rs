use anyhow::Result;
use itertools::Itertools;

use super::common::Puzzle;

pub fn run(input: &str) -> Result<String> {
    let mut puzzle: Puzzle = input.parse()?;
    puzzle.moves.drain(..).for_each(|m| {
        let orig = &mut puzzle.stacks[m.from];
        let mut rest = orig.split_off(orig.len() - m.amount);
        puzzle.stacks[m.to].append(&mut rest);
    });
    Ok(puzzle
        .stacks
        .iter()
        .map(|s| s.last().copied().unwrap_or(' '))
        .join(""))
}
