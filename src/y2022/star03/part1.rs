use std::str::FromStr;

use anyhow::{bail, Error, Result};

struct Rucksack {
    compartments: Vec<Vec<u8>>,
}

impl FromStr for Rucksack {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        let (cp1, cp2) = bytes.split_at(bytes.len() / 2);
        let compartments = [cp1, cp2]
            .into_iter()
            .map(|cp| {
                cp.iter()
                    .map(|b| {
                        Ok(match b {
                            b'a'..=b'z' => b - b'a' + 1,
                            b'A'..=b'Z' => b - b'A' + 27,
                            _ => bail!("Unexpected char"),
                        })
                    })
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { compartments })
    }
}

impl Rucksack {
    fn priority(&self) -> usize {
        for item in &self.compartments[0] {
            if self.compartments[1].iter().any(|i| item == i) {
                return *item as usize;
            }
        }
        unreachable!("Every line should have duplicate")
    }
}

fn run_safe(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(str::parse::<Rucksack>)
        .collect::<Result<Vec<_>>>()?
        .iter()
        .map(Rucksack::priority)
        .sum())
}

pub fn run(input: &str) -> usize {
    run_safe(input).unwrap()
}
