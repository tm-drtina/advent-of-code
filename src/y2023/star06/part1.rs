use std::str::FromStr;

use anyhow::{anyhow, Context, Result};

struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn count_wins(&self) -> u32 {
        // n * (time - n) > distance
        // time > n > 0
        // -> -n^2 + time*n - distance = 0
        // integers between
        // (time +- sqrt(time*time - 4*distance)) / 2
        let mut lower_bound = ((f64::from(self.time)
            - f64::from(self.time * self.time - 4 * self.distance).sqrt())
            / 2.0)
            .ceil() as u32;
        let mut upper_bound = ((f64::from(self.time)
            + f64::from(self.time * self.time - 4 * self.distance).sqrt())
            / 2.0)
            .floor() as u32;
        upper_bound = upper_bound.clamp(0, self.time);
        lower_bound = lower_bound.clamp(0, self.time);
        if lower_bound * (self.time - lower_bound) <= self.distance {
            lower_bound += 1;
        }
        if upper_bound * (self.time - upper_bound) <= self.distance {
            upper_bound -= 1;
        }
        upper_bound - lower_bound + 1
    }
}

struct Puzzle(Vec<Race>);

impl FromStr for Puzzle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();
        let times = lines
            .next()
            .ok_or(anyhow!("Missing 'times' line"))?
            .strip_prefix("Time:")
            .ok_or(anyhow!("Missing 'Time:' prefix"))?
            .split_ascii_whitespace()
            .map(|n| n.parse::<u32>().context("Failed to parse time"));
        let distances = lines
            .next()
            .ok_or(anyhow!("Missing 'distances' line"))?
            .strip_prefix("Distance:")
            .ok_or(anyhow!("Missing 'Distance:' prefix"))?
            .split_ascii_whitespace()
            .map(|n| n.parse::<u32>().context("Failed to parse distance"));

        let races = times
            .zip(distances)
            .map(|(time, distance)| {
                Ok(Race {
                    time: time?,
                    distance: distance?,
                })
            })
            .collect::<Result<_>>()?;
        Ok(Self(races))
    }
}

pub fn run(input: &str) -> Result<u32> {
    Ok(input
        .parse::<Puzzle>()?
        .0
        .into_iter()
        .map(|r| r.count_wins())
        .product())
}
