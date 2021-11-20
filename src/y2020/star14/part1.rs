use std::collections::HashMap;
use std::str::FromStr;

enum Op {
    Mask { and_mask: u64, or_mask: u64 },
    Mem { addr: i32, value: u64 },
}

impl From<&str> for Op {
    fn from(line: &str) -> Self {
        let (op, val) = line.split_once(" = ").unwrap();
        if op == "mask" {
            let mut and_mask = u64::MAX;
            let mut or_mask = u64::MIN;
            for ch in val.chars() {
                match ch {
                    '0' => {
                        and_mask <<= 1;
                        or_mask <<= 1;
                    }
                    '1' => {
                        and_mask <<= 1;
                        and_mask |= 1;
                        or_mask <<= 1;
                        or_mask |= 1;
                    }
                    _ => {
                        and_mask <<= 1;
                        and_mask |= 1;
                        or_mask <<= 1;
                    }
                }
            }
            Op::Mask { and_mask, or_mask }
        } else {
            Op::Mem {
                addr: i32::from_str(&op[4..op.len() - 1]).unwrap(),
                value: u64::from_str(val).unwrap(),
            }
        }
    }
}

struct Program {
    and_mask: u64,
    or_mask: u64,
    memory: HashMap<i32, u64>,
}

impl Program {
    fn new() -> Self {
        Self {
            and_mask: u64::MAX,
            or_mask: u64::MIN,
            memory: HashMap::new(),
        }
    }

    fn step(mut self, op: Op) -> Self {
        match op {
            Op::Mask { and_mask, or_mask } => {
                self.and_mask = and_mask;
                self.or_mask = or_mask;
            }
            Op::Mem { addr, value } => {
                self.memory
                    .insert(addr, (value | self.or_mask) & self.and_mask);
            }
        }
        self
    }

    fn memory_sum(&self) -> u64 {
        self.memory.values().sum()
    }
}

pub fn run(input: &str) -> u64 {
    input
        .lines()
        .map(Op::from)
        .fold(Program::new(), Program::step)
        .memory_sum()
}
