use std::num::ParseIntError;

use anyhow::Result;

pub fn run(input: &str) -> Result<i64> {
    Ok(input
        .lines()
        .map(|line| {
            let mut nums = line
                .split_ascii_whitespace()
                .map(str::parse::<i64>)
                .collect::<Result<Vec<_>, ParseIntError>>()
                .unwrap();
            let mut view = &mut nums[..];
            while view.len() > 1 {
                for i in (0..view.len() - 1).rev() {
                    view[i + 1] -= view[i];
                }
                view = &mut view[1..];
            }
            nums.into_iter()
                .enumerate()
                .map(|(i, v)| if i % 2 == 0 { v } else { -v })
                .sum::<i64>()
        })
        .sum())
}
