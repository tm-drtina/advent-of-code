use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(val: &str) -> Self {
        match val {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Unrecognized direction {val}"),
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
    time: i32,
}

impl Eq for Point {}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(self.x);
        state.write_i32(self.y);
    }
}

#[derive(Default)]
struct Traverse {
    curr_point: Point,
    visited: HashSet<Point>,
}

#[derive(Debug, Clone, Copy)]
struct Move {
    direction: Direction,
    distance: i32,
}

impl From<&str> for Move {
    fn from(val: &str) -> Self {
        Self {
            direction: val[0..1].into(),
            distance: val[1..].parse().expect("Unable to parse distance"),
        }
    }
}

impl Move {
    fn step(self, from: &Point) -> Point {
        match self.direction {
            Direction::Up => Point {
                x: from.x,
                y: from.y + 1,
                time: from.time + 1,
            },
            Direction::Down => Point {
                x: from.x,
                y: from.y - 1,
                time: from.time + 1,
            },
            Direction::Left => Point {
                x: from.x - 1,
                y: from.y,
                time: from.time + 1,
            },
            Direction::Right => Point {
                x: from.x + 1,
                y: from.y,
                time: from.time + 1,
            },
        }
    }

    fn traverse(mut acc: Traverse, value: Self) -> Traverse {
        for _ in 0..value.distance {
            acc.curr_point = value.step(&acc.curr_point);
            acc.visited.insert(acc.curr_point);
        }
        acc
    }
}

pub fn run(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(Move::from)
                .fold(Traverse::default(), Move::traverse)
                .visited
        })
        .reduce(|set1, set2| {
            set1.iter()
                .filter_map(|p| {
                    let other = set2.get(p)?;
                    Some(Point {
                        x: p.x,
                        y: p.y,
                        time: p.time + other.time,
                    })
                })
                .collect()
        })
        .unwrap()
        .iter()
        .map(|p| p.time)
        .min()
        .unwrap()
}
