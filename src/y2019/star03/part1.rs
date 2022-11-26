use std::collections::HashSet;

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
    fn manhattan_dist(self) -> i32 {
        self.x.abs() + self.y.abs()
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
    fn step(self, from: Point) -> Point {
        match self.direction {
            Direction::Up => Point {
                x: from.x,
                y: from.y + 1,
            },
            Direction::Down => Point {
                x: from.x,
                y: from.y - 1,
            },
            Direction::Left => Point {
                x: from.x - 1,
                y: from.y,
            },
            Direction::Right => Point {
                x: from.x + 1,
                y: from.y,
            },
        }
    }

    fn traverse(mut acc: Traverse, value: Self) -> Traverse {
        for _ in 0..value.distance {
            acc.curr_point = value.step(acc.curr_point);
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
        .reduce(|set1, set2| set1.intersection(&set2).copied().collect())
        .unwrap()
        .into_iter()
        .map(Point::manhattan_dist)
        .min()
        .unwrap()
}
