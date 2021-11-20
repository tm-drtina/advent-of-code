use std::str::FromStr;

enum Direction {
    East,
    South,
    West,
    North,
}

struct Position {
    x: i64,
    y: i64,
    wx: i64,
    wy: i64,
}

impl Position {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            wx: 10,
            wy: 1,
        }
    }

    fn with_waypoint(&mut self, wx: i64, wy: i64) {
        self.wx = wx;
        self.wy = wy;
    }

    fn move_waypoint(&mut self, direction: Direction, len: i64) {
        match direction {
            Direction::East => self.with_waypoint(self.wx + len, self.wy),
            Direction::South => self.with_waypoint(self.wx, self.wy - len),
            Direction::West => self.with_waypoint(self.wx - len, self.wy),
            Direction::North => self.with_waypoint(self.wx, self.wy + len),
        }
    }

    fn go_to_waypoint(&mut self, len: i64) {
        self.x += self.wx * len;
        self.y += self.wy * len;
    }

    fn left(&mut self, num: i64) {
        if num > 0 {
            let wx = self.wx;
            let wy = self.wy;
            self.wx = -wy;
            self.wy = wx;
            self.left(num - 90)
        }
    }

    fn right(&mut self, num: i64) {
        if num > 0 {
            let wx = self.wx;
            let wy = self.wy;
            self.wx = wy;
            self.wy = -wx;
            self.right(num - 90)
        }
    }

    fn step(&mut self, op: char, num: i64) {
        match op {
            'N' => self.move_waypoint(Direction::North, num),
            'S' => self.move_waypoint(Direction::South, num),
            'E' => self.move_waypoint(Direction::East, num),
            'W' => self.move_waypoint(Direction::West, num),
            'L' => self.left(num),
            'R' => self.right(num),
            'F' => self.go_to_waypoint(num),
            _ => panic!("Unknown op '{}'", op),
        }
    }

    fn manhattan_distance(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

pub fn run(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            (
                line.chars().next().unwrap(),
                i64::from_str(&line[1..]).unwrap(),
            )
        })
        .fold(Position::new(), |mut acc, (op, num)| {
            acc.step(op, num);
            acc
        })
        .manhattan_distance()
}
