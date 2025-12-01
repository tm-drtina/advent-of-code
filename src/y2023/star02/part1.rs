use std::str::FromStr;

use anyhow::{Result, anyhow, bail};

pub(super) struct Game {
    id: usize,
    pub(super) red: usize,
    pub(super) green: usize,
    pub(super) blue: usize,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(mut s: &str) -> Result<Self> {
        s = s
            .strip_prefix("Game ")
            .ok_or(anyhow!("Missing 'Game ' prefix"))?;
        let (id, plays) = s
            .split_once(": ")
            .ok_or(anyhow!("Failed to split game id"))?;
        let id = id.parse()?;
        let mut game = Self {
            id,
            red: 0,
            green: 0,
            blue: 0,
        };

        for play in plays.split("; ") {
            for cubes in play.split(", ") {
                let (count, color) = cubes
                    .split_once(' ')
                    .ok_or(anyhow!("Failed to split count and color"))?;
                let count = count.parse()?;
                match color {
                    "red" => {
                        game.red = game.red.max(count);
                    }
                    "green" => {
                        game.green = game.green.max(count);
                    }
                    "blue" => {
                        game.blue = game.blue.max(count);
                    }
                    _ => bail!("Invalid color {color}"),
                }
            }
        }
        Ok(game)
    }
}

pub fn run(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(Game::from_str)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .filter(|g| g.red <= 12 && g.green <= 13 && g.blue <= 14)
        .map(|g| g.id)
        .sum())
}
