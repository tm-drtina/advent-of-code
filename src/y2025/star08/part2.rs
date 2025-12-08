use itertools::Itertools;

use super::part1::Agg;
use crate::utils::point::Point3D;

pub fn run(input: &str) -> u64 {
    let points = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            let z = parts.next().unwrap().parse().unwrap();
            Point3D::<u32> { x, y, z }
        })
        .collect::<Vec<_>>();

    let mut connections = points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a.euclidean_distance(b), *a, *b))
        .collect::<Vec<_>>();

    connections.sort_unstable_by(|(d1, _, _), (d2, _, _)| d1.partial_cmp(d2).unwrap());
    let mut remaining = points.len() - 1;
    let mut agg = Agg::new(points);
    for (_d, a, b) in connections {
        if agg.merge(a, b) {
            remaining -= 1;
            if remaining == 0 {
                return (a.x as u64) * (b.x as u64);
            }
        }
    }
    panic!("No result found");
}
