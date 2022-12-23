use std::collections::VecDeque;
use std::str::FromStr;

use anyhow::{anyhow, bail, Error, Result};

use crate::utils::point::Dir;

#[derive(Debug, Clone, Copy)]
enum Command {
    Step(usize),
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Node {
    Wall,
    Empty,
    Missing,
}

#[derive(Debug)]
struct Puzzle {
    map: Vec<Vec<Node>>,
    commands: VecDeque<Command>,
    y: usize,
    x: usize,
    dir: Dir,
}

impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Vec::new();
        let mut lines = s.lines();
        loop {
            let line = lines.next().ok_or(anyhow!("Invalid format"))?;
            if line.is_empty() {
                break;
            }
            map.push(
                line.bytes()
                    .map(|b| {
                        Ok(match b {
                            b'.' => Node::Empty,
                            b'#' => Node::Wall,
                            b' ' => Node::Missing,
                            _ => bail!("Invalid format"),
                        })
                    })
                    .collect::<Result<Vec<_>>>()?,
            );
        }
        let x = map[0]
            .iter()
            .position(|n| matches!(n, Node::Empty))
            .ok_or(anyhow!("Cannot find start"))?;

        let mut commands = VecDeque::new();
        let mut buffer = 0;
        for b in lines.next().ok_or(anyhow!("Invalid format"))?.bytes() {
            match b {
                b'0'..=b'9' => buffer = buffer * 10 + (b - b'0') as usize,
                b'R' => {
                    commands.push_back(Command::Step(buffer));
                    buffer = 0;
                    commands.push_back(Command::Right);
                }
                b'L' => {
                    commands.push_back(Command::Step(buffer));
                    buffer = 0;
                    commands.push_back(Command::Left);
                }
                _ => bail!("Invalid command"),
            }
        }
        commands.push_back(Command::Step(buffer));

        Ok(Self {
            map,
            commands,
            y: 0,
            x,
            dir: Dir::Right,
        })
    }
}

impl Puzzle {
    fn go(&mut self) {
        while let Some(command) = self.commands.pop_front() {
            match command {
                Command::Step(n) => self.step(n),
                Command::Left => {
                    self.dir = self.dir.counterclockwise_90();
                }
                Command::Right => {
                    self.dir = self.dir.clockwise_90();
                }
            }
        }
    }

    fn step(&mut self, n: usize) {
        match self.dir {
            Dir::Right => {
                self.x = self.map[self.y]
                    .iter()
                    .enumerate()
                    .cycle()
                    .skip(self.x + 1)
                    .filter(|(_i, n)| !matches!(n, Node::Missing))
                    .take_while(|(_i, n)| matches!(n, Node::Empty))
                    .take(n)
                    .map(|(index, _n)| index)
                    .last()
                    .unwrap_or(self.x);
            }
            Dir::Bottom => {
                self.y = self
                    .map
                    .iter()
                    .enumerate()
                    .cycle()
                    .skip(self.y + 1)
                    .filter(|(_i, n)| !matches!(n.get(self.x), None | Some(Node::Missing)))
                    .take_while(|(_i, n)| matches!(n[self.x], Node::Empty))
                    .take(n)
                    .map(|(index, _n)| index)
                    .last()
                    .unwrap_or(self.y);
            }
            Dir::Left => {
                self.x = self.map[self.y]
                    .iter()
                    .enumerate()
                    .rev()
                    .cycle()
                    .skip(self.map[self.y].len() - self.x)
                    .filter(|(_i, n)| !matches!(n, Node::Missing))
                    .take_while(|(_i, n)| matches!(n, Node::Empty))
                    .take(n)
                    .map(|(index, _n)| index)
                    .last()
                    .unwrap_or(self.x);
            }
            Dir::Top => {
                self.y = self
                    .map
                    .iter()
                    .enumerate()
                    .rev()
                    .cycle()
                    .skip(self.map.len() - self.y)
                    .filter(|(_i, n)| !matches!(n.get(self.x), None | Some(Node::Missing)))
                    .take_while(|(_i, n)| matches!(n[self.x], Node::Empty))
                    .take(n)
                    .map(|(index, _n)| index)
                    .last()
                    .unwrap_or(self.y);
            }
            _ => unreachable!(),
        }
    }

    fn dir_value(dir: Dir) -> usize {
        match dir {
            Dir::Right => 0,
            Dir::Bottom => 1,
            Dir::Left => 2,
            Dir::Top => 3,
            _ => unreachable!(),
        }
    }

    fn password(&self) -> usize {
        1000 * (self.y + 1) + 4 * (self.x + 1) + Self::dir_value(self.dir)
    }
}

pub fn run(input: &str) -> Result<usize> {
    let mut puzzle: Puzzle = input.parse()?;
    puzzle.go();
    Ok(puzzle.password())
}
