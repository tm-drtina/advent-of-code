use std::collections::{HashMap, BTreeSet};
use std::iter::Cycle;
use std::str::FromStr;

use anyhow::{Error, Result};
use num::Integer;

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
                    for p in self.0.iter_mut() {
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
                    for p in self.0.iter_mut() {
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
            for p in self.0.iter_mut() {
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

pub(super) struct Cave {
    top: usize,
    trimmed: usize,
    blocked: BTreeSet<Point>,
    pub repeat: usize,
    jet_pattern_iter: Cycle<std::vec::IntoIter<Dir>>,
    cache: HashMap<(usize, BTreeSet<Point>), (usize, BTreeSet<Point>)>,
}

impl Cave {
    pub(super) fn new(jet_pattern: JetPattern) -> Self {
        let repeat = jet_pattern.0.len().lcm(&5);
        eprintln!("repeat: {repeat}");
        Self {
            top: 0,
            trimmed: 0,
            blocked: BTreeSet::new(),
            repeat,
            jet_pattern_iter: jet_pattern.0.into_iter().cycle(),
            cache: HashMap::new(),
        }
    }

    pub(super) fn height(&self) -> usize {
        self.top + self.trimmed
    }

    pub(super) fn drop_stone(&mut self, index: usize) {
        let mut stone = Stone::spawn(index, self.top);

        loop {
            let dir = self.jet_pattern_iter.next().unwrap();
            stone.apply_dir(dir, &self.blocked);
            if !stone.fall(&self.blocked) {
                let top = stone.1;
                if self.top < top {
                    self.top = top;
                }
                stone.stop(&mut self.blocked);
                break;
            }
        }
    }

    pub(super) fn drop_batch(&mut self, size: usize) -> usize {
        let key = (size, std::mem::take(&mut self.blocked));
        let cache_entry = self.cache.get(&key);
        self.blocked = key.1;
        if let Some((height, blocked)) = cache_entry {
            self.blocked = blocked.clone();
            self.trimmed += height;
            *height
        } else if size > 1 {
            let key = self.blocked.clone();
            let mut res = 0;
            if size > 1 {
                res += self.drop_batch(size / 2);
                res += self.drop_batch(size / 2);
            }
            if size & 1 == 1 {
                res += self.drop_batch(1);
            }
            self.cache.insert((size, key), (res, self.blocked.clone()));
            res
        } else {
            let key = self.blocked.clone();
            for i in 0..self.repeat {
                self.drop_stone(i);
            }
            let value = self.canonize();
            self.cache.insert((1, key), (value, self.blocked.clone()));
            value
        }
    }

    pub(crate) fn drop_n(&mut self, n: usize) {
        if n >= self.repeat {
            self.drop_batch(n / self.repeat);
        }
        for i in 0..(n % self.repeat) {
            self.drop_stone(i);
        }
    }

    pub(super) fn canonize(&mut self) -> usize {
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
}

impl std::fmt::Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "+^^^^^^^+")?;
        for y in (1..self.top + 3).rev() {
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
