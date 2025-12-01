use std::str::FromStr;

use anyhow::{Error, Result, anyhow};

#[derive(Debug, Clone, Copy)]
pub(super) struct Move {
    pub(super) amount: usize,
    pub(super) from: usize,
    pub(super) to: usize,
}

pub(super) struct Puzzle {
    pub(super) stacks: Vec<Vec<char>>,
    pub(super) moves: Vec<Move>,
}

impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines().peekable();
        let mut acc: Vec<Vec<Option<char>>> = Vec::new();
        loop {
            let line = lines.next().ok_or_else(|| anyhow!("Invalid format"))?;
            if lines.peek().is_none_or(|s| s.is_empty()) {
                break;
            }
            let crates = line
                .bytes()
                .skip(1)
                .step_by(4)
                .map(|b| match b {
                    b' ' => None,
                    b => Some(b as char),
                })
                .collect();
            acc.push(crates);
        }
        let init = vec![vec![]; acc[0].len()];
        let stacks = acc.iter().rev().fold(init, |mut acc, next| {
            acc.iter_mut().zip(next).for_each(|(a, b)| {
                if let Some(b) = b {
                    a.push(*b);
                }
            });
            acc
        });
        lines.next(); // empty line
        let moves = lines
            .map(|line| {
                let line = line
                    .strip_prefix("move ")
                    .ok_or_else(|| anyhow!("Invalid format"))?;
                let (amount, line) = line
                    .split_once(" from ")
                    .ok_or_else(|| anyhow!("Invalid format"))?;
                let (from, to) = line
                    .split_once(" to ")
                    .ok_or_else(|| anyhow!("Invalid format"))?;
                Ok(Move {
                    amount: amount.parse()?,
                    from: from.parse::<usize>()? - 1,
                    to: to.parse::<usize>()? - 1,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { stacks, moves })
    }
}
