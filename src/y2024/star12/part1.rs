use std::collections::BTreeSet;

use anyhow::Result;

use crate::utils::map::Map;
use crate::utils::point::Point2D;

pub fn run(input: &str) -> Result<usize> {
    let map: Map<char> = Map::from_str(input, |ch, _x, _y| ch);

    let mut res = 0;
    let mut visited = BTreeSet::<Point2D<usize>>::new();

    for (point, &ch) in map.iter_values() {
        if !visited.insert(point) {
            continue;
        }

        let mut area = 0;
        let mut edges = 0;
        let mut to_visit = BTreeSet::new();
        to_visit.insert(point);

        while let Some(neighbor) = to_visit.pop_first() {
            let neighborhood = map
                .four_neighborhood(neighbor)
                .into_iter()
                .filter(|pt| *map.at(*pt) == ch)
                .collect::<Vec<_>>();

            visited.insert(neighbor);
            edges += 4 - neighborhood.len();
            area += 1;

            to_visit.extend(neighborhood.into_iter().filter(|pt| !visited.contains(pt)));
        }

        res += area * edges;
    }

    Ok(res)
}
