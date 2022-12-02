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
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
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

impl FromStr for Outcome {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => bail!("Unexpected choice '{}'", s),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    opponent: Choice,
    outcome: Outcome,
}

impl FromStr for Round {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opponent, outcome) = s.split_once(' ').context("Invalid input format")?;
        Ok(Self {
            opponent: opponent.parse()?,
            outcome: outcome.parse()?,
        })
    }
}

impl Round {
    fn myself(self) -> Choice {
        match self {
            Self {
                opponent: Choice::Rock,
                outcome: Outcome::Draw,
            }
            | Self {
                opponent: Choice::Paper,
                outcome: Outcome::Lose,
            }
            | Self {
                opponent: Choice::Scissors,
                outcome: Outcome::Win,
            } => Choice::Rock,

            Self {
                opponent: Choice::Rock,
                outcome: Outcome::Win,
            }
            | Self {
                opponent: Choice::Paper,
                outcome: Outcome::Draw,
            }
            | Self {
                opponent: Choice::Scissors,
                outcome: Outcome::Lose,
            } => Choice::Paper,

            Self {
                opponent: Choice::Rock,
                outcome: Outcome::Lose,
            }
            | Self {
                opponent: Choice::Paper,
                outcome: Outcome::Win,
            }
            | Self {
                opponent: Choice::Scissors,
                outcome: Outcome::Draw,
            } => Choice::Scissors,
        }
    }

    fn score(self) -> usize {
        self.myself().value() + self.outcome.value()
    }
}

fn run_safe(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(str::parse::<Round>)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(Round::score)
        .sum())
}

pub fn run(input: &str) -> usize {
    run_safe(input).unwrap()
}
