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
                x: map[0]
                    .iter()
                    .position(|n| matches!(n, Node::Empty))
                    .unwrap(),
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
            let line = lines.next().ok_or_else(|| anyhow!("Invalid format"))?;
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
        for b in lines.next().ok_or_else(|| anyhow!("Invalid format"))?.bytes() {
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

    pub(super) fn into_cube(self) -> Cube {
        Cube::new(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Transition {
    OuterEdge,
    Straight,
    InnerEdge,
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    start: Point,
    end: Point,
    transition: Transition,
    dir: Dir,
}

pub(super) struct Cube {
    puzzle: Puzzle,
    teleports: HashMap<State, State>,
}

impl Cube {
    pub(super) fn new(puzzle: Puzzle) -> Self {
        let teleports = Self::init_teleports(&puzzle);
        Self { puzzle, teleports }
    }

    pub(super) fn go(self) -> State {
        let mut state = State::new(&self.puzzle.map);
        for command in self.puzzle.commands {
            match command {
                Command::Step(n) => {
                    for _ in 0..n {
                        state = self.teleports.get(&state).copied().unwrap_or_else(|| {
                            let pos = state.pos.step_dir(state.dir);
                            if matches!(self.puzzle.map[pos.y][pos.x], Node::Empty) {
                                State {
                                    pos: state.pos.step_dir(state.dir),
                                    dir: state.dir,
                                }
                            } else {
                                state
                            }
                        });
                    }
                }
                Command::Left => {
                    state.dir = state.dir.counterclockwise_90();
                }
                Command::Right => {
                    state.dir = state.dir.clockwise_90();
                }
            }
        }
        state
    }

    fn next_edge(puzzle: &Puzzle, edge: &mut Edge, side_len: usize) {
        // try inner edge
        if let Some(pt @ Point2D { x, y }) = edge
            .end
            .try_step_dir(edge.dir)
            .and_then(|p| p.try_step_dir(edge.dir.clockwise_90()))
        {
            if puzzle.map.get(y).map_or(false, |row| {
                matches!(row.get(x), Some(Node::Empty | Node::Wall))
            }) {
                let mut end = pt;
                for _ in 1..side_len {
                    end = end.step_dir(edge.dir);
                }
                *edge = Edge {
                    start: pt,
                    end,
                    transition: Transition::InnerEdge,
                    dir: edge.dir.counterclockwise_90(),
                };
                return;
            }
        }

        // try straight
        if let Some(pt @ Point2D { x, y }) = edge.end.try_step_dir(edge.dir.clockwise_90()) {
            if puzzle.map.get(y).map_or(false, |row| {
                matches!(row.get(x), Some(Node::Empty | Node::Wall))
            }) {
                let mut end = pt;
                for _ in 1..side_len {
                    end = end.step_dir(edge.dir.clockwise_90());
                }
                *edge = Edge {
                    start: pt,
                    end,
                    transition: Transition::Straight,
                    dir: edge.dir,
                };
                return;
            }
        }

        // outer edge
        let mut end = edge.end;
        for _ in 1..side_len {
            end = end.step_dir(edge.dir.clockwise_90().clockwise_90());
        }
        *edge = Edge {
            start: edge.end,
            end,
            transition: Transition::OuterEdge,
            dir: edge.dir.clockwise_90(),
        };
    }

    fn gen_teleports(e1: Edge, e2: Edge, side_len: usize, puzzle: &Puzzle, teleports: &mut HashMap<State, State>) {
        let mut pt1 = e1.start;
        let mut pt2 = e2.end;
        for i in 0..side_len {
            if matches!(puzzle.map[pt2.y][pt2.x], Node::Wall) {
                teleports.insert(
                    State {
                        pos: pt1,
                        dir: e1.dir,
                    },
                    State {
                        pos: pt1,
                        dir: e1.dir,
                    },
                );
            } else {
                teleports.insert(
                    State {
                        pos: pt1,
                        dir: e1.dir,
                    },
                    State {
                        pos: pt2,
                        dir: e2.dir.clockwise_90().clockwise_90(),
                    },
                );
            }
            if matches!(puzzle.map[pt1.y][pt1.x], Node::Wall) {
                teleports.insert(
                    State {
                        pos: pt2,
                        dir: e2.dir,
                    },
                    State {
                        pos: pt2,
                        dir: e2.dir,
                    },
                );
            } else {
                teleports.insert(
                    State {
                        pos: pt2,
                        dir: e2.dir,
                    },
                    State {
                        pos: pt1,
                        dir: e1.dir.clockwise_90().clockwise_90(),
                    },
                );
            }
            if i < side_len - 1 {
                pt1 = pt1.step_dir(e1.dir.clockwise_90());
                pt2 = pt2.step_dir(e2.dir.counterclockwise_90());
            }
        }
    }

    fn init_teleports(puzzle: &Puzzle) -> HashMap<State, State> {
        let mut teleports = HashMap::new();
        let count = puzzle
            .map
            .iter()
            .flatten()
            .filter(|n| matches!(n, Node::Empty | Node::Wall))
            .count();
        let side_len = (1usize..).find(|i| i * i * 6 == count).unwrap();
        let start_x = puzzle.map[0]
            .iter()
            .position(|n| matches!(n, Node::Empty))
            .unwrap();

        let mut edge = Edge {
            start: Point2D { x: start_x, y: 0 },
            end: Point2D {
                x: start_x + side_len - 1,
                y: 0,
            },
            dir: Dir::Top,
            transition: Transition::OuterEdge,
        };
        let mut stack = vec![edge];
        let mut end_transitions = vec![Transition::OuterEdge];
        let mut pull = false;
        let mut tps = 0;

        while tps < 7 {
            Self::next_edge(puzzle, &mut edge, side_len);
            match edge.transition {
                Transition::OuterEdge => {
                    if pull
                        && !stack.is_empty()
                        && end_transitions.last() == Some(&Transition::Straight)
                    {
                        Self::gen_teleports(stack.pop().unwrap(), edge, side_len, puzzle, &mut teleports);
                        end_transitions.pop();
                        tps += 1;
                    } else {
                        pull = false;
                        stack.push(edge);
                        end_transitions.push(Transition::OuterEdge);
                    }
                }
                Transition::Straight => {
                    if pull && !stack.is_empty() {
                        Self::gen_teleports(stack.pop().unwrap(), edge, side_len, puzzle, &mut teleports);
                        end_transitions.pop();
                        tps += 1;
                    } else {
                        pull = false;
                        stack.push(edge);
                        end_transitions.push(Transition::Straight);
                    }
                }
                Transition::InnerEdge => {
                    pull = true;
                    Self::gen_teleports(stack.pop().unwrap(), edge, side_len, puzzle, &mut teleports);
                    tps += 1;
                }
            }
        }
        teleports
    }
}
