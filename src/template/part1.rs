use anyhow::Result;

pub fn run(input: &str) -> Result<usize> {
    Ok(input.parse::<usize>()? * 2)
}
