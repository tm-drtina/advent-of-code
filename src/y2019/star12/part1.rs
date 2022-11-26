use std::convert::Infallible;
use std::str::FromStr;

use crate::utils::point::Point3D;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Moon {
    position: Point3D<i32>,
    velocity: Point3D<i32>,
}

impl FromStr for Moon {
    type Err = Infallible;

    // <x=6, y=10, z=10>

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = s[1..s.len() - 1]
            .splitn(3, ", ")
            .map(|s| &s[2..])
            .map(str::parse::<i32>);
        let x = coords.next().unwrap().unwrap();
        let y = coords.next().unwrap().unwrap();
        let z = coords.next().unwrap().unwrap();
        Ok(Self {
            position: Point3D { x, y, z },
            velocity: Point3D::default(),
        })
    }
}

impl Moon {
    fn energy(&self) -> i32 {
        self.position.manhattan_distance() * self.velocity.manhattan_distance()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Simulation {
    moons: [Moon; 4],
}

impl FromStr for Simulation {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        Ok(Self {
            moons: [
                Moon::from_str(lines.next().unwrap()).unwrap(),
                Moon::from_str(lines.next().unwrap()).unwrap(),
                Moon::from_str(lines.next().unwrap()).unwrap(),
                Moon::from_str(lines.next().unwrap()).unwrap(),
            ],
        })
    }
}

impl Simulation {
    fn compute_velocity_x(&mut self) {
        for i in 0..4 {
            for j in i..4 {
                match self.moons[i].position.x.cmp(&self.moons[j].position.x) {
                    std::cmp::Ordering::Less => {
                        self.moons[i].velocity.x += 1;
                        self.moons[j].velocity.x -= 1;
                    }
                    std::cmp::Ordering::Equal => {}
                    std::cmp::Ordering::Greater => {
                        self.moons[i].velocity.x -= 1;
                        self.moons[j].velocity.x += 1;
                    }
                }
            }
        }
    }

    fn compute_velocity_y(&mut self) {
        for i in 0..4 {
            for j in i..4 {
                match self.moons[i].position.y.cmp(&self.moons[j].position.y) {
                    std::cmp::Ordering::Less => {
                        self.moons[i].velocity.y += 1;
                        self.moons[j].velocity.y -= 1;
                    }
                    std::cmp::Ordering::Equal => {}
                    std::cmp::Ordering::Greater => {
                        self.moons[i].velocity.y -= 1;
                        self.moons[j].velocity.y += 1;
                    }
                }
            }
        }
    }

    fn compute_velocity_z(&mut self) {
        for i in 0..4 {
            for j in i..4 {
                match self.moons[i].position.z.cmp(&self.moons[j].position.z) {
                    std::cmp::Ordering::Less => {
                        self.moons[i].velocity.z += 1;
                        self.moons[j].velocity.z -= 1;
                    }
                    std::cmp::Ordering::Equal => {}
                    std::cmp::Ordering::Greater => {
                        self.moons[i].velocity.z -= 1;
                        self.moons[j].velocity.z += 1;
                    }
                }
            }
        }
    }

    fn move_moons(&mut self) {
        for i in 0..4 {
            self.moons[i].position += self.moons[i].velocity;
        }
    }

    pub(super) fn step(&mut self) {
        self.compute_velocity_x();
        self.compute_velocity_y();
        self.compute_velocity_z();
        self.move_moons();
    }

    pub(super) fn step_x(&mut self) {
        self.compute_velocity_x();
        self.move_moons();
    }

    pub(super) fn step_y(&mut self) {
        self.compute_velocity_y();
        self.move_moons();
    }

    pub(super) fn step_z(&mut self) {
        self.compute_velocity_z();
        self.move_moons();
    }

    fn energy(&self) -> i32 {
        self.moons.iter().map(Moon::energy).sum()
    }
}

pub fn run(input: &str, steps: usize) -> i32 {
    let mut simulation = Simulation::from_str(input).unwrap();
    for _ in 0..steps {
        simulation.step();
    }
    simulation.energy()
}
