use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

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
    fn passable(&self, state: &State, point: Point) -> bool {
        if self.map[point.y][point.x] == '#' {
            false
        } else if self.map[point.y][point.x].is_ascii_uppercase() {
            state
                .keys
                .contains(self.map[point.y][point.x].to_ascii_lowercase())
        } else {
            true
        }
    }

    fn reachable_keys(&self, state: &State) -> Vec<(Point, usize, char)> {
        let mut visited = HashSet::new();

        visited.insert(state.position);
        let mut open = VecDeque::new();
        open.push_back((state.position, 0usize));
        let mut reachable = Vec::new();

        while !open.is_empty() {
            let (position, distance) = open.pop_front().unwrap();
            let ch = self.map[position.y][position.x];
            if ch.is_ascii_lowercase() && !state.keys.contains(ch) {
                reachable.push((position, distance, ch));
            }

            if reachable.len() + state.keys.count() == self.keys.count() {
                break;
            }

            if position.x > 0 {
                let point = Point {
                    x: position.x - 1,
                    y: position.y,
                };
                if !visited.contains(&point) && self.passable(state, point) {
                    visited.insert(point);
                    open.push_back((point, distance + 1));
                }
            }

            if position.x < self.width - 1 {
                let point = Point {
                    x: position.x + 1,
                    y: position.y,
                };
                if !visited.contains(&point) && self.passable(state, point) {
                    visited.insert(point);
                    open.push_back((point, distance + 1));
                }
            }

            if position.y > 0 {
                let point = Point {
                    x: position.x,
                    y: position.y - 1,
                };
                if !visited.contains(&point) && self.passable(state, point) {
                    visited.insert(point);
                    open.push_back((point, distance + 1));
                }
            }

            if position.y < self.height {
                let point = Point {
                    x: position.x,
                    y: position.y + 1,
                };
                if !visited.contains(&point) && self.passable(state, point) {
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
    position: Point,
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
    distance_sofar: usize,
    mut best_total_distance: usize,
    memory: &mut HashMap<State, usize>,
) -> usize {
    let mut distance_to_goal = usize::MAX / 2 - 1;
    for (point, distance, key) in map.reachable_keys(&state) {
        if state.keys.count() + 1 == map.keys.count() {
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
                dfs(map, new_state, distance_sofar + distance, best_total_distance, memory)
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
    let (map, start_point) = load_map(input);
    let state = State {
        position: start_point,
        keys: Keys::new(),
    };
    dfs(&map, state, 0, usize::MAX / 2 - 1, &mut memory)
}
