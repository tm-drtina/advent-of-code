use anyhow::Result;

pub fn run(input: &str) -> Result<usize> {
    let mut best = 0;
    let mut current = 0;
    for line in input.lines() {
        if line.is_empty() {
            if current > best {
                best = current;
            }
            current = 0;
        } else {
            current += line.parse::<usize>()?;
        }
    }
    Ok(std::cmp::max(best, current))
}
