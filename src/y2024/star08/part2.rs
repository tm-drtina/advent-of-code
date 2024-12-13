use std::collections::BTreeSet;

use anyhow::Result;
use itertools::Itertools;

use crate::utils::point::Point2D;

use super::part1::Antennas;

struct PointIter {
    counter: usize,
    a: Point2D<usize>,
    b: Point2D<usize>,
    width: usize,
    height: usize,
}

impl Iterator for PointIter {
    type Item = Point2D<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        self.counter += 1;
        let x = ((1 + self.counter) * self.a.x).checked_sub(self.counter * self.b.x)?;
        let y = ((1 + self.counter) * self.a.y).checked_sub(self.counter * self.b.y)?;
        
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(Point2D { x, y })
        }
    }
}

impl Antennas {
    fn point_iter(&self, a: Point2D<usize>, b: Point2D<usize>) -> PointIter {
        PointIter { counter: 0, a, b, width: self.width, height: self.height }
    }

    fn antinodes_part2(&self) -> usize {
        let mut antinodes = BTreeSet::<Point2D<usize>>::new();
        
        for collection in self.antennas.values() {
            if collection.len() > 1 {
                antinodes.extend(collection);
            }
            for (&a, &b) in collection.iter().tuple_combinations() {
                for point in self.point_iter(a, b).chain(self.point_iter(b, a)) {
                    antinodes.insert(point);
                }
            }
        }

        /*for y in 0..self.height {
            for x in 0..self.width {
                if antinodes.contains(&Point2D { x, y }) {
                    eprint!("#")
                } else {
                    eprint!(".")
                }
            }
            eprintln!()
        }*/

        antinodes.len()
    }
}

pub fn run(input: &str) -> Result<usize> {
    Ok(input.parse::<Antennas>()?.antinodes_part2())
}
