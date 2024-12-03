use anyhow::Result;
use regex::Regex;

pub fn run(input: &str) -> Result<usize> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    Ok(re
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [a, b])| Ok(a.parse::<usize>()? * b.parse::<usize>()?))
        .collect::<Result<Vec<usize>>>()?
        .into_iter()
        .sum())
}
