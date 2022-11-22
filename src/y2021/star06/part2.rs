use super::part1::{parse, solve};

pub fn run(input: &str) -> usize {
    let state = parse(input);
    solve(state, 256)
}
