use anyhow::{Result, anyhow};

fn parse_line(mut line: &str) -> Result<(u32, u32)> {
    let mut first = None;
    let mut last = None;

    let mut set_val = |v| {
        if first.is_none() {
            first.replace(v);
        }
        last.replace(v);
    };

    while !line.is_empty() {
        if let Some(n) = line.chars().next().unwrap().to_digit(10) {
            set_val(n);
        } else if line.starts_with("one") {
            set_val(1);
        } else if line.starts_with("two") {
            set_val(2);
        } else if line.starts_with("three") {
            set_val(3);
        } else if line.starts_with("four") {
            set_val(4);
        } else if line.starts_with("five") {
            set_val(5);
        } else if line.starts_with("six") {
            set_val(6);
        } else if line.starts_with("seven") {
            set_val(7);
        } else if line.starts_with("eight") {
            set_val(8);
        } else if line.starts_with("nine") {
            set_val(9);
        }
        line = &line[1..];
    }
    Ok((
        first.ok_or(anyhow!("num not found"))?,
        last.ok_or(anyhow!("num not found"))?,
    ))
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
