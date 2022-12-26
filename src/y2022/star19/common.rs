use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use rayon::prelude::*;

use crate::utils::cache::sync::Cache;

#[derive(Debug, Clone, Copy)]
pub(super) struct Blueprint {
    id: u32,
    ore_robot: u32,
    clay_robot: u32,
    obsidian_robot: (u32, u32),
    geode_robot: (u32, u32),
    max_ore: u32,
    max_clay: u32,
    max_obsidian: u32,
}

impl FromStr for Blueprint {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix("Blueprint ")
            .ok_or_else(|| anyhow!("Invalid format"))?;
        let (id, s) = s
            .split_once(": Each ore robot costs ")
            .ok_or_else(|| anyhow!("Invalid format"))?;
        let (ore_robot_ore, s) = s
            .split_once(" ore. Each clay robot costs ")
            .ok_or_else(|| anyhow!("Invalid format"))?;
        let (clay_robot_ore, s) = s
            .split_once(" ore. Each obsidian robot costs ")
            .ok_or_else(|| anyhow!("Invalid format"))?;
        let (obsidian_robot_ore, s) = s.split_once(" ore and ").ok_or_else(|| anyhow!("Invalid format"))?;
        let (obsidian_robot_clay, s) = s
            .split_once(" clay. Each geode robot costs ")
            .ok_or_else(|| anyhow!("Invalid format"))?;
        let (geode_robot_ore, s) = s.split_once(" ore and ").ok_or_else(|| anyhow!("Invalid format"))?;
        let geode_robot_obsidian = s
            .strip_suffix(" obsidian.")
            .ok_or_else(|| anyhow!("Invalid format"))?;

        let ore_robot = ore_robot_ore.parse()?;
        let clay_robot = clay_robot_ore.parse()?;
        let obsidian_robot = (obsidian_robot_ore.parse()?, obsidian_robot_clay.parse()?);
        let geode_robot = (geode_robot_ore.parse()?, geode_robot_obsidian.parse()?);

        Ok(Self {
            id: id.parse()?,
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
            max_ore: [ore_robot, clay_robot, obsidian_robot.0, geode_robot.0]
                .into_iter()
                .max()
                .unwrap(),
            max_clay: obsidian_robot.1,
            max_obsidian: geode_robot.1,
        })
    }
}

impl Blueprint {
    pub(super) fn max_geodes(self, minutes: u32) -> u32 {
        let cache = Cache::new(Box::new(move |state: &State, cache| {
            if state.minute == minutes {
                state.materials.geode + state.robots.geode
            } else {
                state
                    .next_states(self)
                    .into_par_iter()
                    .map(|s| cache.get_or_compute(s))
                    .max()
                    .unwrap()
            }
        }));
        cache.get_or_compute(State::init())
    }

    pub(super) fn quality(&self, minutes: u32) -> u32 {
        self.max_geodes(minutes) * self.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Items {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl Items {
    fn ore() -> Self {
        Self {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn clay() -> Self {
        Self {
            ore: 0,
            clay: 1,
            obsidian: 0,
            geode: 0,
        }
    }

    fn obsidian() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 1,
            geode: 0,
        }
    }

    fn geode() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 1,
        }
    }

    fn none() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
}

impl std::ops::AddAssign for Items {
    fn add_assign(&mut self, rhs: Self) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
    }
}

impl std::ops::Add for Items {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct State {
    minute: u32,
    robots: Items,
    materials: Items,
}

impl State {
    fn init() -> Self {
        Self {
            minute: 1,
            robots: Items::ore(),
            materials: Items::none(),
        }
    }

    fn next_states(&self, blueprint: Blueprint) -> Vec<Self> {
        if blueprint.geode_robot.0 <= self.materials.ore
            && blueprint.geode_robot.1 <= self.materials.obsidian
        {
            let mut materials = self.materials;
            materials.ore -= blueprint.geode_robot.0;
            materials.obsidian -= blueprint.geode_robot.1;
            materials += self.robots;
            return vec![State {
                minute: self.minute + 1,
                robots: self.robots + Items::geode(),
                materials,
            }];
        }
        let mut res = Vec::with_capacity(4);
        if blueprint.obsidian_robot.0 <= self.materials.ore
            && blueprint.obsidian_robot.1 <= self.materials.clay
            && self.robots.obsidian < blueprint.max_obsidian
        {
            let mut materials = self.materials;
            materials.ore -= blueprint.obsidian_robot.0;
            materials.clay -= blueprint.obsidian_robot.1;
            materials += self.robots;
            res.push(State {
                minute: self.minute + 1,
                robots: self.robots + Items::obsidian(),
                materials,
            });
        }
        if blueprint.clay_robot <= self.materials.ore && self.robots.clay < blueprint.max_clay {
            let mut materials = self.materials;
            materials.ore -= blueprint.clay_robot;
            materials += self.robots;
            res.push(State {
                minute: self.minute + 1,
                robots: self.robots + Items::clay(),
                materials,
            });
        }
        if blueprint.ore_robot <= self.materials.ore && self.robots.ore < blueprint.max_ore {
            let mut materials = self.materials;
            materials.ore -= blueprint.ore_robot;
            materials += self.robots;
            res.push(State {
                minute: self.minute + 1,
                robots: self.robots + Items::ore(),
                materials,
            });
        }
        res.push(State {
            minute: self.minute + 1,
            robots: self.robots,
            materials: self.materials + self.robots,
        });
        res
    }
}
