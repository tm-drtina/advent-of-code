use std::collections::HashMap;

use anyhow::Result;
use itertools::Itertools;

use crate::utils::point::Point3D;

struct Agg {
    members: Vec<Vec<Point3D<u32>>>,
    member_of: HashMap<Point3D<u32>, usize>,
}
impl Agg {
    fn new(points: Vec<Point3D<u32>>) -> Self {
        let mut members = Vec::new();
        let mut member_of = HashMap::new();
        for (i, p) in points.into_iter().enumerate() {
            members.push(vec![p]);
            member_of.insert(p, i);
        }
        Self { members, member_of }
    }

    fn merge(&mut self, a: Point3D<u32>, b: Point3D<u32>) {
        let loc_a = self.member_of[&a];
        let loc_b = self.member_of[&b];
        if loc_a == loc_b {
            return
        }
        let (target, to_move) = if self.members[loc_a].len() >= self.members[loc_b].len() {
            (loc_a, std::mem::take(&mut self.members[loc_b]))
        } else {
            (loc_b, std::mem::take(&mut self.members[loc_a]))
        };

        self.members[target].extend_from_slice(&to_move);
        for p in to_move {
            self.member_of.insert(p, target);
        }
    }
}

pub fn run(input: &str, iters: usize) -> Result<usize> {
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
    let mut agg = Agg::new(points);

    for &(_d, a, b) in &connections[..iters] {
        agg.merge(a, b);
    }

    let mut sizes = agg.members.iter().map(Vec::len).collect::<Vec<_>>();
    sizes.sort_unstable_by_key(|x| std::cmp::Reverse(*x));

    Ok(sizes[0] * sizes[1] * sizes[2])
}
