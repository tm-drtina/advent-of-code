use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};

use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Keys(u32);
impl Keys {
    fn new() -> Self {
        Self(0)
    }

    fn char_to_bitshifted_int(key: char) -> u32 {
        debug_assert!(key.is_ascii_lowercase());
        1u32 << (key as u32 - 'a' as u32)
    }

    fn add_key(&mut self, key: char) {
        self.0 |= Self::char_to_bitshifted_int(key);
    }

    fn contains(&self, key: char) -> bool {
        self.0 & Self::char_to_bitshifted_int(key) != 0
    }

    fn count(&self) -> usize {
        self.0.count_ones() as usize
    }
}

impl Default for Keys {
    fn default() -> Self {
        Self::new()
    }
}

struct Map {
    map: Vec<Vec<char>>,
    keys: Keys,
    width: usize,
    height: usize,
}

impl Map {
    fn passable(&self, keys: &Keys, point: Point) -> bool {
        if self.map[point.y][point.x] == '#' {
            false
        } else if self.map[point.y][point.x].is_ascii_uppercase() {
            keys.contains(self.map[point.y][point.x].to_ascii_lowercase())
        } else {
            true
        }
    }

    fn reachable_states(&self, state: State) -> Vec<(State, usize)> {
        (0..4).map(move |i| {
            self.reachable_keys(&state.keys, state.positions[i]).into_iter().map(move |(point, step_distance, key)| {
                let mut positions = state.positions;
                positions[i] = point;
                let mut keys = state.keys;
                keys.add_key(key);
                (State { positions, keys }, step_distance)
            })
        }).flatten().collect()
    }

    fn reachable_keys(&self, keys: &Keys, position: Point) -> Vec<(Point, usize, char)> {
        let mut visited = HashSet::new();

        visited.insert(position);
        let mut open = VecDeque::new();
        open.push_back((position, 0usize));
        let mut reachable = Vec::new();

        while !open.is_empty() {
            let (position, distance) = open.pop_front().unwrap();
            let ch = self.map[position.y][position.x];
            if ch.is_ascii_lowercase() && !keys.contains(ch) {
                reachable.push((position, distance, ch));
            }

            //if reachable.len() + keys.count() == self.keys.count() {
            //    break;
            //}

            if position.x > 0 {
                let point = Point {
                    x: position.x - 1,
                    y: position.y,
                };
                if !visited.contains(&point) && self.passable(keys, point) {
                    visited.insert(point);
                    open.push_back((point, distance + 1));
                }
            }

            if position.x < self.width - 1 {
                let point = Point {
                    x: position.x + 1,
                    y: position.y,
                };
                if !visited.contains(&point) && self.passable(keys, point) {
                    visited.insert(point);
                    open.push_back((point, distance + 1));
                }
            }

            if position.y > 0 {
                let point = Point {
                    x: position.x,
                    y: position.y - 1,
                };
                if !visited.contains(&point) && self.passable(keys, point) {
                    visited.insert(point);
                    open.push_back((point, distance + 1));
                }
            }

            if position.y < self.height {
                let point = Point {
                    x: position.x,
                    y: position.y + 1,
                };
                if !visited.contains(&point) && self.passable(keys, point) {
                    visited.insert(point);
                    open.push_back((point, distance + 1));
                }
            }
        }

        reachable
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    positions: [Point; 4],
    keys: Keys,
}

fn load_map(input: &str) -> (Map, Point) {
    let mut map = Map {
        map: Vec::new(),
        keys: Keys::new(),
        width: 0,
        height: 0,
    };
    let mut start_point = Point { x: 0, y: 0 };

    for (y, line) in input.lines().enumerate() {
        if line.is_empty() {
            break;
        }
        if y == 0 {
            map.width = line.len();
        }
        map.height = y + 1;

        let chars = line.chars().collect_vec();
        for (x, char) in chars.iter().enumerate() {
            if char.is_ascii_lowercase() {
                map.keys.add_key(*char);
            } else if *char == '@' {
                start_point = Point { x, y };
            }
        }
        map.map.push(chars);
    }

    (map, start_point)
}

fn dfs(
    map: &Map,
    state: State,
    memory: Arc<Mutex<HashMap<State, usize>>>,
) -> usize {
    let memory = &memory;
    let distance_to_goal = map.reachable_states(state).into_par_iter().map(|(new_state, step_distance)| {
        if new_state.keys.count() == map.keys.count() {
            step_distance
        } else {
            let cached_value = memory.lock().unwrap().get(&new_state).copied();
            let distance_rest = if let Some(value) = cached_value {
                value
            } else {
                dfs(map, new_state, Arc::clone(memory))
            };
            distance_rest + step_distance
        }
    }).min().unwrap_or(usize::MAX / 2 - 1);
    memory.lock().unwrap().insert(state, distance_to_goal);
    distance_to_goal
}

pub fn run(input: &str) -> usize {
    let memory = Arc::new(Mutex::new(HashMap::new()));
    let (mut map, start_point) = load_map(input);
    let state = State {
        positions: [
            Point {
                x: start_point.x - 1,
                y: start_point.y - 1,
            },
            Point {
                x: start_point.x - 1,
                y: start_point.y + 1,
            },
            Point {
                x: start_point.x + 1,
                y: start_point.y - 1,
            },
            Point {
                x: start_point.x + 1,
                y: start_point.y + 1,
            },
        ],
        keys: Keys::new(),
    };
    map.map[start_point.y][start_point.x] = '#';
    map.map[start_point.y - 1][start_point.x] = '#';
    map.map[start_point.y + 1][start_point.x] = '#';
    map.map[start_point.y][start_point.x - 1] = '#';
    map.map[start_point.y][start_point.x + 1] = '#';
    dfs(&map, state, memory)
}
