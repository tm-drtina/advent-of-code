use std::collections::HashSet;

enum Op {
    Acc { val: i32 },
    Jmp { val: i32 },
    Nop,
}

impl From<&str> for Op {
    fn from(s: &str) -> Self {
        let (op, str_val) = s.split_once(" ").unwrap();
        let val = str_val.parse().unwrap();
        match op {
            "acc" => Op::Acc { val },
            "jmp" => Op::Jmp { val },
            "nop" => Op::Nop,
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
                Op::Acc { val } => {
                    self.acc += val;
                    self.ip += 1;
                }
                Op::Jmp { val } => {
                    self.ip += val;
                }
                Op::Nop => {
                    self.ip += 1;
                }
            }
        }
    }
}

pub fn run(input: &str) -> i32 {
    let mut prog = Program::new(input.lines().map(Op::from).collect());
    prog.run_until_cycle();
    prog.acc
}
