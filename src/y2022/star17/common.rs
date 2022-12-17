use std::collections::HashSet;
use std::str::FromStr;

use anyhow::{Error, Result};

use crate::utils::point::Point2D;

type Point = Point2D<usize>;

#[derive(Debug, Clone, Copy)]
pub(super) enum Dir {
    Left,
    Right,
}

pub(super) struct JetPattern(Vec<Dir>);

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

impl JetPattern {
    pub(super) fn len(&self) -> usize {
        self.0.len()
    }

    pub(super) fn iter(&self) -> impl Iterator<Item = Dir> + '_ {
        self.0.iter().copied().cycle()
    }
}

#[derive(Debug, Clone)]
struct Stone(Vec<Point>, usize);

impl Stone {
    fn spawn(index: usize, top: usize) -> Self {
        let (points, top) = match index % 5 {
            0 => (vec![
                Point2D { x: 2, y: top + 4 },
                Point2D { x: 3, y: top + 4 },
                Point2D { x: 4, y: top + 4 },
                Point2D { x: 5, y: top + 4 },
            ], top+4),
            1 => (vec![
                Point2D { x: 3, y: top + 4 },
                Point2D { x: 2, y: top + 5 },
                Point2D { x: 3, y: top + 5 },
                Point2D { x: 4, y: top + 5 },
                Point2D { x: 3, y: top + 6 },
            ], top+6),
            2 => (vec![
                Point2D { x: 2, y: top + 4 },
                Point2D { x: 3, y: top + 4 },
                Point2D { x: 4, y: top + 4 },
                Point2D { x: 4, y: top + 5 },
                Point2D { x: 4, y: top + 6 },
            ], top+6),
            3 => (vec![
                Point2D { x: 2, y: top + 4 },
                Point2D { x: 2, y: top + 5 },
                Point2D { x: 2, y: top + 6 },
                Point2D { x: 2, y: top + 7 },
            ], top+7),
            4 => (vec![
                Point2D { x: 2, y: top + 4 },
                Point2D { x: 3, y: top + 4 },
                Point2D { x: 2, y: top + 5 },
                Point2D { x: 3, y: top + 5 },
            ], top+5),
            _ => unreachable!()
        };
        Self(points, top)
    }

    fn apply_dir(&mut self, dir: Dir, blocked: &HashSet<Point>) {
        match dir {
            Dir::Left => {
                if self.0.iter().all(|p| { 
                    p.x > 0 && !blocked.contains(&p.left())
                }) {
                    for p in self.0.iter_mut() {
                        *p = p.left();
                    }
                }
            },
            Dir::Right => {
                if self.0.iter().all(|p| { 
                    p.x < 6 && !blocked.contains(&p.right())
                }) {
                    for p in self.0.iter_mut() {
                        *p = p.right();
                    }
                }
            },
        }
    }

    fn fall(&mut self, blocked: &HashSet<Point>) -> bool {
        if self.0.iter().all(|p| { 
            p.y > 1 && !blocked.contains(&p.top())
        }) {
            for p in self.0.iter_mut() {
                *p = p.top();
            }
            self.1 -= 1;
            true
        } else {
            false
        }
    }

    fn stop(self, blocked: &mut HashSet<Point>) {
        for point in self.0 {
            blocked.insert(point);
        }
    }
}

pub(super) struct Cave<T> {
    top: usize,
    blocked: HashSet<Point>,
    jet_pattern: T,
}

impl<T> Cave<T>
where
    T: Iterator<Item = Dir>,
{
    pub(super) fn new(jet_pattern: T) -> Self {
        Self {
            top: 0,
            blocked: HashSet::new(),
            jet_pattern,
        }
    }

    pub(super) fn height(&self) -> usize {
        self.top
    }

    pub(super) fn drop_stone(&mut self, index: usize) {
        let mut stone = Stone::spawn(index, self.top);

        if index % 1000 == 0 {
            self.blocked.retain(|p| {
                p.y + 1000 > self.top
            })
        }

        loop {
            let dir = self.jet_pattern.next().unwrap();
            stone.apply_dir(dir, &self.blocked);
            if !stone.fall(&self.blocked) {
                let top = stone.1;
                if self.top < top {
                    self.top = top;
                }
                stone.stop(&mut self.blocked);
                break
            }
        };
    }
}

impl<T> std::fmt::Display for Cave<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "+^^^^^^^+")?;
        for y in (1..self.top+3).rev() {
            write!(f, "|")?;
            for x in 0..7 {
                if self.blocked.contains(&Point2D { x, y }) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "+-------+")
    }
}
