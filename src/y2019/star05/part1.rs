use std::collections::VecDeque;

const ADD: i32 = 1;
const MUL: i32 = 2;
const READ: i32 = 3;
const WRITE: i32 = 4;
const EXIT: i32 = 99;

pub(super) struct IntcodeProgram {
    pub tape: Vec<i32>,
    position: usize,
    input: VecDeque<i32>,
    output: Vec<i32>,
}

impl IntcodeProgram {
    pub fn new(tape: &str) -> Self {
        Self {
            tape: tape.split(',').map(|x| x.parse().unwrap()).collect(),
            position: 0,
            input: VecDeque::new(),
            output: Vec::new(),
        }
    }

    fn op(&mut self, n: usize) -> &mut i32 {
        let param_mode = self.tape[self.position] / (10_i32.pow(n as u32 + 1)) % 10;
        match param_mode {
            0 => {
                let i = self.tape[self.position + n] as usize;
                &mut self.tape[i]
            }
            1 => &mut self.tape[self.position + n],
            _ => panic!("Unrecognized param mode '{}'", param_mode),
        }
    }

    fn operation(&self) -> i32 {
        self.tape[self.position] % 100
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
                    *self.op(1) = self.input.pop_front().expect("No values in input stream.");
                    self.position += 2;
                }
                WRITE => {
                    let val = *self.op(1);
                    self.output.push(val);
                    self.position += 2;
                }
                EXIT => break,
                _ => return Err(format!("Unknown operation {}", self.tape[self.position])),
            }
        }
        Ok(())
    }
}

pub fn run(input: &str) -> i32 {
    let mut program = IntcodeProgram::new(input);
    program.input.push_back(1);
    program.run().unwrap();
    *program.output.last().unwrap()
}
