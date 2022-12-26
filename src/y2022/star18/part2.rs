use std::collections::HashSet;

use anyhow::{anyhow, Result};

use crate::utils::point::Point3D;

type Point = Point3D<i32>;

fn min_max<T: Iterator<Item = Point>>(points: T) -> (Point, Point) {
    points.fold(
        (Point3D { x: 1, y: 1, z: 1 }, Point3D { x: 1, y: 1, z: 1 }),
        |(pmin, pmax), p| {
            (
                Point3D {
                    x: pmin.x.min(p.x),
                    y: pmin.y.min(p.y),
                    z: pmin.z.min(p.z),
                },
                Point3D {
                    x: pmax.x.max(p.x),
                    y: pmax.y.max(p.y),
                    z: pmax.z.max(p.z),
                },
            )
        },
    )
}

fn find_bubble(p: Point, points: &mut HashSet<Point>, pmin: Point, pmax: Point) -> usize {
    let mut visited = HashSet::new();
    if points.contains(&p) {
        return 0;
    }
    let mut to_visit = vec![p];
    let mut res = 0;
    while !to_visit.is_empty() {
        let point = to_visit.pop().unwrap();
        if !visited.insert(point) {
            continue;
        }
        for n in point.six_neighborhood() {
            if n.x < pmin.x
                || n.y < pmin.y
                || n.z < pmin.z
                || n.x > pmax.x
                || n.y > pmax.y
                || n.z > pmax.z
            {
                return 0;
            }
            if visited.contains(&n) {
                // don't care
            } else if points.contains(&n) {
                res += 1;
            } else {
                to_visit.push(n);
            }
        }
    }
    for p in visited {
        points.insert(p);
    }
    res
}

pub fn run(input: &str) -> Result<usize> {
    let mut res = 0;
    let mut points = HashSet::new();
    for line in input.lines() {
        let mut split = line.split(',');
        let p: Point = Point3D {
            x: split.next().ok_or_else(|| anyhow!("Invalid format"))?.parse()?,
            y: split.next().ok_or_else(|| anyhow!("Invalid format"))?.parse()?,
            z: split.next().ok_or_else(|| anyhow!("Invalid format"))?.parse()?,
        };
        res += 6;
        points.insert(p);
        p.six_neighborhood()
            .iter()
            .filter(|p| points.contains(p))
            .for_each(|_| res -= 2);
    }

    let (pmin, pmax) = min_max(points.iter().copied());

    for x in pmin.x + 1..pmax.x {
        for y in pmin.y + 1..pmax.y {
            for z in pmin.z + 1..pmax.z {
                res -= find_bubble(Point3D { x, y, z }, &mut points, pmin, pmax);
            }
        }
    }

    Ok(res)
}
