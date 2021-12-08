use std::collections::HashSet;

#[derive(Clone)]
enum Op {
    Acc { val: i32 },
    Jmp { val: i32 },
    Nop { val: i32 },
}

impl From<&str> for Op {
    fn from(s: &str) -> Self {
        let (op, str_val) = s.split_once(" ").unwrap();
        let val = str_val.parse().unwrap();
        match op {
            "acc" => Op::Acc { val },
            "jmp" => Op::Jmp { val },
            "nop" => Op::Nop { val },
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
                Op::Acc { val } => {
                    self.acc += val;
                    self.ip += 1;
                }
                Op::Jmp { val } => {
                    self.ip += val;
                }
                Op::Nop { val: _ } => {
                    self.ip += 1;
                }
            }
        }
    }
}

pub fn run(input: &str) -> i32 {
    let orig_prog = Program::new(input.lines().map(Op::from).collect());
    (0..orig_prog.ops.len())
        .filter(|i| matches!(orig_prog.ops[*i], Op::Nop { val: _ } | Op::Jmp { val: _ }))
        .map(|i| {
            let mut clone = Program::new(orig_prog.ops.clone());
            match clone.ops[i] {
                Op::Jmp { val } => clone.ops[i] = Op::Nop { val },
                Op::Nop { val } => clone.ops[i] = Op::Jmp { val },
                _ => {}
            }
            clone
        })
        .find_map(|mut p| {
            p.run_until_cycle().ok()?;
            Some(p.acc)
        })
        .unwrap()

    //orig_prog.acc
}
