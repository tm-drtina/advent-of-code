use std::ops::RangeInclusive;
use std::str::FromStr;

use anyhow::{Error, Result, anyhow};

#[derive(Debug, Clone)]
pub(super) struct Pair(RangeInclusive<usize>, RangeInclusive<usize>);

impl FromStr for Pair {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s
            .split_once(',')
            .ok_or_else(|| anyhow!("Invalid line syntax"))?;

        let (l_start, l_end) = l
            .split_once('-')
            .ok_or_else(|| anyhow!("Invalid range syntax"))?;
        let l_start = l_start.parse()?;
        let l_end = l_end.parse()?;
        let l = l_start..=l_end;

        let (r_start, r_end) = r
            .split_once('-')
            .ok_or_else(|| anyhow!("Invalid range syntax"))?;
        let r_start = r_start.parse()?;
        let r_end = r_end.parse()?;
        let r = r_start..=r_end;

        Ok(Self(l, r))
    }
}

impl Pair {
    pub fn self_contained(&self) -> bool {
        self.0.contains(self.1.start()) && self.0.contains(self.1.end())
            || self.1.contains(self.0.start()) && self.1.contains(self.0.end())
    }

    pub fn overlaps(&self) -> bool {
        self.0.contains(self.1.start())
            || self.0.contains(self.1.end())
            || self.1.contains(self.0.start())
            || self.1.contains(self.0.end())
    }
}
