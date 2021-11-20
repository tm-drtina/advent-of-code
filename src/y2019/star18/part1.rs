use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

struct Map {
    map: Vec<Vec<char>>,
    keys: usize,
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
                .contains(&self.map[point.y][point.x].to_ascii_lowercase())
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
            if ch.is_ascii_lowercase() && !state.keys.contains(&ch) {
                reachable.push((position, distance, ch));
            }

            if reachable.len() + state.keys.len() == self.keys {
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

#[derive(Debug)]
struct State {
    position: Point,
    keys: BTreeSet<char>,
    distance: usize,
}

fn load_map(input: &str) -> (Map, Point) {
    let mut map = Map {
        map: Vec::new(),
        keys: 0,
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
                map.keys += 1;
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
    state: &State,
    mut best_distance: usize,
    memory: &mut HashMap<(BTreeSet<char>, Point), usize>,
) -> usize {
    let mut best_rest = usize::MAX / 2 - 1;
    for (point, distance, key) in map.reachable_keys(state) {
        let new_distance = state.distance + distance;
        if state.keys.len() + 1 == map.keys {
            best_rest = best_rest.min(distance);
        } else if new_distance < best_distance {
            let mut new_state = State {
                position: point,
                distance: state.distance + distance,
                keys: state.keys.clone(),
            };
            new_state.keys.insert(key);

            let key = (new_state.keys.clone(), point);
            let rest = if let Some(value) = memory.get(&key) {
                *value
            } else {
                let rest = dfs(map, &new_state, best_distance, memory);
                memory.insert(key, rest);
                rest
            };
            best_rest = best_rest.min(rest + distance);
            best_distance = best_distance.min(state.distance + best_rest);
        }
    }
    memory.insert((state.keys.clone(), state.position), best_rest);
    best_rest
}

pub fn run(input: &str) -> usize {
    let mut memory = HashMap::new();
    let (map, start_point) = load_map(input);
    let state = State {
        position: start_point,
        keys: BTreeSet::new(),
        distance: 0,
    };
    dfs(&map, &state, usize::MAX / 2 - 1, &mut memory)
}
