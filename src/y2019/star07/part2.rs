use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use itertools::Itertools;

const ADD: i32 = 1;
const MUL: i32 = 2;
const READ: i32 = 3;
const WRITE: i32 = 4;
const JUMP: i32 = 5;
const JUMP_NOT: i32 = 6;
const LT: i32 = 7;
const EQ: i32 = 8;
const EXIT: i32 = 99;

struct IntcodeProgram {
    pub tape: Vec<i32>,
    position: usize,
    input: Rc<RefCell<VecDeque<i32>>>,
    output: Rc<RefCell<VecDeque<i32>>>,
}

impl IntcodeProgram {
    fn new(
        tape: &str,
        input: Rc<RefCell<VecDeque<i32>>>,
        output: Rc<RefCell<VecDeque<i32>>>,
    ) -> Self {
        Self {
            tape: tape.split(',').map(|x| x.parse().unwrap()).collect(),
            position: 0,
            input,
            output,
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

    fn step(&mut self) -> Result<bool, String> {
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
                let val = self.input.borrow_mut().pop_front();
                if let Some(val) = val {
                    *self.op(1) = val;
                    self.position += 2;
                }
            }
            WRITE => {
                let val = *self.op(1);
                self.output.borrow_mut().push_back(val);
                self.position += 2;
            }
            JUMP => {
                if *self.op(1) != 0 {
                    self.position = *self.op(2) as usize;
                } else {
                    self.position += 3;
                }
            }
            JUMP_NOT => {
                if *self.op(1) == 0 {
                    self.position = *self.op(2) as usize;
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
            EXIT => return Ok(true),
            _ => return Err(format!("Unknown operation {}", self.tape[self.position])),
        }
        Ok(false)
    }
}

fn run_with_phase_setting(input: &str, phase_setting: &[i32]) -> Result<i32, String> {
    let o5i1: Rc<RefCell<VecDeque<i32>>> = Rc::new(RefCell::new(VecDeque::new()));
    let o1i2: Rc<RefCell<VecDeque<i32>>> = Rc::new(RefCell::new(VecDeque::new()));
    let o2i3: Rc<RefCell<VecDeque<i32>>> = Rc::new(RefCell::new(VecDeque::new()));
    let o3i4: Rc<RefCell<VecDeque<i32>>> = Rc::new(RefCell::new(VecDeque::new()));
    let o4i5: Rc<RefCell<VecDeque<i32>>> = Rc::new(RefCell::new(VecDeque::new()));
    let mut p1 = IntcodeProgram::new(input, o5i1.clone(), o1i2.clone());
    let mut p2 = IntcodeProgram::new(input, o1i2, o2i3.clone());
    let mut p3 = IntcodeProgram::new(input, o2i3, o3i4.clone());
    let mut p4 = IntcodeProgram::new(input, o3i4, o4i5.clone());
    let mut p5 = IntcodeProgram::new(input, o4i5, o5i1);

    p1.input.borrow_mut().push_back(phase_setting[0]);
    p2.input.borrow_mut().push_back(phase_setting[1]);
    p3.input.borrow_mut().push_back(phase_setting[2]);
    p4.input.borrow_mut().push_back(phase_setting[3]);
    p5.input.borrow_mut().push_back(phase_setting[4]);

    p1.input.borrow_mut().push_back(0);
    loop {
        let r1 = p1.step()?;
        let r2 = p2.step()?;
        let r3 = p3.step()?;
        let r4 = p4.step()?;
        let r5 = p5.step()?;
        if r1 && r2 && r3 && r4 && r5 {
            break;
        }
    }
    let res = *p5.output.borrow_mut().back().unwrap();
    Ok(res)
}

pub fn run(input: &str) -> i32 {
    (5..10)
        .permutations(5)
        .map(|perm| run_with_phase_setting(input, &perm).unwrap())
        .max()
        .unwrap()
}
