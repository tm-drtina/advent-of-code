use anyhow::Result;

use super::common::Puzzle;

pub fn run(input: &str) -> Result<i64> {
    let mut puzzle: Puzzle = input.parse()?;
    puzzle.apply_decryption_key(811589153);
    for _ in 0..10 {
        puzzle.mix();
    }
    puzzle.groove()
}
