use anyhow::{anyhow, Result};
use pathfinding::prelude::dijkstra;

use super::common::Puzzle;

pub fn run(input: &str) -> Result<usize> {
    let puzzle: Puzzle = input.parse()?;
    Ok(dijkstra(
        &puzzle.start(),
        |p| puzzle.successors(*p),
        |p| puzzle.is_end(*p),
    )
    .ok_or_else(|| anyhow!("solution not found"))?
    .1)
}
