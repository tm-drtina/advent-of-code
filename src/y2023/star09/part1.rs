use std::num::ParseIntError;

use anyhow::Result;

pub fn run(input: &str) -> Result<i32> {
    Ok(input
        .lines()
        .map(|line| {
            let mut nums = line
                .split_ascii_whitespace()
                .map(str::parse::<i32>)
                .collect::<Result<Vec<_>, ParseIntError>>()
                .unwrap();
            for l in (1..nums.len()).rev() {
                for i in 0..l {
                    nums[i] = nums[i + 1] - nums[i];
                }
            }
            nums.into_iter().sum::<i32>()
        })
        .sum())
}
