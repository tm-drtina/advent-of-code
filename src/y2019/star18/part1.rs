use std::collections::HashMap;

use super::common::{Keys, Maze};
use crate::utils::map::Point2D;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    position: Point2D,
    keys: Keys,
}

fn dfs(
    maze: &Maze,
    state: State,
    distance_sofar: usize,
    mut best_total_distance: usize,
    memory: &mut HashMap<State, usize>,
) -> usize {
    let mut distance_to_goal = usize::MAX / 2 - 1;
    for (point, distance, key) in maze.reachable_keys(state.keys, state.position) {
        if state.keys.count() + 1 == maze.keys.count() {
            distance_to_goal = distance_to_goal.min(distance);
        } else if distance_sofar + distance < best_total_distance {
            let mut new_state = State {
                position: point,
                keys: state.keys,
            };
            new_state.keys.add_key(key);

            let distance_rest = if let Some(value) = memory.get(&new_state) {
                *value
            } else {
                dfs(
                    maze,
                    new_state,
                    distance_sofar + distance,
                    best_total_distance,
                    memory,
                )
            };
            distance_to_goal = distance_to_goal.min(distance_rest + distance);
        }
        best_total_distance = best_total_distance.min(distance_sofar + distance_to_goal);
    }
    memory.insert(state, distance_to_goal);
    distance_to_goal
}

pub fn run(input: &str) -> usize {
    let mut memory = HashMap::new();
    let (map, start_point) = Maze::load(input);
    let state = State {
        position: start_point,
        keys: Keys::new(),
    };
    dfs(&map, state, 0, usize::MAX / 2 - 1, &mut memory)
}
