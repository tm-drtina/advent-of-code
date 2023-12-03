use anyhow::Result;

use super::part1::{Part, Puzzle};
use crate::utils::point::Point2D;

impl Puzzle {
    fn gears_iter(self) -> GearsIter<impl Iterator<Item = Point2D<usize>>> {
        GearsIter {
            parts: self.parts,
            gears: self.symbols.into_iter().filter_map(
                |(k, v)| {
                    if v == '*' { Some(k) } else { None }
                },
            ),
        }
    }
}

struct GearsIter<I> {
    parts: Vec<Part>,
    gears: I,
}
impl<I: Iterator<Item = Point2D<usize>>> Iterator for GearsIter<I> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let gear = self.gears.next()?;
            let parts = self
                .parts
                .iter()
                .filter(|p| {
                    p.row.abs_diff(gear.y) <= 1
                        && (p.col.contains(&gear.x)
                            || p.col.start().abs_diff(gear.x) <= 1
                            || p.col.end().abs_diff(gear.x) <= 1)
                })
                .collect::<Vec<_>>();
            if parts.len() == 2 {
                return Some(parts[0].value * parts[1].value);
            }
        }
    }
}

pub fn run(input: &str) -> Result<u32> {
    Ok(input.parse::<Puzzle>()?.gears_iter().sum())
}
