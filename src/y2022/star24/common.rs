use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use anyhow::{Error, Result, bail};

use crate::utils::point::{Dir, Point2D};

type Point = Point2D<usize>;

pub(super) struct Map {
    storms: HashMap<Point, Vec<Dir>>,
    height: usize,
    width: usize,
}

impl Map {
    fn move_storm(&self, position: Point, direction: Dir) -> Point {
        match direction {
            Dir::Top => {
                if position.y > 1 {
                    Point2D {
                        x: position.x,
                        y: position.y - 1,
                    }
                } else {
                    Point2D {
                        x: position.x,
                        y: self.height - 2,
                    }
                }
            }
            Dir::Right => {
                if position.x < self.width - 2 {
                    Point2D {
                        x: position.x + 1,
                        y: position.y,
                    }
                } else {
                    Point2D {
                        x: 1,
                        y: position.y,
                    }
                }
            }
            Dir::Bottom => {
                if position.y < self.height - 2 {
                    Point2D {
                        x: position.x,
                        y: position.y + 1,
                    }
                } else {
                    Point2D {
                        x: position.x,
                        y: 1,
                    }
                }
            }
            Dir::Left => {
                if position.x > 1 {
                    Point2D {
                        x: position.x - 1,
                        y: position.y,
                    }
                } else {
                    Point2D {
                        x: self.width - 2,
                        y: position.y,
                    }
                }
            }
            Dir::TopLeft | Dir::TopRight | Dir::BottomRight | Dir::BottomLeft => unreachable!(),
        }
    }

    fn move_elf(&self, position: Point) -> Vec<Point> {
        if position.x == 1 && position.y == 0 {
            let p = Point2D { x: 1, y: 1 };
            return if self.storms.contains_key(&p) {
                vec![position]
            } else {
                vec![position, p]
            };
        }
        if position.x == self.width - 2 && position.y == self.height - 1 {
            let p = Point2D {
                x: self.width - 2,
                y: self.height - 2,
            };
            return if self.storms.contains_key(&p) {
                vec![position]
            } else {
                vec![position, p]
            };
        }

        let mut res = Vec::new();
        if !self.storms.contains_key(&position) {
            res.push(position);
        }
        if position.y > 1 || (position.x == 1 && position.y == 1) {
            let p = Point2D {
                x: position.x,
                y: position.y - 1,
            };
            if !self.storms.contains_key(&p) {
                res.push(p);
            }
        }
        if position.y < self.height - 2
            || (position.y == self.height - 2 && position.x == self.width - 2)
        {
            let p = Point2D {
                x: position.x,
                y: position.y + 1,
            };
            if !self.storms.contains_key(&p) {
                res.push(p);
            }
        }
        if position.x > 1 {
            let p = Point2D {
                x: position.x - 1,
                y: position.y,
            };
            if !self.storms.contains_key(&p) {
                res.push(p);
            }
        }
        if position.x < self.width - 2 {
            let p = Point2D {
                x: position.x + 1,
                y: position.y,
            };
            if !self.storms.contains_key(&p) {
                res.push(p);
            }
        }
        res
    }

    pub(super) fn step(&self) -> Self {
        let mut next_storms: HashMap<Point2D<usize>, Vec<Dir>> = HashMap::new();
        for (&position, storms) in &self.storms {
            for &direction in storms {
                let next_pos = self.move_storm(position, direction);
                next_storms.entry(next_pos).or_default().push(direction);
            }
        }
        Self {
            storms: next_storms,
            height: self.height,
            width: self.width,
        }
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut storms = HashMap::new();
        let mut height = 0;
        let mut width = 0;
        for (y, line) in s.lines().enumerate() {
            height += 1;
            if y == 0 {
                width = line.len();
            }
            for (x, b) in line.bytes().enumerate() {
                match b {
                    b'<' => {
                        storms.insert(Point2D { x, y }, vec![Dir::Left]);
                    }
                    b'>' => {
                        storms.insert(Point2D { x, y }, vec![Dir::Right]);
                    }
                    b'^' => {
                        storms.insert(Point2D { x, y }, vec![Dir::Top]);
                    }
                    b'v' => {
                        storms.insert(Point2D { x, y }, vec![Dir::Bottom]);
                    }
                    b'.' | b'#' => {}
                    _ => bail!("Invalid format"),
                }
            }
        }
        Ok(Self {
            storms,
            height,
            width,
        })
    }
}

pub(super) struct State(HashSet<Point>);

impl State {
    pub(super) fn start() -> Self {
        let mut positions = HashSet::new();
        positions.insert(Point2D { x: 1, y: 0 });
        Self(positions)
    }

    pub(super) fn end(map: &Map) -> Self {
        let mut positions = HashSet::new();
        positions.insert(Point2D {
            x: map.width - 2,
            y: map.height - 1,
        });
        Self(positions)
    }

    pub(super) fn step(self, map: &Map) -> Self {
        Self(
            self.0
                .into_iter()
                .flat_map(|position| map.move_elf(position))
                .collect(),
        )
    }

    pub(super) fn is_final(&self, map: &Map) -> bool {
        self.0
            .iter()
            .any(|&Point2D { x, y }| y == map.height - 1 && x == map.width - 2)
    }

    pub(super) fn is_start(&self) -> bool {
        self.0.iter().any(|&Point2D { x, y }| y == 0 && x == 1)
    }
}
