use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;

use anyhow::Result;
use itertools::Itertools;

use crate::utils::point::Point2D;

struct Antennas {
    antennas: BTreeMap<char, Vec<Point2D<usize>>>,
    height: usize,
    width: usize,
}

impl FromStr for Antennas {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut height = 0;
        let mut width = 0;
        let mut antennas: BTreeMap<char, Vec<Point2D<usize>>> = BTreeMap::new();

        for (y, line) in s.lines().enumerate() {
            height += 1;
            for (x, ch) in line.chars().enumerate() {
                if y == 0 {
                    width += 1;
                }

                if ch != '.' {
                    antennas.entry(ch).or_default().push(Point2D {x, y});
                }
            }
        }
        
        Ok(Self { antennas, height, width })
    }
}

impl Antennas {
    fn is_valid_point(&self, x: Option<usize>, y: Option<usize>) -> Option<Point2D<usize>> {
        let x = x?;
        let y = y?;
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(Point2D { x, y })
        }
    }

    fn antinodes(&self) -> usize {
        let mut antinodes = BTreeSet::<Point2D<usize>>::new();
        
        for collection in self.antennas.values() {
            for (a, b) in collection.iter().tuple_combinations() {
                let x1 = (2 * a.x).checked_sub(b.x);
                let y1 = (2 * a.y).checked_sub(b.y);
                let x2 = (2 * b.x).checked_sub(a.x);
                let y2 = (2 * b.y).checked_sub(a.y);
                if let Some(pt1) = self.is_valid_point(x1, y1) {
                    antinodes.insert(pt1);
                }
                if let Some(pt2) = self.is_valid_point(x2, y2) {
                    antinodes.insert(pt2);
                }
            }
        }

        antinodes.len()
    }
}

pub fn run(input: &str) -> Result<usize> {
    Ok(input.parse::<Antennas>()?.antinodes())
}
