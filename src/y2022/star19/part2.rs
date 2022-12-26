use anyhow::Result;

use super::common::Blueprint;

pub fn run(input: &str) -> Result<u32> {
    Ok(input
        .lines()
        .take(3)
        .map(str::parse)
        .collect::<Result<Vec<Blueprint>>>()?
        .iter()
        .map(|b| b.max_geodes::<32>())
        .product())
}
