use std::collections::HashSet;

use anyhow::Result;

use super::common::Sensor;

pub fn run(input: &str, check_level: i32) -> Result<usize> {
    let sensors = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Sensor>>>()?;

    let mut blocked = HashSet::<i32>::new();
    for sensor in sensors {
        let dist = sensor.distance - sensor.position.y.abs_diff(check_level) as i32;
        if dist >= 0 {
            if sensor.beacon.y == check_level {
                for x in (sensor.position.x - dist)..(sensor.beacon.x) {
                    blocked.insert(x);
                }
                for x in (sensor.beacon.x + 1)..=(sensor.position.x + dist) {
                    blocked.insert(x);
                }
            } else {
                for x in (sensor.position.x - dist)..=(sensor.position.x + dist) {
                    blocked.insert(x);
                }
            }
        }
    }

    Ok(blocked.len())
}
