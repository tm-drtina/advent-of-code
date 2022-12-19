use std::collections::{BTreeSet, HashMap};
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use pathfinding::prelude::dijkstra_all;

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
            .map(Self::name_to_id)
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
struct StateKey {
    position: u16,
    opened: BTreeSet<u16>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct StateValue {
    time: u32,
    pressure: u32,
}

fn next_states<'a>(
    key: &'a StateKey,
    value: &'a StateValue,
    puzzle: &'a Puzzle,
) -> impl Iterator<Item = (StateKey, StateValue)> + 'a {
    dijkstra_all(&key.position, |node| {
        puzzle.valves[node].tunnels.iter().map(|t| (*t, 1))
    })
    .into_iter()
    .filter(|(dest, _)| !key.opened.contains(dest))
    .filter(|(dest, _)| puzzle.valves[dest].rate > 0)
    .filter(|(_, (_, time))| value.time + time < 29)
    .map(|(dest, (_, cost))| {
        (
            StateKey {
                position: dest,
                opened: {
                    let mut tmp = key.opened.clone();
                    tmp.insert(dest);
                    tmp
                },
            },
            StateValue {
                time: value.time + cost + 1,
                pressure: value.pressure + puzzle.valves[&dest].rate * (30 - value.time - cost),
            },
        )
    })
}

pub fn run(input: &str) -> Result<u32> {
    let puzzle = input.parse::<Puzzle>()?;

    let mut curr = HashMap::new();
    curr.insert(
        StateKey { position: Valve::name_to_id("AA"), opened: BTreeSet::new() },
        vec![StateValue { time: 1, pressure: 0 }]
    );
    let mut res = 0;

    loop {
        let mut next: HashMap<StateKey, Vec<StateValue>> = HashMap::new();
        for (key, values) in curr {
            for value in values {
                let mut last = true;
                for (key, value) in next_states(&key, &value, &puzzle) {
                    next.entry(key).or_default().push(value);
                    last = false;
                }
                if last && res < value.pressure {
                    res = value.pressure;
                }
            }
        }
        if next.is_empty() {
            break;
        }
        for values in next.values_mut() {
            *values = values.iter().copied().filter(|value| {
                !values.iter().any(|rhs| {
                    (value.pressure <= rhs.pressure && value.time > rhs.time) ||
                    (value.pressure < rhs.pressure && value.time >= rhs.time)
                }) 
            }).collect();
        }
        curr = next;
    }

    Ok(res)
}
