use std::str::FromStr;

use anyhow::{anyhow, Context, Result};

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn count_wins(&self) -> u64 {
        // n * (time - n) > distance
        // time > n > 0
        // -> -n^2 + time*n - distance = 0
        // integers between
        // (time +- sqrt(time*time - 4*distance)) / 2
        let mut lower_bound = ((self.time as f64
            - ((self.time * self.time - 4 * self.distance) as f64).sqrt())
            / 2.0)
            .ceil() as u64;
        let mut upper_bound = ((self.time as f64
            + ((self.time * self.time - 4 * self.distance) as f64).sqrt())
            / 2.0)
            .floor() as u64;
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

impl FromStr for Race {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();
        let time = lines
            .next()
            .ok_or(anyhow!("Missing 'times' line"))?
            .strip_prefix("Time:")
            .ok_or(anyhow!("Missing 'Time:' prefix"))?
            .replace(' ', "")
            .parse()
            .context("Failed to parse time")?;
        let distance = lines
            .next()
            .ok_or(anyhow!("Missing 'distances' line"))?
            .strip_prefix("Distance:")
            .ok_or(anyhow!("Missing 'Distance:' prefix"))?
            .replace(' ', "")
            .parse()
            .context("Failed to parse distance")?;

        Ok(Race { time, distance })
    }
}

pub fn run(input: &str) -> Result<u64> {
    Ok(input.parse::<Race>()?.count_wins())
}
