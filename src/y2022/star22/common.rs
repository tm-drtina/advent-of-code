use std::collections::HashMap;
use std::str::FromStr;

use anyhow::{anyhow, bail, Error, Result};

use crate::utils::point::{Dir, Point2D};

type Point = Point2D<usize>;

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

type Map = Vec<Vec<Node>>;

#[derive(Debug)]
pub(super) struct Puzzle {
    map: Map,
    commands: Vec<Command>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct State {
    pos: Point,
    dir: Dir,
}

impl State {
    fn new(map: &Map) -> Self {
        Self {
            pos: Point2D {
                x: map[0].iter().position(|n| matches!(n, Node::Empty)).unwrap(),
                y: 0,
            },
            dir: Dir::Right,
        }
    }

    fn go(&mut self, command: Command, map: &Map) {
            match command {
                Command::Step(n) => self.step(n, map),
                Command::Left => {
                    self.dir = self.dir.counterclockwise_90();
                }
                Command::Right => {
                    self.dir = self.dir.clockwise_90();
                }
        }
    }

    fn go_cube(&mut self, command: Command, transitions: &HashMap<State, State>) {
        match command {
            Command::Step(n) => {
                for _ in 0..n {
                    *self = transitions[self];
                }
            },
            Command::Left => {
                self.dir = self.dir.counterclockwise_90();
            }
            Command::Right => {
                self.dir = self.dir.clockwise_90();
            }
        }
    }

    fn step(&mut self, n: usize, map: &Map) {
        match self.dir {
            Dir::Right => {
                self.pos.x = map[self.pos.y]
                    .iter()
                    .enumerate()
                    .cycle()
                    .skip(self.pos.x + 1)
                    .filter(|(_i, n)| !matches!(n, Node::Missing))
                    .take_while(|(_i, n)| matches!(n, Node::Empty))
                    .take(n)
                    .map(|(index, _n)| index)
                    .last()
                    .unwrap_or(self.pos.x);
            }
            Dir::Bottom => {
                self.pos.y = map
                    .iter()
                    .enumerate()
                    .cycle()
                    .skip(self.pos.y + 1)
                    .filter(|(_i, n)| !matches!(n.get(self.pos.x), None | Some(Node::Missing)))
                    .take_while(|(_i, n)| matches!(n[self.pos.x], Node::Empty))
                    .take(n)
                    .map(|(index, _n)| index)
                    .last()
                    .unwrap_or(self.pos.y);
            }
            Dir::Left => {
                self.pos.x = map[self.pos.y]
                    .iter()
                    .enumerate()
                    .rev()
                    .cycle()
                    .skip(map[self.pos.y].len() - self.pos.x)
                    .filter(|(_i, n)| !matches!(n, Node::Missing))
                    .take_while(|(_i, n)| matches!(n, Node::Empty))
                    .take(n)
                    .map(|(index, _n)| index)
                    .last()
                    .unwrap_or(self.pos.x);
            }
            Dir::Top => {
                self.pos.y = map
                    .iter()
                    .enumerate()
                    .rev()
                    .cycle()
                    .skip(map.len() - self.pos.y)
                    .filter(|(_i, n)| !matches!(n.get(self.pos.x), None | Some(Node::Missing)))
                    .take_while(|(_i, n)| matches!(n[self.pos.x], Node::Empty))
                    .take(n)
                    .map(|(index, _n)| index)
                    .last()
                    .unwrap_or(self.pos.y);
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

    pub(super) fn password(&self) -> usize {
        1000 * (self.pos.y + 1) + 4 * (self.pos.x + 1) + Self::dir_value(self.dir)
    }
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

        let mut commands = Vec::new();
        let mut buffer = 0;
        for b in lines.next().ok_or(anyhow!("Invalid format"))?.bytes() {
            match b {
                b'0'..=b'9' => buffer = buffer * 10 + (b - b'0') as usize,
                b'R' => {
                    commands.push(Command::Step(buffer));
                    buffer = 0;
                    commands.push(Command::Right);
                }
                b'L' => {
                    commands.push(Command::Step(buffer));
                    buffer = 0;
                    commands.push(Command::Left);
                }
                _ => bail!("Invalid command"),
            }
        }
        commands.push(Command::Step(buffer));

        Ok(Self { map, commands })
    }
}

impl Puzzle {
    pub(super) fn go(self) -> State {
        let mut state = State::new(&self.map);
        for command in self.commands {
            state.go(command, &self.map);
        }
        state
    }

    fn transition_top(&self, transitions: &mut HashMap<State, State>, pos: Point) {
        let from = State {
            pos,
            dir: Dir::Top,
        };
        if pos.y > 0 {
            match self.map[pos.y - 1][pos.x] {
                Node::Wall => {
                    transitions.insert(from, from);
                    return
                },
                Node::Empty => {
                    transitions.insert(from, State { pos: Point2D { x: pos.x, y: pos.y - 1 }, dir: Dir::Top });
                    return
                },
                Node::Missing => {
                    todo!("wrap")
                },
            }
        }
        todo!("wrap")
    }
    fn transition_right(&self, transitions: &mut HashMap<State, State>, pos: Point) {
        let from = State {
            pos,
            dir: Dir::Right,
        };
        if pos.x < self.map[pos.y].len() - 1 {
            match self.map[pos.y][pos.x + 1] {
                Node::Wall => {
                    transitions.insert(from, from);
                    return
                },
                Node::Empty => {
                    transitions.insert(from, State { pos: Point2D { x: pos.x + 1, y: pos.y }, dir: Dir::Right });
                    return
                },
                Node::Missing => {
                    todo!("wrap")
                },
            }
        }
        todo!("wrap")
    }
    fn transition_bottom(&self, transitions: &mut HashMap<State, State>, pos: Point) {
        let from = State {
            pos,
            dir: Dir::Bottom,
        };
        if pos.y < self.map.len() - 1 {
            match self.map[pos.y + 1][pos.x] {
                Node::Wall => {
                    transitions.insert(from, from);
                    return
                },
                Node::Empty => {
                    transitions.insert(from, State { pos: Point2D { x: pos.x, y: pos.y + 1 }, dir: Dir::Bottom });
                    return
                },
                Node::Missing => {
                    todo!("wrap")
                },
            }
        }
        todo!("wrap")
    }
    fn transition_left(&self, transitions: &mut HashMap<State, State>, pos: Point) {
        let from = State {
            pos,
            dir: Dir::Left,
        };
        if pos.x > 0 {
            match self.map[pos.y][pos.x - 1] {
                Node::Wall => {
                    transitions.insert(from, from);
                    return
                },
                Node::Empty => {
                    transitions.insert(from, State { pos: Point2D { x: pos.x - 1, y: pos.y }, dir: Dir::Left });
                    return
                },
                Node::Missing => {
                    todo!("wrap")
                },
            }
        }
        todo!("wrap")
    }

    fn transitions(&self) -> HashMap<State, State> {
        let mut transitions = HashMap::new();
        let count = self.map.iter().flatten().filter(|n| matches!(n, Node::Empty | Node::Wall)).count();
        let side_len = (1usize..).find(|i| i * i * 6 == count).unwrap();
        for (y, row) in self.map.iter().enumerate() {
            for (x, _node) in row.iter().enumerate().filter(|(_, node)| matches!(node, Node::Empty)) {
                let pos = Point2D { x, y };
                self.transition_top(&mut transitions, pos);
                self.transition_right(&mut transitions, pos);
                self.transition_bottom(&mut transitions, pos);
                self.transition_left(&mut transitions, pos);
            }
        }
        transitions
    }

    pub(super) fn go_cube(self) -> State {
        let mut state = State::new(&self.map);
        let transitions = self.transitions();
        for command in self.commands {
            state.go_cube(command, &transitions);
        }
        state
    }
}
