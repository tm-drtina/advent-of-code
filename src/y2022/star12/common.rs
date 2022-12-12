use std::rc::Rc;
use std::str::FromStr;

use anyhow::{Error, Result};

use crate::utils::map::Map;
use crate::utils::point::Point2D;

type Point = Point2D<usize>;

enum Node {
    Start(u8),
    End(u8),
    Path(u8),
}

impl Node {
    fn height(&self) -> u8 {
        match self {
            Node::Start(height) | Node::End(height) | Node::Path(height) => *height,
        }
    }
}

pub(super) struct Puzzle {
    map: Rc<Map<Node>>,
    start: Point,
}

impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = Point::default();

        let map = Map::from_str(s, |ch, x, y| match ch {
            'S' => {
                start = Point2D { x, y };
                Node::Start(0)
            }
            'E' => Node::End(b'z' - b'a'),
            _ => Node::Path(ch as u8 - b'a'),
        });
        let map = Rc::new(map);
        Ok(Self { map, start })
    }
}

impl Puzzle {
    pub(super) fn start(&self) -> Point {
        self.start
    }

    pub(super) fn lowest_points(&self) -> Vec<Point> {
        (0..self.map.width())
            .flat_map(|x| {
                let map = &self.map;
                (0..self.map.height()).filter_map(move |y| {
                    let point = Point2D { x, y };
                    if map.at(point).height() == 0 {
                        Some(point)
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    pub(super) fn successors(&self, current: Point) -> Vec<(Point, usize)> {
        let height = self.map.at(current).height();
        self.map
            .four_neighborhood(current)
            .into_iter()
            .filter(|p| self.map.at(*p).height() <= height + 1)
            .map(|p| (p, 1))
            .collect()
    }

    pub(super) fn is_end(&self, current: Point) -> bool {
        matches!(self.map.at(current), Node::End(_))
    }
}
