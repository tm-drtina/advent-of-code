use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::str::FromStr;

use anyhow::Result;

use crate::utils::point::Point2D;

#[derive(Debug)]
pub(super) struct Part {
    pub(super) value: u32,
    pub(super) row: usize,
    pub(super) col: RangeInclusive<usize>,
}

#[derive(Debug)]
pub(super) struct Puzzle {
    pub(super) parts: Vec<Part>,
    pub(super) symbols: HashMap<Point2D<usize>, char>,
}

impl FromStr for Puzzle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut puzzle = Self {
            parts: Vec::new(),
            symbols: HashMap::new(),
        };
        for (row, line) in s.lines().enumerate() {
            let mut part: Option<Part> = None;
            for (col, ch) in line.chars().enumerate() {
                if let Some(digit) = ch.to_digit(10) {
                    if let Some(part) = part.as_mut() {
                        part.value = part.value * 10 + digit;
                        part.col = *part.col.start()..=col;
                    } else {
                        part = Some(Part {
                            value: digit,
                            row,
                            col: col..=col,
                        });
                    }
                } else {
                    if let Some(part) = part.take() {
                        puzzle.parts.push(part);
                    }
                    if ch != '.' {
                        puzzle.symbols.insert(Point2D { x: col, y: row }, ch);
                    }
                }
            }
            if let Some(part) = part.take() {
                puzzle.parts.push(part);
            }
        }
        Ok(puzzle)
    }
}

impl Puzzle {
    fn valid_parts_iter(self) -> ValidPartsIter {
        ValidPartsIter {
            parts: self.parts.into_iter(),
            symbols: self.symbols,
        }
    }
}

struct ValidPartsIter {
    parts: <Vec<Part> as IntoIterator>::IntoIter,
    symbols: HashMap<Point2D<usize>, char>,
}
impl Iterator for ValidPartsIter {
    type Item = Part;

    fn next(&mut self) -> Option<Self::Item> {
        fn check_neighborhood(part: &Part, symbols: &HashMap<Point2D<usize>, char>) -> bool {
            for col in part.col.clone() {
                let point = Point2D {
                    x: col,
                    y: part.row,
                };
                for neigh in point.try_eight_neighborhood() {
                    if symbols.contains_key(&neigh) {
                        return true;
                    }
                }
            }
            false
        }

        loop {
            let part = self.parts.next()?;
            if check_neighborhood(&part, &self.symbols) {
                return Some(part);
            }
        }
    }
}

pub fn run(input: &str) -> Result<u32> {
    Ok(input
        .parse::<Puzzle>()?
        .valid_parts_iter()
        .map(|p| p.value)
        .sum())
}
