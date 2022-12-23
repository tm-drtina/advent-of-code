use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

use anyhow::{bail, Error, Result};

use crate::utils::point::{Dir, Point2D};

type Point = Point2D<i32>;

pub(super) struct State {
    directions: VecDeque<Dir>,
    positions: HashSet<Point>,
}

impl FromStr for State {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut directions = VecDeque::with_capacity(4);
        directions.push_back(Dir::Top);
        directions.push_back(Dir::Bottom);
        directions.push_back(Dir::Left);
        directions.push_back(Dir::Right);
        let mut positions = HashSet::new();

        for (y, line) in s.lines().enumerate() {
            let y = y as i32;
            for (x, b) in line.bytes().enumerate() {
                let x = x as i32;
                match b {
                    b'.' => {}
                    b'#' => {
                        positions.insert(Point2D { x, y });
                    }
                    _ => bail!("Invalid format"),
                }
            }
        }

        Ok(Self {
            directions,
            positions,
        })
    }
}

impl State {
    fn next_position(&self, position: Point) -> Point {
        if position
            .eight_neighborhood()
            .into_iter()
            .all(|p| !self.positions.contains(&p))
        {
            return position;
        }
        for dir in &self.directions {
            match dir {
                Dir::Top => {
                    if !self.positions.contains(&position.top_left())
                        && !self.positions.contains(&position.top())
                        && !self.positions.contains(&position.top_right())
                    {
                        return position.top();
                    }
                }
                Dir::Right => {
                    if !self.positions.contains(&position.top_right())
                        && !self.positions.contains(&position.right())
                        && !self.positions.contains(&position.bottom_right())
                    {
                        return position.right();
                    }
                }
                Dir::Bottom => {
                    if !self.positions.contains(&position.bottom_left())
                        && !self.positions.contains(&position.bottom())
                        && !self.positions.contains(&position.bottom_right())
                    {
                        return position.bottom();
                    }
                }
                Dir::Left => {
                    if !self.positions.contains(&position.top_left())
                        && !self.positions.contains(&position.left())
                        && !self.positions.contains(&position.bottom_left())
                    {
                        return position.left();
                    }
                }
                _ => unreachable!(),
            }
        }
        position
    }

    pub(super) fn round(&mut self) -> bool {
        let mut next: HashMap<Point, Vec<Point>> = HashMap::new();
        let mut moved = false;
        for pos in &self.positions {
            next.entry(self.next_position(*pos)).or_default().push(*pos);
        }
        self.positions.clear();
        for (new_pos, old_positions) in next {
            if old_positions.len() == 1 {
                self.positions.insert(new_pos);
                if new_pos != old_positions[0] {
                    moved = true;
                }
            } else {
                for pos in old_positions {
                    self.positions.insert(pos);
                }
            }
        }
        let first = self.directions.pop_front().unwrap();
        self.directions.push_back(first);
        moved
    }

    fn bounding_box(&self) -> (Point, Point) {
        let first = *self.positions.iter().next().unwrap();
        self.positions
            .iter()
            .fold((first, first), |(min, max), &Point2D { x, y }| {
                (
                    Point2D {
                        x: min.x.min(x),
                        y: min.y.min(y),
                    },
                    Point2D {
                        x: max.x.max(x),
                        y: max.y.max(y),
                    },
                )
            })
    }

    pub(super) fn empty_spaces(&self) -> usize {
        let (min, max) = self.bounding_box();
        let width = (max.x - min.x + 1) as usize;
        let height = (max.y - min.y + 1) as usize;
        width * height - self.positions.len()
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (Point2D { x: min_x, y: min_y }, Point2D { x: max_x, y: max_y }) = self.bounding_box();
        for y in min_y..=max_y {
            for x in min_x..max_x {
                if self.positions.contains(&Point2D { x, y }) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
