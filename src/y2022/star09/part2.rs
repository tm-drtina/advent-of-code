use std::collections::HashSet;

use anyhow::{anyhow, bail, Result};

use crate::utils::point::Point2D;

type Point = Point2D<i32>;

struct Puzzle {
    history: HashSet<Point>,
    positions: [Point; 10],
}

impl Puzzle {
    fn new() -> Self {
        let zero = Point { x: 0, y: 0 };
        let mut history = HashSet::new();
        history.insert(zero);
        Self {
            history,
            positions: [zero; 10],
        }
    }

    fn left(&mut self, index: usize) {
        self.positions[index] = self.positions[index].left();
        if index == self.positions.len() - 1 {
            self.history.insert(self.positions[index]);
        } else if self.positions[index].x + 2 == self.positions[index + 1].x {
            match self.positions[index].y.cmp(&self.positions[index + 1].y) {
                std::cmp::Ordering::Less => self.up_left(index + 1),
                std::cmp::Ordering::Equal => self.left(index + 1),
                std::cmp::Ordering::Greater => self.down_left(index + 1),
            }
        }
    }

    fn right(&mut self, index: usize) {
        self.positions[index] = self.positions[index].right();
        if index == self.positions.len() - 1 {
            self.history.insert(self.positions[index]);
        } else if self.positions[index].x == self.positions[index + 1].x + 2 {
            match self.positions[index].y.cmp(&self.positions[index + 1].y) {
                std::cmp::Ordering::Less => self.up_right(index + 1),
                std::cmp::Ordering::Equal => self.right(index + 1),
                std::cmp::Ordering::Greater => self.down_right(index + 1),
            }
        }
    }

    fn up(&mut self, index: usize) {
        self.positions[index] = self.positions[index].top();
        if index == self.positions.len() - 1 {
            self.history.insert(self.positions[index]);
        } else if self.positions[index].y + 2 == self.positions[index + 1].y {
            match self.positions[index].x.cmp(&self.positions[index + 1].x) {
                std::cmp::Ordering::Less => self.up_left(index + 1),
                std::cmp::Ordering::Equal => self.up(index + 1),
                std::cmp::Ordering::Greater => self.up_right(index + 1),
            }
        }
    }

    fn down(&mut self, index: usize) {
        self.positions[index] = self.positions[index].bottom();
        if index == self.positions.len() - 1 {
            self.history.insert(self.positions[index]);
        } else if self.positions[index].y == self.positions[index + 1].y + 2 {
            match self.positions[index].x.cmp(&self.positions[index + 1].x) {
                std::cmp::Ordering::Less => self.down_left(index + 1),
                std::cmp::Ordering::Equal => self.down(index + 1),
                std::cmp::Ordering::Greater => self.down_right(index + 1),
            }
        }
    }

    fn up_left(&mut self, index: usize) {
        self.positions[index] = self.positions[index].top_left();
        if index == self.positions.len() - 1 {
            self.history.insert(self.positions[index]);
        } else if self.positions[index].x + 2 == self.positions[index + 1].x {
            match self.positions[index].y.cmp(&self.positions[index + 1].y) {
                std::cmp::Ordering::Less => self.up_left(index + 1),
                std::cmp::Ordering::Equal => self.left(index + 1),
                std::cmp::Ordering::Greater => unreachable!("up_left goes up_right"),
            }
        } else if self.positions[index].y + 2 == self.positions[index + 1].y {
            match self.positions[index].x.cmp(&self.positions[index + 1].x) {
                std::cmp::Ordering::Less => self.up_left(index + 1),
                std::cmp::Ordering::Equal => self.up(index + 1),
                std::cmp::Ordering::Greater => unreachable!("up_left goes up_right"),
            }
        }
    }

    fn up_right(&mut self, index: usize) {
        self.positions[index] = self.positions[index].top_right();
        if index == self.positions.len() - 1 {
            self.history.insert(self.positions[index]);
        } else if self.positions[index].x == self.positions[index + 1].x + 2 {
            match self.positions[index].y.cmp(&self.positions[index + 1].y) {
                std::cmp::Ordering::Less => self.up_right(index + 1),
                std::cmp::Ordering::Equal => self.right(index + 1),
                std::cmp::Ordering::Greater => unreachable!("up_right goes down_right"),
            }
        } else if self.positions[index].y + 2 == self.positions[index + 1].y {
            match self.positions[index].x.cmp(&self.positions[index + 1].x) {
                std::cmp::Ordering::Less => unreachable!("up_right goes up_left"),
                std::cmp::Ordering::Equal => self.up(index + 1),
                std::cmp::Ordering::Greater => self.up_right(index + 1),
            }
        }
    }

    fn down_left(&mut self, index: usize) {
        self.positions[index] = self.positions[index].bottom_left();
        if index == self.positions.len() - 1 {
            self.history.insert(self.positions[index]);
        } else if self.positions[index].x + 2 == self.positions[index + 1].x {
            match self.positions[index].y.cmp(&self.positions[index + 1].y) {
                std::cmp::Ordering::Less => unreachable!("down_left goes up_left"),
                std::cmp::Ordering::Equal => self.left(index + 1),
                std::cmp::Ordering::Greater => self.down_left(index + 1),
            }
        } else if self.positions[index].y == self.positions[index + 1].y + 2 {
            match self.positions[index].x.cmp(&self.positions[index + 1].x) {
                std::cmp::Ordering::Less => self.down_left(index + 1),
                std::cmp::Ordering::Equal => self.down(index + 1),
                std::cmp::Ordering::Greater => unreachable!("down_left goes down_right"),
            }
        }
    }

    fn down_right(&mut self, index: usize) {
        self.positions[index] = self.positions[index].bottom_right();
        if index == self.positions.len() - 1 {
            self.history.insert(self.positions[index]);
        } else if self.positions[index].x == self.positions[index + 1].x + 2 {
            match self.positions[index].y.cmp(&self.positions[index + 1].y) {
                std::cmp::Ordering::Less => unreachable!("down_right goes up_right"),
                std::cmp::Ordering::Equal => self.right(index + 1),
                std::cmp::Ordering::Greater => self.down_right(index + 1),
            }
        } else if self.positions[index].y == self.positions[index + 1].y + 2 {
            match self.positions[index].x.cmp(&self.positions[index + 1].x) {
                std::cmp::Ordering::Less => unreachable!("down_right goes down_left"),
                std::cmp::Ordering::Equal => self.down(index + 1),
                std::cmp::Ordering::Greater => self.down_right(index + 1),
            }
        }
    }

    fn step(&mut self, line: &str) -> Result<()> {
        let (dir, amount) = line
            .split_once(' ')
            .ok_or_else(|| anyhow!("Invalid format"))?;
        let amount = amount.parse::<usize>()?;
        match dir {
            "L" => (0..amount).for_each(|_| self.left(0)),
            "R" => (0..amount).for_each(|_| self.right(0)),
            "U" => (0..amount).for_each(|_| self.up(0)),
            "D" => (0..amount).for_each(|_| self.down(0)),
            _ => bail!("Invalid direction {}", dir),
        }
        Ok(())
    }
}

pub fn run(input: &str) -> Result<usize> {
    let mut puzzle = Puzzle::new();
    input.lines().try_for_each(|line| puzzle.step(line))?;
    Ok(puzzle.history.len())
}
