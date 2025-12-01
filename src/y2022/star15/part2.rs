use anyhow::{Result, bail};

use super::common::Sensor;
use crate::utils::point::Point2D;

pub fn run(input: &str, check_level: i32) -> Result<i64> {
    let sensors = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Sensor>>>()?;

    for y in 0..=check_level {
        let mut x_iter = 0..=check_level;
        while !x_iter.is_empty() {
            let x = *x_iter.start();
            let point = Point2D { x, y };
            let skip = sensors
                .iter()
                .filter_map(|s| {
                    let dist = point.manhattan_distance(s.position);
                    if dist <= s.distance {
                        Some(s.distance - dist)
                    } else {
                        None
                    }
                })
                .max();
            match skip {
                Some(skip) => {
                    x_iter = x + skip + 1..=*x_iter.end();
                }
                None => return Ok(x as i64 * 4_000_000 + y as i64),
            }
        }
    }
    bail!("No solution found!");
}
