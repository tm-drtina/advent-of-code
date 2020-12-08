use std::collections::HashSet;
use std::str::FromStr;

enum Op {
    ACC { val: i32 },
    JMP { val: i32 },
    NOP,
}

impl From<&str> for Op {
    fn from(s: &str) -> Self {
        let (op, str_val) = s.split_once(" ").unwrap();
        let val = i32::from_str(str_val).unwrap();
        match op {
            "acc" => Op::ACC { val },
            "jmp" => Op::JMP { val },
            "nop" => Op::NOP,
            _ => panic!("Unknown op {}", op),
        }
    }
}

struct Program {
    ops: Vec<Op>,
    ip: i32,
    acc: i32,
}

impl Program {
    fn new(ops: Vec<Op>) -> Self {
        Self { ops, ip: 0, acc: 0 }
    }

    fn run_until_cycle(&mut self) {
        let mut visited: HashSet<i32> = HashSet::new();
        loop {
            if visited.contains(&self.ip) {
                break;
            }
            visited.insert(self.ip);
            match self.ops[self.ip as usize] {
                Op::ACC { val } => {
                    self.acc += val;
                    self.ip += 1;
                }
                Op::JMP { val } => {
                    self.ip += val;
                }
                Op::NOP => {
                    self.ip += 1;
                }
            }
        }
    }
}

pub fn run(input: &str) -> i32 {
    let mut prog = Program::new(input.lines().map(|line| Op::from(line)).collect());
    prog.run_until_cycle();
    prog.acc
}
