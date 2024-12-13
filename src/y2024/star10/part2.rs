use anyhow::Result;

use crate::utils::map::Map;
use crate::utils::point::Point2D;

pub fn run(input: &str) -> Result<usize> {
    let mut trailheads = Vec::<Point2D<usize>>::new();
    let mut map = Map::from_str(input, |ch, x, y| {
        if ch == '0' {
            trailheads.push(Point2D { x, y });
        }
        (ch as u8 - b'0', if ch == '9' { 1 } else { 0 })
    });

    for i in (0..9).rev() {
        let points = map
            .iter_values()
            .filter(|(_pt, (height, _score))| *height == i)
            .map(|(pt, _)| pt)
            .collect::<Vec<_>>();
        for point in points {
            let new_score: usize = map
                .four_neighborhood(point)
                .into_iter()
                .map(|pt| map.at(pt))
                .filter(|(height, _)| *height == i + 1)
                .map(|(_, score)| score)
                .sum();
            map.set(point, (i, new_score));
        }
    }

    Ok(map
        .iter_values()
        .filter(|(_, (height, _))| *height == 0)
        .map(|(_, (_, score))| *score)
        .sum())
}
