use std::ops::Range;
use std::str::FromStr;

use anyhow::{Context, Result, anyhow};

struct Mapping {
    data: Vec<(Range<u64>, Range<u64>)>,
}

impl Mapping {
    fn map(&self, value: u64) -> u64 {
        for (source, dest) in &self.data {
            if source.contains(&value) {
                return dest.start + (value - source.start);
            }
        }
        value
    }

    fn map_range(&self, mut value: Range<u64>) -> Vec<Range<u64>> {
        let mut results = Vec::new();
        let mut ranges = &self.data[..];
        loop {
            while !ranges.is_empty() && ranges[0].0.end <= value.start {
                ranges = &ranges[1..];
            }
            if ranges.is_empty() || ranges[0].0.start >= value.end {
                results.push(value);
                return results;
            }
            if ranges[0].0.start > value.start {
                results.push(value.start..ranges[0].0.start);
                value = ranges[0].0.start..value.end;
            }
            if value.is_empty() {
                return results;
            }
            let offset = value.start - ranges[0].0.start;
            if ranges[0].0.end >= value.end {
                results.push(
                    ranges[0].1.start + offset
                        ..(ranges[0].1.start + offset + value.end - value.start),
                );
                return results;
            }
            let overlap = ranges[0].0.end - value.start;
            results.push(ranges[0].1.start + offset..ranges[0].1.start + offset + overlap);
            value = value.start + overlap..value.end;
        }
    }
}

pub(super) struct Puzzle {
    pub(super) initial_seeds: Vec<u64>,
    mappings: Vec<Mapping>,
}

impl FromStr for Puzzle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();
        let initials_seeds = lines.next().ok_or(anyhow!("Missing initial seeds"))?;
        lines.next();

        let initial_seeds = initials_seeds
            .strip_prefix("seeds: ")
            .ok_or(anyhow!("missing 'seeds: ' prefix"))?
            .split_ascii_whitespace()
            .map(|s| s.parse::<u64>().context("Failed to parse seed"))
            .collect::<Result<_>>()?;

        let mut puzzle = Puzzle {
            initial_seeds,
            mappings: Vec::new(),
        };

        while let Some(_) = lines.next() {
            let mut mapping = Mapping { data: Vec::new() };
            for m in lines.by_ref() {
                if m.is_empty() {
                    break;
                }
                let mut parts = m.split_ascii_whitespace();
                let dest_start = parts
                    .next()
                    .ok_or(anyhow!("Missing destination start"))?
                    .parse::<u64>()?;
                let source_start = parts
                    .next()
                    .ok_or(anyhow!("Missing source start"))?
                    .parse::<u64>()?;
                let length = parts
                    .next()
                    .ok_or(anyhow!("Missing length"))?
                    .parse::<u64>()?;
                mapping.data.push((
                    source_start..source_start + length,
                    dest_start..dest_start + length,
                ));
            }
            mapping.data.sort_unstable_by_key(|(k, _v)| k.start);
            puzzle.mappings.push(mapping);
        }

        Ok(puzzle)
    }
}

impl Puzzle {
    pub(super) fn map(&self, s: u64) -> u64 {
        self.mappings
            .iter()
            .fold(s, |prev, mapping| mapping.map(prev))
    }

    pub(super) fn min_by_map(&self, range: Range<u64>) -> u64 {
        self.mappings
            .iter()
            .fold(vec![range], |prev, mapping| {
                prev.into_iter()
                    .flat_map(|r| mapping.map_range(r))
                    .collect()
            })
            .into_iter()
            .map(|r| r.start)
            .min()
            .unwrap()
    }
}

pub fn run(input: &str) -> Result<u64> {
    let puzzle: Puzzle = input.parse()?;
    puzzle
        .initial_seeds
        .iter()
        .map(|s| puzzle.map(*s))
        .min()
        .ok_or(anyhow!("No initial seeds"))
}
