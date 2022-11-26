use std::str::FromStr;

use num::integer::lcm;

use super::part1::Simulation;

pub fn run(input: &str) -> u64 {
    let mut simulation = Simulation::from_str(input).unwrap();
    let start = simulation.clone();
    simulation.step_x();
    let mut steps_x = 1;
    while simulation != start {
        steps_x += 1;
        simulation.step_x();
    }
    simulation.step_y();
    let mut steps_y = 1;
    while simulation != start {
        steps_y += 1;
        simulation.step_y();
    }
    simulation.step_z();
    let mut steps_z = 1;
    while simulation != start {
        steps_z += 1;
        simulation.step_z();
    }
    lcm(lcm(steps_x, steps_y), steps_z)
}
