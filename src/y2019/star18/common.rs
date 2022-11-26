use std::collections::{HashSet, VecDeque};

use crate::utils::map::Map;
use crate::utils::point::Point2D;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Keys(u32);
impl Keys {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn add_key(&mut self, key: u8) {
        self.0 |= 1u32 << key;
    }

    pub fn contains(self, key: u8) -> bool {
        self.0 & (1u32 << key) != 0
    }

    pub fn count(self) -> usize {
        self.0.count_ones() as usize
    }
}

impl Default for Keys {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Wall,
    Key(u8),
    Door(u8),
}

pub struct Maze {
    pub map: Map<Tile>,
    pub keys: Keys,
}

impl Maze {
    pub fn passable(&self, keys: Keys, point: Point2D<usize>) -> bool {
        match self.map.at(point) {
            Tile::Empty | Tile::Key(_) => true,
            Tile::Door(key) => keys.contains(*key),
            Tile::Wall => false,
        }
    }

    pub fn reachable_keys(
        &self,
        keys: Keys,
        position: Point2D<usize>,
    ) -> Vec<(Point2D<usize>, usize, u8)> {
        let mut visited = HashSet::new();

        visited.insert(position);
        let mut open = VecDeque::new();
        open.push_back((position, 0usize));
        let mut reachable = Vec::new();

        while !open.is_empty() {
            let (position, distance) = open.pop_front().unwrap();
            let ch = *self.map.at(position);
            match ch {
                Tile::Key(key) if !keys.contains(key) => {
                    reachable.push((position, distance, key));
                }
                _ => {}
            }

            //if reachable.len() + keys.count() == self.keys.count() {
            //    break;
            //}

            self.map
                .four_neighborhood(position)
                .into_iter()
                .for_each(|point| {
                    if !visited.contains(&point) && self.passable(keys, point) {
                        visited.insert(point);
                        open.push_back((point, distance + 1));
                    }
                });
        }

        reachable
    }

    pub fn load(input: &str) -> (Self, Point2D<usize>) {
        let mut keys = Keys::new();
        let mut start_point = Point2D { x: 0, y: 0 };

        let map = Map::from_str(input, |ch, x, y| match ch {
            _ if ch.is_ascii_lowercase() => {
                let key = ch as u8 - b'a';
                keys.add_key(key);
                Tile::Key(key)
            }
            _ if ch.is_ascii_uppercase() => Tile::Door(ch as u8 - b'A'),
            '#' => Tile::Wall,
            '.' => Tile::Empty,
            '@' => {
                start_point = Point2D { x, y };
                Tile::Empty
            }
            _ => unreachable!("Invalid char {}", ch),
        });
        (Self { map, keys }, start_point)
    }
}
