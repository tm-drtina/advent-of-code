use std::collections::HashSet;

use anyhow::{anyhow, Result};

use crate::utils::point::Point3D;

pub fn run(input: &str) -> Result<usize> {
    let mut res = 0;
    let mut points = HashSet::new();
    for line in input.lines() {
        let mut split = line.split(',');
        let p: Point3D<i32> = Point3D {
            x: split
                .next()
                .ok_or_else(|| anyhow!("Invalid format"))?
                .parse()?,
            y: split
                .next()
                .ok_or_else(|| anyhow!("Invalid format"))?
                .parse()?,
            z: split
                .next()
                .ok_or_else(|| anyhow!("Invalid format"))?
                .parse()?,
        };
        res += 6;
        points.insert(p);
        p.six_neighborhood()
            .iter()
            .filter(|p| points.contains(p))
            .for_each(|_| res -= 2);
    }
    Ok(res)
}
