use std::str::FromStr;

use anyhow::Result;

use crate::utils::map::Map;
use crate::utils::point::Point2D;

struct State {
    position: Point2D<usize>,
    next: Point2D<usize>,
    map: Map<char>,
}

impl FromStr for State {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut start = None;
        let map = Map::from_str(s, |ch, x, y| {
            if ch == 'S' {
                start = Some(Point2D { x, y });
            }
            ch
        });
        let start = start.unwrap();
        let next = map
            .four_neighborhood(start)
            .into_iter()
            .find(|n| {
                let n = *n;
                let n_symbol = map.at(n);
                (start.right() == n && ['-', 'J', '7'].contains(n_symbol))
                    || (start.bottom() == n && ['|', 'L', 'J'].contains(n_symbol))
                    || (start.try_left() == Some(n) && ['-', 'L', 'F'].contains(n_symbol))
                    || (start.try_top() == Some(n) && ['|', '7', 'F'].contains(n_symbol))
            })
            .unwrap();

        Ok(Self {
            position: start,
            next,
            map,
        })
    }
}

impl Iterator for State {
    type Item = Point2D<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if *self.map.at(self.next) == 'S' {
            return None;
        }

        let next = match self.map.at(self.next) {
            '|' if self.next.y == self.position.bottom().y => self.next.bottom(),
            '|' if self.next.y == self.position.try_top().unwrap().y => {
                self.next.try_top().unwrap()
            }
            '-' if self.next.x == self.position.right().x => self.next.right(),
            '-' if self.next.x == self.position.try_left().unwrap().x => {
                self.next.try_left().unwrap()
            }
            'F' | 'L' if self.next.x == self.position.x => self.next.right(),
            'L' | 'J' if self.next.y == self.position.y => self.next.try_top().unwrap(),
            'J' | '7' if self.next.x == self.position.x => self.next.try_left().unwrap(),
            '7' | 'F' if self.next.y == self.position.y => self.next.bottom(),
            _ => unreachable!("Invalid next point"),
        };
        self.position = self.next;
        self.next = next;
        Some(self.position)
    }
}

pub fn run(input: &str) -> Result<usize> {
    Ok(input.parse::<State>()?.count().div_ceil(2))
}
