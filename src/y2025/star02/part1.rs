use std::str::FromStr;

use anyhow::{Result, anyhow};

fn log10_floor(n: u64) -> u32 {
    (n as f64).log10().floor() as u32
}

struct Range(u64, u64);

impl Range {
    fn invalid_ids(self) -> impl Iterator<Item = u64> {
        let start = {
            let d = log10_floor(self.0);
            if d % 2 == 0 {
                // Odd number of digits, return first number with even digits
                // 100 (d=2) -> 1000
                10u64.pow(d / 2)
            } else {
                let x = 10u64.pow(d / 2 + 1);
                let s = self.0 / x;
                if s * x + s < self.0 { s + 1 } else { s }
            }
        };
        let end = {
            let d = log10_floor(self.1);
            if d % 2 == 0 {
                // Odd number of digits, return previous number with even digits
                // 120 (d=2) -> 99
                10u64.pow(d / 2) - 1
            } else {
                let x = 10u64.pow(d / 2 + 1);
                let s = self.1 / x;
                if s * x + s > self.1 { s - 1 } else { s }
            }
        };
        (start..=end)
            .into_iter()
            .map(|n| n + n * 10u64.pow(log10_floor(n) + 1))
    }
}

impl FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (l, r) = s.split_once('-').ok_or(anyhow!("Missing delimiter"))?;
        Ok(Self(l.parse()?, r.parse()?))
    }
}

pub fn run(input: &str) -> Result<u64> {
    Ok(input
        .trim()
        .split_terminator(',')
        .map(Range::from_str)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flat_map(Range::invalid_ids)
        .sum::<u64>())
}
