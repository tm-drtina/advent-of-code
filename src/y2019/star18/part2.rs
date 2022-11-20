use rayon::prelude::*;

use super::common::{Keys, Maze, Tile};
use crate::utils::cache::sync::Cache;
use crate::utils::map::Point2D;

fn reachable_states(maze: &Maze, state: &State) -> Vec<(State, usize)> {
    (0..4)
        .flat_map(move |i| {
            maze.reachable_keys(state.keys, state.positions[i])
                .into_iter()
                .map(move |(point, step_distance, key)| {
                    let mut positions = state.positions;
                    positions[i] = point;
                    let mut keys = state.keys;
                    keys.add_key(key);
                    (State { positions, keys }, step_distance)
                })
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    positions: [Point2D; 4],
    keys: Keys,
}

fn compute(maze: &Maze, state: &State, cache: &Cache<State, usize>) -> usize {
    reachable_states(maze, state)
        .into_par_iter()
        .map(|(new_state, step_distance)| {
            if new_state.keys.count() == maze.keys.count() {
                step_distance
            } else {
                let distance_rest = cache.get_or_compute(new_state);
                distance_rest + step_distance
            }
        })
        .min()
        .unwrap()
}

pub fn run(input: &str) -> usize {
    let (mut maze, start_point) = Maze::load(input);
    let state = State {
        positions: [
            start_point.top_left(),
            start_point.top_right(),
            start_point.bottom_right(),
            start_point.bottom_left(),
        ],
        keys: Keys::new(),
    };
    maze.map.set(start_point, Tile::Wall);
    maze.map.set(start_point.top(), Tile::Wall);
    maze.map.set(start_point.right(), Tile::Wall);
    maze.map.set(start_point.bottom(), Tile::Wall);
    maze.map.set(start_point.left(), Tile::Wall);

    let cache = Cache::new(Box::new(move |state, cache| compute(&maze, state, cache)));
    cache.get_or_compute(state)
}
