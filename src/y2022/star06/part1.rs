use anyhow::{Result, anyhow};
use itertools::Itertools;

pub fn run(input: &str) -> Result<usize> {
    input
        .bytes()
        .tuple_windows()
        .position(|(a, b, c, d)| a != b && a != c && a != d && b != c && b != d && c != d)
        .map(|x| x + 4)
        .ok_or_else(|| anyhow!("Pattern not found"))
}
