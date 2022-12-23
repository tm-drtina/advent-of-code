use anyhow::Result;

use super::common::State;

pub fn run(input: &str) -> Result<usize> {
    let mut state: State = input.parse()?;
    for _ in 0..10 {
        state.round();
    }
    Ok(state.empty_spaces())
}
