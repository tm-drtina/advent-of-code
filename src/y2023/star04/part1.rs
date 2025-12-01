use std::collections::HashSet;
use std::str::FromStr;

use anyhow::{Context, Result, anyhow};

pub(super) struct Card {
    #[allow(dead_code)]
    id: usize,
    winning: HashSet<u32>,
    mine: Vec<u32>,
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(mut s: &str) -> Result<Self> {
        s = s
            .strip_prefix("Card ")
            .ok_or(anyhow!("Missing 'Card ' prefix"))?;
        let (id, rest) = s
            .split_once(": ")
            .ok_or(anyhow!("Failed to split card id"))?;
        let id = id.trim_start().parse()?;

        let (winning, mine) = rest
            .split_once(" | ")
            .ok_or(anyhow!("Falied to split two sets of numbers"))?;

        let winning = winning
            .split_ascii_whitespace()
            .map(|n| n.parse::<u32>().context("Failed to parse winning number"))
            .collect::<Result<_>>()?;
        let mine = mine
            .split_ascii_whitespace()
            .map(|n| n.parse::<u32>().context("Failed to parse my number"))
            .collect::<Result<_>>()?;

        Ok(Self { id, winning, mine })
    }
}

impl Card {
    pub(super) fn matches(&self) -> usize {
        self.mine
            .iter()
            .filter(|n| self.winning.contains(n))
            .count()
    }

    fn score(&self) -> u32 {
        let matches = self.matches();
        if matches == 0 { 0 } else { 1 << (matches - 1) }
    }
}

pub fn run(input: &str) -> Result<u32> {
    Ok(input
        .lines()
        .map(Card::from_str)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(|g| g.score())
        .sum())
}
