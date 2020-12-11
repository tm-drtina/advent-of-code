use std::collections::HashSet;
use std::str::FromStr;

#[derive(Clone)]
enum Op {
    ACC { val: i32 },
    JMP { val: i32 },
    NOP { val: i32 },
}

impl From<&str> for Op {
    fn from(s: &str) -> Self {
        let (op, str_val) = s.split_once(" ").unwrap();
        let val = i32::from_str(str_val).unwrap();
        match op {
            "acc" => Op::ACC { val },
            "jmp" => Op::JMP { val },
            "nop" => Op::NOP { val },
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

    fn run_until_cycle(&mut self) -> Result<(), ()> {
        let mut visited: HashSet<i32> = HashSet::new();
        loop {
            if visited.contains(&self.ip) {
                return Err(());
            }
            if self.ip >= self.ops.len() as i32 {
                return Ok(());
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
                Op::NOP { val: _ } => {
                    self.ip += 1;
                }
            }
        }
    }
}

pub fn run(input: &str) -> i32 {
    let orig_prog = Program::new(input.lines().map(|line| Op::from(line)).collect());
    (0..orig_prog.ops.len())
        .filter(|i| matches!(orig_prog.ops[*i], Op::NOP { val: _ } | Op::JMP { val: _ }))
        .map(|i| {
            let mut clone = Program::new(orig_prog.ops.clone());
            match clone.ops[i] {
                Op::JMP { val } => clone.ops[i] = Op::NOP { val },
                Op::NOP { val } => clone.ops[i] = Op::JMP { val },
                _ => {}
            }
            clone
        })
        .filter_map(|mut p| {
            p.run_until_cycle().ok()?;
            Some(p.acc)
        })
        .next()
        .unwrap()

    //orig_prog.acc
}
