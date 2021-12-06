use super::part1::*;

pub fn run(input: &str) -> usize {
    let state = parse(input);
    solve(state, 256)
}
