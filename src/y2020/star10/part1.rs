use itertools::Itertools;
use std::str::FromStr;

struct Diff {
    d1: i32,
    d2: i32,
    d3: i32,
}

impl Diff {
    fn new() -> Self {
        Self {
            d1: 0,
            d2: 0,
            d3: 1,
        }
    }

    fn add_diff(mut self, diff: i32) -> Self {
        match diff {
            1 => self.d1 += 1,
            2 => self.d2 += 1,
            3 => self.d3 += 1,
            _ => {}
        }
        self
    }

    fn result(self) -> i32 {
        self.d1 * self.d3
    }
}

pub fn run(input: &str) -> i32 {
    input
        .lines()
        .map(|line| i32::from_str(line).unwrap())
        .sorted()
        .fold((0, Diff::new()), |(last_value, diff), value| {
            (value, diff.add_diff(value - last_value))
        })
        .1
        .result()
}
