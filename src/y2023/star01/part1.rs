use anyhow::{Result, anyhow};

fn parse_line(line: &str) -> Result<(u32, u32)> {
    let mut iter = line.chars().filter_map(|c| c.to_digit(10));
    let first = iter.next().ok_or(anyhow!(""))?;
    let last = iter.next_back().unwrap_or(first);
    Ok((first, last))
}

pub fn run(input: &str) -> Result<u32> {
    Ok(input
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(|(a, b)| a * 10 + b)
        .sum())
}
