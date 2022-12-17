use anyhow::Result;
use num::Integer;

use super::common::{Cave, JetPattern};

pub fn run(input: &str) -> Result<usize> {
    let jet_pattern: JetPattern = input.parse()?;
    let mut cave = Cave::new(jet_pattern.iter());
    let repeat = jet_pattern.len().lcm(&5);
    let mut index = 0;
    const TARGET: usize = 1000;
    loop {
        if index + repeat < TARGET {
            // TODO: load from cache?
            for i in 0..repeat {
                cave.drop_stone(i);
            }
            index += repeat;
            cave.canonize();
            eprintln!("{index}");
            eprintln!("{cave}");
        } else {
            for i in index..=TARGET {
                cave.drop_stone(i);
            }
            return Ok(cave.height())
        }
    }
}
