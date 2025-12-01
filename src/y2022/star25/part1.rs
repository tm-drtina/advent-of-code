use std::iter::Sum;
use std::ops::Add;
use std::str::FromStr;

use anyhow::{Error, Result, bail};

type Value = i64;

#[derive(Debug, Clone, Copy)]
struct Snafu(Value);

impl From<i64> for Snafu {
    fn from(value: Value) -> Self {
        Self(value)
    }
}

impl Add for Snafu {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sum for Snafu {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|a, b| a + b).unwrap_or(Snafu(0))
    }
}

impl FromStr for Snafu {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut val = 0;
        let mut mult = 1;
        for b in s.bytes().rev() {
            let curr = match b {
                b'2' | b'1' | b'0' => (b - b'0') as Value,
                b'-' => -1,
                b'=' => -2,
                _ => bail!("Invalid format"),
            };
            val += curr * mult;
            mult *= 5;
        }
        Ok(Self(val))
    }
}

impl Snafu {
    fn process(self) -> String {
        let mut bytes = Vec::<u8>::new();
        let mut v = self.0;
        while v != 0 {
            bytes.push(match (v + 2) % 5 {
                0 => b'=',
                1 => b'-',
                2 => b'0',
                3 => b'1',
                4 => b'2',
                _ => unreachable!(),
            });
            v = (v + 2) / 5;
        }
        bytes.reverse();
        String::from_utf8(bytes).unwrap()
    }
}

pub fn run(input: &str) -> Result<String> {
    Ok(input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Snafu>>>()?
        .into_iter()
        .sum::<Snafu>()
        .process())
}
