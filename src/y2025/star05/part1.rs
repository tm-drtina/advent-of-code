use anyhow::{Result, anyhow};

pub fn run(input: &str) -> Result<usize> {
    let mut lines = input.lines();
    let mut ranges = Vec::new();

    while let Some(line) = lines.next() && !line.is_empty() {
        let (start, end) = line.split_once('-').ok_or(anyhow!("Wrong format"))?;
        let start = start.parse::<u64>()?;
        let end = end.parse::<u64>()?;
        ranges.push(start..=end);
    }

    let mut count = 0;
    for line in lines {
        let ingredient = line.parse::<u64>()?;
        if ranges.iter().any(|range| range.contains(&ingredient)) {
            count += 1;
        }
    }

    Ok(count)
}
