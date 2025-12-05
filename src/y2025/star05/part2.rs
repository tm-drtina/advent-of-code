use anyhow::{Result, anyhow};

type Range = std::ops::RangeInclusive<u64>;

fn can_merge(a: Range, b: Range) -> bool {
    a.end() + 1 == *b.start()
        || b.end() + 1 == *a.start()
        || a.contains(b.start())
        || a.contains(b.end())
        || b.contains(a.start())
        || b.contains(a.end())
}

pub fn run(input: &str) -> Result<usize> {
    let mut lines = input.lines();
    let mut ranges: Vec<Range> = Vec::new();

    while let Some(line) = lines.next()
        && !line.is_empty()
    {
        let (start, end) = line.split_once('-').ok_or(anyhow!("Wrong format"))?;
        let start = start.parse::<u64>()?;
        let end = end.parse::<u64>()?;
        let mut new = start..=end;

        let (to_merge, new_ranges): (Vec<_>, Vec<_>) = ranges
            .into_iter()
            .partition(|r| can_merge(new.clone(), r.clone()));
        ranges = new_ranges;

        for r in to_merge {
            new = (*new.start().min(r.start()))..=(*new.end().max(r.end()));
        }
        ranges.push(new);
    }

    Ok(ranges.into_iter().map(Range::count).sum())
}
