use anyhow::Result;
use num::Integer;

use super::common::{Cave, JetPattern};

pub fn run(input: &str) -> Result<usize> {
    let jet_pattern: JetPattern = input.parse()?;
    let checkpoint = jet_pattern.len().lcm(&5);
    let mut cave = Cave::new(jet_pattern.iter());
    for i in 0..2022 {
        if i % checkpoint == 0 {
            cave.canonize();
        }
        cave.drop_stone(i);
        // eprintln!("{cave}");
    }
    Ok(cave.height())
}
