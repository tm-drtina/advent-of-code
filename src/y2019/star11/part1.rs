use std::collections::{HashMap, HashSet, VecDeque};

const ADD: i64 = 1;
const MUL: i64 = 2;
const READ: i64 = 3;
const WRITE: i64 = 4;
const JUMP: i64 = 5;
const JUMP_NOT: i64 = 6;
const LT: i64 = 7;
const EQ: i64 = 8;
const RBASE: i64 = 9;
const EXIT: i64 = 99;

pub(super) struct IntcodeProgram {
    tape: HashMap<i64, i64>,
    position: i64,
    rel_base: i64,
    pub input: VecDeque<i64>,
    pub output: VecDeque<i64>,
}

impl IntcodeProgram {
    pub fn new(tape_str: &str) -> Self {
        Self {
            tape: tape_str
                .split(',')
                .enumerate()
                .map(|(index, val)| (index as i64, val.parse().unwrap()))
                .collect(),
            position: 0,
            rel_base: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }

    fn op(&mut self, n: i64) -> &mut i64 {
        let param_mode = self.tape[&self.position] / (10_i64.pow(n as u32 + 1)) % 10;
        match param_mode {
            0 => {
                let i = self.tape[&(self.position + n)];
                self.tape.entry(i).or_insert(0)
            }
            1 => self.tape.entry(self.position + n).or_insert(0),
            2 => {
                let i = self.tape[&(self.position + n)] + self.rel_base;
                self.tape.entry(i).or_insert(0)
            }
            _ => panic!("Unrecognized param mode '{}'", param_mode),
        }
    }

    fn operation(&self) -> i64 {
        self.tape[&self.position] % 100
    }

    pub fn run_until_output(&mut self) -> Result<bool, String> {
        loop {
            let stop = self.step()?;
            if stop {
                return Ok(true);
            }
            if !self.output.is_empty() {
                return Ok(false);
            }
        }
    }

    pub fn step(&mut self) -> Result<bool, String> {
        match self.operation() {
            ADD => {
                *self.op(3) = *self.op(1) + *self.op(2);
                self.position += 4;
            }
            MUL => {
                *self.op(3) = *self.op(1) * *self.op(2);
                self.position += 4;
            }
            READ => {
                let val = self
                    .input
                    .pop_front()
                    .ok_or_else(|| "No values in input stream.".to_owned())?;
                *self.op(1) = val;
                self.position += 2;
            }
            WRITE => {
                let val = *self.op(1);
                self.output.push_back(val);
                self.position += 2;
            }
            JUMP => {
                if *self.op(1) != 0 {
                    self.position = *self.op(2);
                } else {
                    self.position += 3;
                }
            }
            JUMP_NOT => {
                if *self.op(1) == 0 {
                    self.position = *self.op(2);
                } else {
                    self.position += 3;
                }
            }
            LT => {
                *self.op(3) = if *self.op(1) < *self.op(2) { 1 } else { 0 };
                self.position += 4;
            }
            EQ => {
                *self.op(3) = if *self.op(1) == *self.op(2) { 1 } else { 0 };
                self.position += 4;
            }
            RBASE => {
                self.rel_base += *self.op(1);
                self.position += 2;
            }
            EXIT => return Ok(true),
            _ => return Err(format!("Unknown operation {}", self.tape[&self.position])),
        }
        Ok(false)
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub struct Robot {
    direction: Direction,
    pub position: (i32, i32),
}

impl Default for Robot {
    fn default() -> Self {
        Self::new()
    }
}

impl Robot {
    pub fn new() -> Self {
        Self {
            direction: Direction::Up,
            position: (0, 0),
        }
    }

    pub fn left(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    pub fn right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn step(&mut self) {
        self.position = match self.direction {
            Direction::Up => (self.position.0, self.position.1 + 1),
            Direction::Right => (self.position.0 + 1, self.position.1),
            Direction::Down => (self.position.0, self.position.1 - 1),
            Direction::Left => (self.position.0 - 1, self.position.1),
        }
    }
}

pub fn run(input: &str) -> usize {
    let mut prog = IntcodeProgram::new(input);
    let mut robot = Robot::new();
    let mut painted: HashSet<(i32, i32)> = HashSet::new();
    let mut white: HashSet<(i32, i32)> = HashSet::new();
    prog.input.push_back(0);
    loop {
        if prog.run_until_output().unwrap() {
            break;
        }
        painted.insert(robot.position);
        match prog.output.pop_front().unwrap() {
            0 => {
                white.remove(&robot.position);
            }
            1 => {
                white.insert(robot.position);
            }
            _ => panic!("Invalid color"),
        }
        if prog.run_until_output().unwrap() {
            break;
        }
        match prog.output.pop_front().unwrap() {
            0 => robot.left(),
            1 => robot.right(),
            _ => panic!("Invalid rotation"),
        }
        robot.step();
        prog.input.push_back(if white.contains(&robot.position) {
            1
        } else {
            0
        })
    }
    painted.len()
}
