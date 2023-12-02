use std::str::FromStr;

use anyhow::Result;

use super::part1::Game;

pub fn run(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(Game::from_str)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(|g| g.red * g.green * g.blue)
        .sum())
}
