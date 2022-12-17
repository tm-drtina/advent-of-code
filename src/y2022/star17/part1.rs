use anyhow::Result;

use super::common::Cave;

pub fn run(input: &str) -> Result<usize> {
    let mut cave = Cave::new(input.parse()?);
    cave.drop_n(2022);
    Ok(cave.height())
}
