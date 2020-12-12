use std::str::FromStr;

#[derive(Copy, Clone)]
enum Direction {
    EAST,
    SOUTH,
    WEST,
    NORTH,
}

struct Position {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Position {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            direction: Direction::EAST,
        }
    }
    fn with_position(&self, x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            direction: self.direction,
        }
    }
    fn with_direction(self, direction: Direction) -> Self {
        Self {
            x: self.x,
            y: self.y,
            direction,
        }
    }

    fn go(&self, direction: Direction, len: i32) -> Self {
        match direction {
            Direction::EAST => self.with_position(self.x + len, self.y),
            Direction::SOUTH => self.with_position(self.x, self.y - len),
            Direction::WEST => self.with_position(self.x - len, self.y),
            Direction::NORTH => self.with_position(self.x, self.y + len),
        }
    }

    fn left(self, num: i32) -> Self {
        if num == 0 {
            self
        } else {
            match self.direction {
                Direction::EAST => self.with_direction(Direction::NORTH).left(num - 90),
                Direction::SOUTH => self.with_direction(Direction::EAST).left(num - 90),
                Direction::WEST => self.with_direction(Direction::SOUTH).left(num - 90),
                Direction::NORTH => self.with_direction(Direction::WEST).left(num - 90),
            }
        }
    }

    fn right(self, num: i32) -> Self {
        if num == 0 {
            self
        } else {
            match self.direction {
                Direction::EAST => self.with_direction(Direction::SOUTH).right(num - 90),
                Direction::SOUTH => self.with_direction(Direction::WEST).right(num - 90),
                Direction::WEST => self.with_direction(Direction::NORTH).right(num - 90),
                Direction::NORTH => self.with_direction(Direction::EAST).right(num - 90),
            }
        }
    }

    fn step(self, op: char, num: i32) -> Self {
        match op {
            'N' => self.go(Direction::NORTH, num),
            'S' => self.go(Direction::SOUTH, num),
            'E' => self.go(Direction::EAST, num),
            'W' => self.go(Direction::WEST, num),
            'L' => self.left(num),
            'R' => self.right(num),
            'F' => self.go(self.direction, num),
            _ => panic!("Unknown op '{}'", op),
        }
    }

    fn manhattan_distance(self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

pub fn run(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            (
                line.chars().next().unwrap(),
                i32::from_str(&line[1..]).unwrap(),
            )
        })
        .fold(Position::new(), |acc, (op, num)| acc.step(op, num))
        .manhattan_distance()
}
