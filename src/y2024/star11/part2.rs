use std::num::ParseIntError;

use anyhow::Result;

use crate::utils::cache::unsync::Cache;

pub fn run(input: &str) -> Result<usize> {
    let cache: Cache<(usize, usize), usize> = Cache::new(Box::new(|(num, blinks), cache| {
        if *blinks == 0 {
            return 1;
        }

        if *num == 0 {
            cache.get_or_compute((1, blinks - 1))
        } else {
            let log10 = num.ilog10();
            if log10 & 1 == 0 {
                cache.get_or_compute((num * 2024, blinks - 1))
            } else {
                let log10_2 = log10.div_ceil(2);
                let pow10 = 10usize.pow(log10_2);
                cache.get_or_compute((num / pow10, blinks - 1))
                    + cache.get_or_compute((num % pow10, blinks - 1))
            }
        }
    }));

    Ok(input
        .split_ascii_whitespace()
        .map(str::parse::<usize>)
        .collect::<Result<Vec<_>, ParseIntError>>()?
        .into_iter()
        .map(|num| cache.get_or_compute((num, 75)))
        .sum())
}
