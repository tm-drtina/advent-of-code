use std::collections::BTreeSet;
use std::str::FromStr;

use anyhow::{Error, Result};
use pathfinding::prelude::*;

use crate::utils::point::Point2D;

type Point = Point2D<usize>;

#[derive(Debug, Clone, Copy)]
pub(super) enum Dir {
    Left,
    Right,
}

pub(super) struct JetPattern(Vec<Dir>);

impl JetPattern {
    fn at(&self, index: usize) -> Dir {
        self.0[index]
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl FromStr for JetPattern {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = s
            .bytes()
            .map_while(|b| match b {
                b'<' => Some(Dir::Left),
                b'>' => Some(Dir::Right),
                _ => None,
            })
            .collect();
        Ok(Self(pattern))
    }
}

#[derive(Debug, Clone)]
struct Stone(Vec<Point>, usize);

impl Stone {
    fn spawn(index: usize, top: usize) -> Self {
        let (points, top) = match index % 5 {
            0 => (
                vec![
                    Point2D { x: 2, y: top + 4 },
                    Point2D { x: 3, y: top + 4 },
                    Point2D { x: 4, y: top + 4 },
                    Point2D { x: 5, y: top + 4 },
                ],
                top + 4,
            ),
            1 => (
                vec![
                    Point2D { x: 3, y: top + 4 },
                    Point2D { x: 2, y: top + 5 },
                    Point2D { x: 3, y: top + 5 },
                    Point2D { x: 4, y: top + 5 },
                    Point2D { x: 3, y: top + 6 },
                ],
                top + 6,
            ),
            2 => (
                vec![
                    Point2D { x: 2, y: top + 4 },
                    Point2D { x: 3, y: top + 4 },
                    Point2D { x: 4, y: top + 4 },
                    Point2D { x: 4, y: top + 5 },
                    Point2D { x: 4, y: top + 6 },
                ],
                top + 6,
            ),
            3 => (
                vec![
                    Point2D { x: 2, y: top + 4 },
                    Point2D { x: 2, y: top + 5 },
                    Point2D { x: 2, y: top + 6 },
                    Point2D { x: 2, y: top + 7 },
                ],
                top + 7,
            ),
            4 => (
                vec![
                    Point2D { x: 2, y: top + 4 },
                    Point2D { x: 3, y: top + 4 },
                    Point2D { x: 2, y: top + 5 },
                    Point2D { x: 3, y: top + 5 },
                ],
                top + 5,
            ),
            _ => unreachable!(),
        };
        Self(points, top)
    }

    fn apply_dir(&mut self, dir: Dir, blocked: &BTreeSet<Point>) {
        match dir {
            Dir::Left => {
                if self
                    .0
                    .iter()
                    .all(|p| p.x > 0 && !blocked.contains(&p.left()))
                {
                    for p in &mut self.0 {
                        *p = p.left();
                    }
                }
            }
            Dir::Right => {
                if self
                    .0
                    .iter()
                    .all(|p| p.x < 6 && !blocked.contains(&p.right()))
                {
                    for p in &mut self.0 {
                        *p = p.right();
                    }
                }
            }
        }
    }

    fn fall(&mut self, blocked: &BTreeSet<Point>) -> bool {
        if self
            .0
            .iter()
            .all(|p| p.y > 1 && !blocked.contains(&p.top()))
        {
            for p in &mut self.0 {
                *p = p.top();
            }
            self.1 -= 1;
            true
        } else {
            false
        }
    }

    fn stop(self, blocked: &mut BTreeSet<Point>) {
        for point in self.0 {
            blocked.insert(point);
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    top: usize,
    trimmed: usize,
    jet_pattern_index: usize,
    stone_index: usize,
    blocked: BTreeSet<Point>,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.jet_pattern_index == other.jet_pattern_index
            && self.stone_index == other.stone_index
            && self.blocked == other.blocked
    }
}

impl State {
    pub(super) fn new() -> Self {
        Self {
            top: 0,
            trimmed: 0,
            jet_pattern_index: 0,
            stone_index: 0,
            blocked: BTreeSet::new(),
        }
    }

    pub(super) fn height(&self) -> usize {
        self.top + self.trimmed
    }

    fn canonize(&mut self) -> usize {
        let mut lvl = self.top;
        while lvl > 1 {
            if (0..7).all(|x| {
                self.blocked.contains(&Point2D { x, y: lvl })
                    || self.blocked.contains(&Point2D { x, y: lvl - 1 })
            }) {
                break;
            }
            lvl -= 1;
        }
        if lvl < 2 {
            return 0;
        }
        lvl -= 2;
        self.top -= lvl;
        self.trimmed += lvl;

        self.blocked = self
            .blocked
            .iter()
            .filter_map(|p| {
                if p.y <= lvl {
                    None
                } else {
                    Some(Point2D {
                        x: p.x,
                        y: p.y - lvl,
                    })
                }
            })
            .collect();
        lvl
    }

    pub(super) fn drop_stone(&self, jet_pattern: &JetPattern) -> Self {
        let mut stone = Stone::spawn(self.stone_index, self.top);
        let mut jet_pattern_index = self.jet_pattern_index;
        let mut blocked = self.blocked.clone();
        let mut top = self.top;

        loop {
            let dir = jet_pattern.at(jet_pattern_index);
            jet_pattern_index = (jet_pattern_index + 1) % jet_pattern.len();

            stone.apply_dir(dir, &blocked);
            if !stone.fall(&blocked) {
                top = std::cmp::max(top, stone.1);
                stone.stop(&mut blocked);
                break;
            }
        }

        let mut new_state = Self {
            top,
            trimmed: self.trimmed,
            jet_pattern_index,
            stone_index: (self.stone_index + 1) % 5,
            blocked,
        };
        new_state.canonize();
        new_state
    }
}

pub(super) struct Puzzle {
    jet_pattern: JetPattern,
    repeat_start_index: usize,
    repeat_start_state: State,
    repeat_period: usize,
}

impl Puzzle {
    pub(super) fn compute_periodicity(jet_pattern: JetPattern) -> Self {
        let (repeat_period, repeat_start_state, repeat_start_index) =
            floyd(State::new(), |prev| prev.drop_stone(&jet_pattern));
        Self {
            jet_pattern,
            repeat_start_index,
            repeat_start_state,
            repeat_period,
        }
    }

    pub(super) fn drop_n(self, n: usize) -> usize {
        if n >= self.repeat_start_index + self.repeat_period {
            let mut state = self.repeat_start_state;
            let height_start = state.height();
            for _ in 0..self.repeat_period {
                state = state.drop_stone(&self.jet_pattern);
            }
            let period_height = state.height() - height_start;
            let n = n - self.repeat_start_index - self.repeat_period;
            state.trimmed += period_height * (n / self.repeat_period);
            let n = n % self.repeat_period;
            for _ in 0..n {
                state = state.drop_stone(&self.jet_pattern);
            }
            state.height()
        } else if n >= self.repeat_start_index {
            let n = n - self.repeat_start_index;
            let mut state = self.repeat_start_state;
            for _ in 0..n {
                state = state.drop_stone(&self.jet_pattern);
            }
            state.height()
        } else {
            let mut state = State::new();
            for _ in 0..n {
                state = state.drop_stone(&self.jet_pattern);
            }
            state.height()
        }
    }
}
