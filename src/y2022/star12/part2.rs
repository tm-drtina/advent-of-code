use anyhow::{Result, anyhow};
use pathfinding::prelude::dijkstra;

use super::common::Puzzle;

pub fn run(input: &str) -> Result<usize> {
    let puzzle: Puzzle = input.parse()?;
    puzzle
        .lowest_points()
        .into_iter()
        .filter_map(|start| {
            dijkstra(&start, |p| puzzle.successors(*p), |p| puzzle.is_end(*p)).map(|(_, len)| len)
        })
        .min()
        .ok_or_else(|| anyhow!("solution not found"))
}
