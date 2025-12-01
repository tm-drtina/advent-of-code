use anyhow::{Result, anyhow};

use super::common::{Map, State};

pub fn run(input: &str) -> Result<usize> {
    let mut map: Map = input.parse()?;
    let mut state = State::start();
    for i in 1.. {
        map = map.step();
        state = state.step(&map);
        if state.is_final(&map) {
            return Ok(i);
        }
    }
    Err(anyhow!("Solution not found"))
}
