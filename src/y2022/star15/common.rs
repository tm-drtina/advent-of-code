use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

use crate::utils::point::Point2D;

type Point = Point2D<i32>;

#[derive(Debug, Clone, Copy)]
pub(super) struct Sensor {
    pub(super) position: Point,
    pub(super) beacon: Point,
    pub(super) distance: i32,
}

impl FromStr for Sensor {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix("Sensor at x=")
            .ok_or_else(|| anyhow!("Invalid format"))?;
        let (p_x, s) = s
            .split_once(", y=")
            .ok_or_else(|| anyhow!("Invalid format"))?;
        let (p_y, s) = s
            .split_once(": closest beacon is at x=")
            .ok_or_else(|| anyhow!("Invalid format"))?;
        let (b_x, b_y) = s
            .split_once(", y=")
            .ok_or_else(|| anyhow!("Invalid format"))?;

        let position = Point2D {
            x: p_x.parse()?,
            y: p_y.parse()?,
        };
        let beacon = Point2D {
            x: b_x.parse()?,
            y: b_y.parse()?,
        };
        Ok(Self {
            position,
            beacon,
            distance: position.manhattan_distance(beacon),
        })
    }
}
