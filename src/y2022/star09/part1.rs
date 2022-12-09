use std::collections::HashSet;

use anyhow::{anyhow, bail, Result};

use crate::utils::point::Point2D;

type Point = Point2D<i32>;

struct Puzzle {
    history: HashSet<Point>,
    head: Point,
    tail: Point,
}

impl Puzzle {
    fn new() -> Self {
        let zero = Point { x: 0, y: 0 };
        let mut history = HashSet::new();
        history.insert(zero);
        Self {
            history,
            head: zero,
            tail: zero,
        }
    }

    fn left(&mut self) {
        self.head = self.head.left();
        if self.head.x + 2 == self.tail.x {
            self.tail = self.head.right();
            self.history.insert(self.tail);
        }
    }

    fn right(&mut self) {
        self.head = self.head.right();
        if self.head.x == self.tail.x + 2 {
            self.tail = self.head.left();
            self.history.insert(self.tail);
        }
    }

    fn up(&mut self) {
        self.head = self.head.top();
        if self.head.y + 2 == self.tail.y {
            self.tail = self.head.bottom();
            self.history.insert(self.tail);
        }
    }

    fn down(&mut self) {
        self.head = self.head.bottom();
        if self.head.y == self.tail.y + 2 {
            self.tail = self.head.top();
            self.history.insert(self.tail);
        }
    }

    fn step(&mut self, line: &str) -> Result<()> {
        let (dir, amount) = line.split_once(' ').ok_or(anyhow!("Invalid format"))?;
        let amount = amount.parse::<usize>()?;
        match dir {
            "L" => (0..amount).for_each(|_| self.left()),
            "R" => (0..amount).for_each(|_| self.right()),
            "U" => (0..amount).for_each(|_| self.up()),
            "D" => (0..amount).for_each(|_| self.down()),
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
