use std::str::FromStr;
use std::convert::TryInto;

const ADD: i32 = 1;
const MUL: i32 = 2;
const EXIT: i32 = 99;

pub(super) struct IntcodeProgram {
    pub tape: Vec<i32>,
    position: usize,
}

impl IntcodeProgram {
    pub fn new(tape: &str) -> Self {
        Self {
            tape: tape.split(',').map(|x| i32::from_str(x).unwrap()).collect(),
            position: 0,
        }
    }

    fn op_as_index(&self, n: usize) -> usize { self.tape[self.position + n].try_into().unwrap() }

    pub fn run(&mut self) -> Result<(), String> {
        loop {
            match self.tape[self.position] {
                ADD => {
                    let op1 = self.op_as_index(1);
                    let op2 = self.op_as_index(2);
                    let op3 = self.op_as_index(3);
                    self.tape[op3] = self.tape[op1] + self.tape[op2];
                    self.position += 4;
                }
                MUL => {
                    let op1 = self.op_as_index(1);
                    let op2 = self.op_as_index(2);
                    let op3 = self.op_as_index(3);
                    self.tape[op3] = self.tape[op1] * self.tape[op2];
                    self.position += 4;
                }
                EXIT => break,
                _ => return Err(format!("Unknown operation {}", self.tape[self.position]))
            }
        }
        Ok(())
    }
}

pub fn run(input: &str) -> i32 {
    let mut program = IntcodeProgram::new(input);
    program.tape[1] = 12;
    program.tape[2] = 2;
    program.run().unwrap();
    program.tape[0]
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_intcode_program1() {
        let mut program = super::IntcodeProgram::new("1,0,0,0,99");
        program.run().unwrap();
        assert_eq!(vec![2,0,0,0,99], program.tape);
    }
    #[test]
    fn test_intcode_program2() {
        let mut program = super::IntcodeProgram::new("2,3,0,3,99");
        program.run().unwrap();
        assert_eq!(vec![2,3,0,6,99], program.tape);
    }
    #[test]
    fn test_intcode_program3() {
        let mut program = super::IntcodeProgram::new("2,4,4,5,99,0");
        program.run().unwrap();
        assert_eq!(vec![2,4,4,5,99,9801], program.tape);
    }
    #[test]
    fn test_intcode_program4() {
        let mut program = super::IntcodeProgram::new("1,1,1,4,99,5,6,0,99");
        program.run().unwrap();
        assert_eq!(vec![30,1,1,4,2,5,6,0,99], program.tape);
    }
}