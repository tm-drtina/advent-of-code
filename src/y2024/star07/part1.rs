use std::num::ParseIntError;
use std::str::FromStr;

use anyhow::{Context, Result, anyhow};

#[derive(Debug)]
pub struct Equation {
    pub result: usize,
    pub operands: Vec<usize>,
}

impl FromStr for Equation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (result, operands) = s
            .split_once(": ")
            .ok_or(anyhow!("Invalid input format, expected ': ' on the line"))?;

        let result = result.parse().context("Invalid number for result")?;
        let operands = operands
            .split_ascii_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<_>, ParseIntError>>()
            .context("Invalid number for operand")?;

        Ok(Self { result, operands })
    }
}

impl Equation {
    fn is_possible(&self) -> bool {
        let mut operands = self.operands.iter();
        let mut partial_results =
            vec![*operands.next().expect("At least one operand must be given")];
        for operand in operands {
            let mut new_partial_results = Vec::new();
            for partial_result in partial_results {
                for new_result in [partial_result * operand, partial_result + operand] {
                    match new_result.cmp(&self.result) {
                        std::cmp::Ordering::Less => new_partial_results.push(new_result),
                        std::cmp::Ordering::Equal => return true,
                        std::cmp::Ordering::Greater => {},
                    }
                }
            }
            if new_partial_results.is_empty() {
                return false;
            }
            partial_results = new_partial_results;
        }
        false
    }
}

pub fn run(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(str::parse::<Equation>)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .filter(Equation::is_possible)
        .map(|eq| eq.result)
        .sum())
}
