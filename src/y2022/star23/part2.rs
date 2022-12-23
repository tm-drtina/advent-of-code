use anyhow::{anyhow, Result};

use super::common::State;

pub fn run(input: &str) -> Result<usize> {
    let mut state: State = input.parse()?;
    (2..)
        .take_while(|_| state.round())
        .last()
        .ok_or(anyhow!("Solution not found"))
}
