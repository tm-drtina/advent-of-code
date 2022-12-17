use anyhow::Result;
use num::Integer;

use super::common::{Cave, JetPattern};

pub fn run(input: &str) -> Result<usize> {
    let jet_pattern: JetPattern = input.parse()?;
    let mut cave = Cave::new(jet_pattern.iter());
    let checkpoint = jet_pattern.len().lcm(&5);
    let mut aa = 0;
    //for i in 0..1_000_000_000_000 {
    for i in 0..100 {
        if i % checkpoint == 0 {
            eprintln!("AA: {}  {}", cave.height() - aa, cave.height());
            aa = cave.height();
            eprintln!("{cave}");
        }
        cave.drop_stone(i);
    }
    Ok(cave.height())
}

