use std::collections::BTreeSet;
use std::str::FromStr;

use anyhow::{Result, anyhow};

struct Range(u64, u64);
impl Range {
    fn reconstruct_num(n: u64, repeat: u32) -> u64 {
        let mut res = n;
        for _ in 1..repeat {
            res = res * 10u64.pow(n.ilog10() + 1) + n;
        }
        res
    }

    fn invalid_id_by_n(&self, n: u32) -> impl Iterator<Item = u64> {
        let start = {
            let d = self.0.ilog10() + 1;
            if d.is_multiple_of(n) {
                let shift = 10u64.pow(d - d / n);
                let s = self.0 / shift;
                if Self::reconstruct_num(s, n) < self.0 {
                    s + 1
                } else {
                    s
                }
            } else {
                10u64.pow(d / n)
            }
        };
        let end = {
            let d = self.1.ilog10() + 1;
            if d.is_multiple_of(n) {
                let shift = 10u64.pow(d - d / n);
                let s = self.1 / shift;
                if Self::reconstruct_num(s, n) > self.1 {
                    s - 1
                } else {
                    s
                }
            } else {
                10u64.pow(d / n) - 1
            }
        };
        (start..=end).map(move |x| Self::reconstruct_num(x, n))
    }

    fn invalid_ids(self) -> impl Iterator<Item = u64> {
        let d = self.1.ilog10() + 1;
        (2..=d)
            .flat_map(|n| self.invalid_id_by_n(n))
            .collect::<BTreeSet<_>>()
            .into_iter()
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
