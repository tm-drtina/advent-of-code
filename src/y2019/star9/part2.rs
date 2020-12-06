use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

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
    pub tape: HashMap<i64, i64>,
    position: i64,
    rel_base: i64,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
}

impl IntcodeProgram {
    pub fn new(tape_str: &str) -> Self {
        Self {
            tape: tape_str
                .split(",")
                .enumerate()
                .map(|(index, val)| (index as i64, i64::from_str(val).unwrap()))
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

    pub fn run(&mut self) -> Result<(), String> {
        loop {
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
                    let val = self.input.pop_front().expect("No values in input stream.");
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
                EXIT => break,
                _ => return Err(format!("Unknown operation {}", self.tape[&self.position])),
            }
        }
        Ok(())
    }
}

pub fn run(input: &str) -> i64 {
    let mut prog = IntcodeProgram::new(input);
    prog.input.push_back(2);
    prog.run().unwrap();
    *prog.output.back().unwrap()
}
