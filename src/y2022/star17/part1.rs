use anyhow::Result;

use super::common::{Cave, JetPattern};

pub fn run(input: &str) -> Result<usize> {
    let jet_pattern: JetPattern = input.parse()?;
    let mut cave = Cave::new(jet_pattern.iter());
    for i in 0..2022 {
        cave.drop_stone(i);
        // eprintln!("{cave}");
    }
    Ok(cave.height())
}
