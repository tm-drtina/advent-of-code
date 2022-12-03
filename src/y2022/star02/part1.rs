use std::str::FromStr;

use anyhow::{bail, Context, Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn value(self) -> usize {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

impl FromStr for Choice {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => bail!("Unexpected choice '{}'", s),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn value(self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

struct Round {
    opponent: Choice,
    myself: Choice,
}

impl FromStr for Round {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opponent, myself) = s.split_once(' ').context("Invalid input format")?;
        Ok(Self {
            opponent: opponent.parse()?,
            myself: myself.parse()?,
        })
    }
}

impl Round {
    fn outcome(self) -> Outcome {
        match self {
            Self {
                opponent: Choice::Rock,
                myself: Choice::Rock,
            }
            | Self {
                opponent: Choice::Paper,
                myself: Choice::Paper,
            }
            | Self {
                opponent: Choice::Scissors,
                myself: Choice::Scissors,
            } => Outcome::Draw,

            Self {
                opponent: Choice::Rock,
                myself: Choice::Scissors,
            }
            | Self {
                opponent: Choice::Paper,
                myself: Choice::Rock,
            }
            | Self {
                opponent: Choice::Scissors,
                myself: Choice::Paper,
            } => Outcome::Lose,

            Self {
                opponent: Choice::Rock,
                myself: Choice::Paper,
            }
            | Self {
                opponent: Choice::Paper,
                myself: Choice::Scissors,
            }
            | Self {
                opponent: Choice::Scissors,
                myself: Choice::Rock,
            } => Outcome::Win,
        }
    }

    fn score(self) -> usize {
        self.myself.value() + self.outcome().value()
    }
}

pub fn run(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(str::parse::<Round>)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(Round::score)
        .sum::<usize>())
}
