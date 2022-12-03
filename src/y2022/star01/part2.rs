use anyhow::Result;

pub fn run(input: &str) -> Result<usize> {
    let mut values = Vec::new();
    let mut current = 0;
    for line in input.lines() {
        if line.is_empty() {
            values.push(current);
            current = 0;
        } else {
            current += line.parse::<usize>()?;
        }
    }
    values.push(current);
    values.sort_unstable();
    Ok(values.iter().rev().take(3).sum())
}
