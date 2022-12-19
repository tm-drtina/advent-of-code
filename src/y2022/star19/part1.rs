use anyhow::Result;

use super::common::Blueprint;

pub fn run(input: &str) -> Result<u32> {
    Ok(input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Blueprint>>>()?
        .iter()
        .map(|b| b.quality(24))
        .sum())
}
