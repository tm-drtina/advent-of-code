use std::collections::HashMap;
use std::str::FromStr;

enum Op {
    Mask { value: String },
    Mem { addr: u64, value: u64 },
}

impl From<&str> for Op {
    fn from(line: &str) -> Self {
        let (op, val) = line.split_once(" = ").unwrap();
        if op == "mask" {
            Op::Mask {
                value: String::from(val),
            }
        } else {
            Op::Mem {
                addr: u64::from_str(&op[4..op.len() - 1]).unwrap(),
                value: u64::from_str(val).unwrap(),
            }
        }
    }
}

struct Program {
    mask: String,
    memory: HashMap<u64, u64>,
}

impl Program {
    fn new() -> Self {
        Self {
            mask: String::from("000000000000000000000000000000000000"),
            memory: HashMap::new(),
        }
    }

    fn mask_addr(&self, addr: u64) -> Vec<u64> {
        let mut addrs: Vec<u64> = vec![0];
        for (index, mask_bit) in self.mask.chars().enumerate() {
            match mask_bit {
                '0' => {
                    let addr_bit = (addr >> (self.mask.len() - index - 1)) % 2;
                    addrs = addrs.iter().map(|addr| (addr << 1) + addr_bit).collect()
                }
                '1' => addrs = addrs.iter().map(|addr| (addr << 1) + 1).collect(),
                'X' => {
                    addrs = addrs
                        .iter()
                        .flat_map(|addr| vec![(addr << 1), (addr << 1) + 1])
                        .collect()
                }
                _ => panic!("Unknown mask char {}", mask_bit),
            }
        }
        addrs
    }

    fn step(mut self, op: Op) -> Self {
        match op {
            Op::Mask { value } => {
                self.mask = value;
            }
            Op::Mem { addr, value } => {
                for real_addr in self.mask_addr(addr) {
                    self.memory.insert(real_addr, value);
                }
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
        .map(|line| Op::from(line))
        .fold(Program::new(), Program::step)
        .memory_sum()
}
