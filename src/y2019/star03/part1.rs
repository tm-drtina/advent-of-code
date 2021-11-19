use std::collections::HashSet;
use std::str::FromStr;

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl From<&str> for Direction {
    fn from(val: &str) -> Self {
        match val {
            "U" => Direction::UP,
            "D" => Direction::DOWN,
            "L" => Direction::LEFT,
            "R" => Direction::RIGHT,
            _ => panic!("Unrecognized direction {}", val),
        }
    }
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_dist(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Default)]
struct Traverse {
    curr_point: Point,
    visited: HashSet<Point>,
}

struct Move {
    direction: Direction,
    distance: i32,
}

impl From<&str> for Move {
    fn from(val: &str) -> Self {
        Self {
            direction: Direction::from(&val[0..1]),
            distance: i32::from_str(&val[1..]).expect("Unable to parse distance"),
        }
    }
}

impl Move {
    fn step(&self, from: &Point) -> Point {
        match self.direction {
            Direction::UP => Point {
                x: from.x,
                y: from.y + 1,
            },
            Direction::DOWN => Point {
                x: from.x,
                y: from.y - 1,
            },
            Direction::LEFT => Point {
                x: from.x - 1,
                y: from.y,
            },
            Direction::RIGHT => Point {
                x: from.x + 1,
                y: from.y,
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
            line.split(",")
                .map(|str_move| Move::from(str_move))
                .fold(Traverse::default(), Move::traverse)
                .visited
        })
        .reduce(|set1, set2| set1.intersection(&set2).cloned().collect())
        .unwrap()
        .iter()
        .map(|p| p.manhattan_dist())
        .min()
        .unwrap()
}
