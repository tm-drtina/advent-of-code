use std::collections::{HashMap, BTreeSet};
use std::str::FromStr;
use std::sync::Arc;

use anyhow::{anyhow, Error, Result};
use rayon::prelude::*;

use crate::utils::cache::sync::Cache;

#[derive(Debug)]
struct Valve {
    id: u16,
    rate: u32,
    tunnels: Vec<u16>,
}

impl Valve {
    fn name_to_id(name: &str) -> u16 {
        let b = name.as_bytes();
        ((b[0] as u16) << 8) | (b[1] as u16)
    }
}

impl FromStr for Valve {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Valve ").ok_or(anyhow!("Invalid format"))?;
        let (name, s) = s
            .split_once(" has flow rate=")
            .ok_or(anyhow!("Invalid format"))?;
        let (rate, s) = s.split_once("; ").ok_or(anyhow!("Invalid format"))?;
        let tunnels = s
            .strip_prefix("tunnels lead to valves ")
            .or_else(|| s.strip_prefix("tunnel leads to valve "))
            .ok_or(anyhow!("Invalid format"))?
            .split(", ")
            .map(|s| Self::name_to_id(s))
            .collect();

        let id = Self::name_to_id(name);
        let rate = rate.parse()?;

        Ok(Self { id, rate, tunnels })
    }
}

#[derive(Debug)]
struct Puzzle {
    valves: HashMap<u16, Valve>,
}

impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let valves = s
            .lines()
            .map(|line| {
                let valve: Valve = line.parse()?;
                Ok((valve.id, valve))
            })
            .collect::<Result<_>>()?;
        Ok(Self { valves })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    time: u32,
    position: u16,
    pressure: u32,
    opened: Arc<BTreeSet<u16>>,
}

impl State {
    fn init() -> Self {
        Self {
            time: 1,
            position: Valve::name_to_id("AA"),
            pressure: 0,
            opened: Arc::new(BTreeSet::new()),
        }
    }

    fn search(&self, puzzle: &Puzzle, cache: &CacheT) -> u32 {
        if self.time > 29 {
            return self.pressure;
        }
        let valve = &puzzle.valves[&self.position];
        let mut next_states = Vec::with_capacity(valve.tunnels.len() + 1);
        if valve.rate > 0 && !self.opened.contains(&self.position) {
            let mut opened = BTreeSet::clone(&self.opened);
            opened.insert(self.position);
            let pressure = self.pressure + valve.rate * (30 - self.time + 1);

            next_states.push(Self {
                time: self.time + 1,
                position: self.position,
                pressure,
                opened: Arc::new(opened),
            });
        }

        for &next in &valve.tunnels {
            next_states.push(Self {
                time: self.time + 1,
                position: next,
                pressure: self.pressure,
                opened: Arc::clone(&self.opened),
            });
        }

        next_states
            .par_iter()
            .map(|s| s.search(puzzle, cache))
            .max()
            .unwrap_or(self.pressure)
    }
}

type CacheT = Cache<State, u32>;

pub fn run(input: &str) -> Result<u32> {
    let puzzle = input.parse::<Puzzle>()?;
    eprintln!("{:?}", puzzle);
    let cache = Cache::new(Box::new(move |state: &State, cache| {
        state.search(&puzzle, cache)
    }));
    Ok(cache.get_or_compute(State::init()))
}
