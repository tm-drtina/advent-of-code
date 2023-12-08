use std::collections::HashMap;

use anyhow::Result;

use super::part1::{GoalIter, Loc, Puzzle};

struct State {
    pos: Loc,
    steps: usize,
}

impl GoalIter<'_> {
    fn find_end_pos(self) -> (Loc, usize) {
        for (i, pos) in self.enumerate() {
            if pos & 255 == b'Z' as Loc {
                return (pos, i + 1);
            }
        }
        unreachable!()
    }
}

impl Puzzle {
    fn solve_multi_goal(self) -> usize {
        let starts: Vec<_> = self
            .transitions
            .keys()
            .copied()
            .filter(|p| p & 255 == b'A' as Loc)
            .collect();

        let simple_transitions: HashMap<Loc, (Loc, usize)> = self
            .transitions
            .keys()
            .copied()
            .filter(|p| p & 255 == b'Z' as Loc)
            .map(|p| (p, self.goal_iter(p).find_end_pos()))
            .collect();

        let mut max_steps = 0;

        let mut states: Vec<_> = starts
            .into_iter()
            .map(|p| {
                let (pos, steps) = self.goal_iter(p).find_end_pos();
                if steps > max_steps {
                    max_steps = steps;
                }
                State { pos, steps }
            })
            .collect();

        let mut changed = true;
        while changed {
            changed = false;
            for state in &mut states {
                while state.steps < max_steps {
                    let (new_pos, steps) = simple_transitions.get(&state.pos).copied().unwrap();
                    state.pos = new_pos;
                    state.steps += steps;
                    if state.steps > max_steps {
                        max_steps = state.steps;
                        changed = true;
                    }
                }
            }
        }
        max_steps
    }
}

pub fn run(input: &str) -> Result<usize> {
    Ok(input.parse::<Puzzle>()?.solve_multi_goal())
}
