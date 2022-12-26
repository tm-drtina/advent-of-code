use std::collections::HashSet;
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use itertools::Itertools;

use crate::utils::point::Point2D;

type Point = Point2D<u32>;

struct Puzzle {
    blocked: HashSet<Point>,
    max_y: u32,
}

impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocked = HashSet::new();
        let mut max_y = 0;
        for line in s.lines() {
            for (from, to) in line.split(" -> ").tuple_windows() {
                let (from_x, from_y) = from
                    .split_once(',')
                    .ok_or_else(|| anyhow!("Invalid format"))?;
                let mut from_x = from_x.parse()?;
                let mut from_y = from_y.parse()?;
                let (to_x, to_y) = to
                    .split_once(',')
                    .ok_or_else(|| anyhow!("Invalid format"))?;
                let mut to_x = to_x.parse()?;
                let mut to_y = to_y.parse()?;

                if from_y > to_y {
                    std::mem::swap(&mut from_y, &mut to_y);
                }
                if from_x > to_x {
                    std::mem::swap(&mut from_x, &mut to_x);
                }

                if to_y > max_y {
                    max_y = to_y;
                }

                for x in from_x..=to_x {
                    for y in from_y..=to_y {
                        blocked.insert(Point2D { x, y });
                    }
                }
            }
        }
        Ok(Self { blocked, max_y })
    }
}

impl Puzzle {
    fn simulate(&self) -> Point {
        let mut pos = Point2D { x: 500, y: 0 };
        loop {
            if pos.y == self.max_y + 1 {
                return pos;
            }
            if !self.blocked.contains(&pos.bottom()) {
                pos = pos.bottom();
            } else if !self.blocked.contains(&pos.bottom_left()) {
                pos = pos.bottom_left();
            } else if !self.blocked.contains(&pos.bottom_right()) {
                pos = pos.bottom_right();
            } else {
                return pos;
            }
        }
    }
}

impl Iterator for Puzzle {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.simulate();
        self.blocked.insert(next);
        Some(next)
    }
}

pub fn run(input: &str) -> Result<usize> {
    Ok(input.parse::<Puzzle>()?.take_while(|p| p.y > 0).count() + 1)
}
