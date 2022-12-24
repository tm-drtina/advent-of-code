use anyhow::Result;

use super::common::{Map, State};

pub fn run(input: &str) -> Result<usize> {
    let mut map: Map = input.parse()?;
    let mut state = State::start();
    let mut index = 0;
    loop {
        index += 1;
        map = map.step();
        state = state.step(&map);
        if state.is_final(&map) {
            break;
        }
    }
    state = State::end(&map);
    loop {
        index += 1;
        map = map.step();
        state = state.step(&map);
        if state.is_start() {
            break;
        }
    }
    state = State::start();
    loop {
        index += 1;
        map = map.step();
        state = state.step(&map);
        if state.is_final(&map) {
            break;
        }
    }
    Ok(index)
}
