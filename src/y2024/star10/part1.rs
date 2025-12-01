use std::collections::BTreeSet;

use anyhow::Result;

use crate::utils::map::Map;
use crate::utils::point::Point2D;

fn trailhead_score(map: &Map<u8>, start: Point2D<usize>) -> usize {
    let mut starts = BTreeSet::new();
    starts.insert(start);

    for i in 1..=9 {
        starts = starts
            .into_iter()
            .flat_map(|pt| map.four_neighborhood(pt))
            .filter(|pt| *map.at(*pt) == i)
            .collect();
    }
    starts.len()
}

pub fn run(input: &str) -> Result<usize> {
    let mut trailheads = Vec::<Point2D<usize>>::new();
    let map = Map::from_str(input, |ch, x, y| {
        if ch == '0' {
            trailheads.push(Point2D { x, y });
        }
        ch as u8 - b'0'
    });

    Ok(trailheads
        .into_iter()
        .map(|start| trailhead_score(&map, start))
        .sum())
}
