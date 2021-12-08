use itertools::Itertools;

struct Moves {
    minus0: i64,
    minus1: i64,
    minus2: i64,
}

impl Moves {
    fn new() -> Self {
        Self {
            minus0: 1,
            minus1: 0,
            minus2: 0,
        }
    }

    fn step(self, prev: i32, curr: i32) -> Self {
        match curr - prev {
            1 => Self {
                minus0: self.minus0 + self.minus1 + self.minus2,
                minus1: self.minus0,
                minus2: self.minus1,
            },
            2 => Self {
                minus0: self.minus0 + self.minus1,
                minus1: self.minus0,
                minus2: 0,
            },
            3 => Self {
                minus0: self.minus0,
                minus1: 0,
                minus2: 0,
            },
            _ => panic!("invalid diff"),
        }
    }
}

pub fn run(input: &str) -> i64 {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .sorted()
        .fold((0, Moves::new()), |(last_value, moves), value| {
            (value, moves.step(last_value, value))
        })
        .1
        .minus0
}
