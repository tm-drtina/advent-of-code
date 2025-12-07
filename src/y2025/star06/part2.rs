use std::str::Bytes;

enum NumItem {
    Num(u64),
    Empty,
}

struct NumGen<'a> {
    input: Vec<Bytes<'a>>,
    buf: Vec<u8>,
}
impl<'a> NumGen<'a> {
    fn new(input: Vec<&'a str>) -> Self {
        let input = input
            .into_iter()
            .map(|line| line.bytes())
            .collect::<Vec<_>>();
        let buf = Vec::with_capacity(input.len());
        Self { input, buf }
    }
}

impl Iterator for NumGen<'_> {
    type Item = NumItem;

    fn next(&mut self) -> Option<Self::Item> {
        self.buf.clear();
        for line in &mut self.input {
            if let ch @ b'0'..=b'9' = line.next()? {
                self.buf.push(ch - b'0');
            }
        }
        Some(if self.buf.is_empty() {
            NumItem::Empty
        } else {
            let mut n = 0;
            for i in &self.buf {
                n = n * 10 + (*i as u64);
            }
            NumItem::Num(n)
        })
    }
}

pub enum Op {
    Add,
    Mul,
}

impl Op {
    fn apply(&self, lhs: &mut u64, rhs: u64) {
        match self {
            Op::Add => *lhs += rhs,
            Op::Mul => *lhs *= rhs,
        }
    }
    fn init(&self) -> u64 {
        match self {
            Op::Add => 0,
            Op::Mul => 1,
        }
    }
}

pub fn run(input: &str) -> u64 {
    let mut lines = input.lines().collect::<Vec<_>>();
    let mut ops = lines
        .pop()
        .unwrap()
        .split_ascii_whitespace()
        .map(|ch| match ch {
            "*" => Op::Mul,
            "+" => Op::Add,
            _ => panic!(),
        });

    let mut res = 0;
    let mut op = ops.next().unwrap();
    let mut buf = op.init();
    for item in NumGen::new(lines) {
        match item {
            NumItem::Num(n) => {
                op.apply(&mut buf, n);
            }
            NumItem::Empty => {
                res += buf;
                op = ops.next().unwrap();
                buf = op.init();
            }
        }
    }
    res + buf
}
