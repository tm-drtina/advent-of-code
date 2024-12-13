use std::num::ParseIntError;

use anyhow::Result;

fn num_stones(num: usize, blinks: usize) -> usize {
    if blinks == 0 {
        return 1;
    }

    if num == 0 {
        num_stones(1, blinks - 1)
    } else {
        let log10 = num.ilog10();
        if log10 & 1 == 0 {
            num_stones(num * 2024, blinks - 1)
        } else {
            let log10_2 = (log10 + 1) / 2;
            let pow10 = 10usize.pow(log10_2);
            num_stones(num / pow10, blinks - 1) + num_stones(num % pow10, blinks - 1)
        }
    }
}

pub fn run(input: &str) -> Result<usize> {
    Ok(input
        .split_ascii_whitespace()
        .map(str::parse::<usize>)
        .collect::<Result<Vec<_>, ParseIntError>>()?
        .into_iter()
        .map(|num| num_stones(num, 25))
        .sum())
}
