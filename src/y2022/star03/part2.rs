use std::str::Lines;

use anyhow::{bail, Error, Result};
use itertools::{Chunk, Itertools};

struct Group {
    elves: Vec<Vec<u8>>,
}

impl<'a> TryFrom<Chunk<'a, Lines<'a>>> for Group {
    type Error = Error;

    fn try_from(lines: Chunk<'a, Lines<'a>>) -> Result<Self, Self::Error> {
        let elves = lines
            .map(|l| {
                l.bytes()
                    .map(|b| {
                        Ok(match b {
                            b'a'..=b'z' => b - b'a' + 1,
                            b'A'..=b'Z' => b - b'A' + 27,
                            _ => bail!("Unexpected char"),
                        })
                    })
                    .collect()
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { elves })
    }
}

impl Group {
    fn priority(&self) -> usize {
        for item in &self.elves[0] {
            if self.elves[1].iter().any(|i| item == i) && self.elves[2].iter().any(|i| item == i) {
                return *item as usize;
            }
        }
        unreachable!("Every group should have duplicate")
    }
}

pub fn run(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .chunks(3)
        .into_iter()
        .map(TryFrom::try_from)
        .collect::<Result<Vec<_>>>()?
        .iter()
        .map(Group::priority)
        .sum())
}
