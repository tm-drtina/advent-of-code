use std::str::FromStr;

use anyhow::Result;

use super::part1::Card;

pub fn run(input: &str) -> Result<usize> {
    let cards = input
        .lines()
        .map(Card::from_str)
        .collect::<Result<Vec<_>>>()?;

    let mut final_counts = vec![1; cards.len()];

    let mut counts = &mut final_counts[..];
    for card in cards {
        for i in 0..card.matches() {
            counts[i + 1] += counts[0];
        }
        counts = &mut counts[1..];
    }

    Ok(final_counts.into_iter().sum())
}
