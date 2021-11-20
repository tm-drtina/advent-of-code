use std::str::FromStr;

#[derive(Copy, Clone)]
enum Direction {
    East,
    South,
    West,
    North,
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
            direction: Direction::East,
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
            Direction::East => self.with_position(self.x + len, self.y),
            Direction::South => self.with_position(self.x, self.y - len),
            Direction::West => self.with_position(self.x - len, self.y),
            Direction::North => self.with_position(self.x, self.y + len),
        }
    }

    fn left(self, num: i32) -> Self {
        if num == 0 {
            self
        } else {
            match self.direction {
                Direction::East => self.with_direction(Direction::North).left(num - 90),
                Direction::South => self.with_direction(Direction::East).left(num - 90),
                Direction::West => self.with_direction(Direction::South).left(num - 90),
                Direction::North => self.with_direction(Direction::West).left(num - 90),
            }
        }
    }

    fn right(self, num: i32) -> Self {
        if num == 0 {
            self
        } else {
            match self.direction {
                Direction::East => self.with_direction(Direction::South).right(num - 90),
                Direction::South => self.with_direction(Direction::West).right(num - 90),
                Direction::West => self.with_direction(Direction::North).right(num - 90),
                Direction::North => self.with_direction(Direction::East).right(num - 90),
            }
        }
    }

    fn step(self, op: char, num: i32) -> Self {
        match op {
            'N' => self.go(Direction::North, num),
            'S' => self.go(Direction::South, num),
            'E' => self.go(Direction::East, num),
            'W' => self.go(Direction::West, num),
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
