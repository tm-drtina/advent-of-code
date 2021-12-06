fn solve_step(state: [usize; 9]) -> [usize; 9] {
    [
        state[1],
        state[2],
        state[3],
        state[4],
        state[5],
        state[6],
        state[7] + state[0],
        state[8],
        state[0],

    ]
}

pub(super) fn solve(mut state: [usize; 9], runs: usize) -> usize {
    for _ in 0..runs {
        state = solve_step(state);
    }
    state.into_iter().sum()
}

pub(super) fn parse(input: &str) -> [usize; 9] {
    input.split(',').fold([0; 9], |mut acc, s| {
        acc[s.parse::<usize>().unwrap()] += 1;
        acc
    })
}

pub fn run(input: &str) -> usize {
    let state = parse(input);
    solve(state, 80)
}
