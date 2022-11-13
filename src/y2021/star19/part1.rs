use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Coord {
    X,
    Y,
    Z,
}
type Point3D = (isize, isize, isize);
type Rotation = ((Coord, Coord, Coord), Point3D);
const ROTATIONS: [Rotation; 24] = [
    ((Coord::X, Coord::Y, Coord::Z), (1, 1, 1)),
    ((Coord::X, Coord::Z, Coord::Y), (1, 1, -1)),
    ((Coord::X, Coord::Y, Coord::Z), (1, -1, -1)),
    ((Coord::X, Coord::Z, Coord::Y), (1, -1, 1)),
    ((Coord::X, Coord::Y, Coord::Z), (-1, -1, 1)),
    ((Coord::X, Coord::Z, Coord::Y), (-1, -1, -1)),
    ((Coord::X, Coord::Y, Coord::Z), (-1, 1, -1)),
    ((Coord::X, Coord::Z, Coord::Y), (-1, 1, 1)),
    ((Coord::Y, Coord::X, Coord::Z), (1, -1, 1)),
    ((Coord::Y, Coord::Z, Coord::X), (1, -1, -1)),
    ((Coord::Y, Coord::X, Coord::Z), (1, 1, -1)),
    ((Coord::Y, Coord::Z, Coord::X), (1, 1, 1)),
    ((Coord::Y, Coord::X, Coord::Z), (-1, 1, 1)),
    ((Coord::Y, Coord::Z, Coord::X), (-1, 1, -1)),
    ((Coord::Y, Coord::X, Coord::Z), (-1, -1, -1)),
    ((Coord::Y, Coord::Z, Coord::X), (-1, -1, 1)),
    ((Coord::Z, Coord::Y, Coord::X), (1, 1, -1)),
    ((Coord::Z, Coord::X, Coord::Y), (1, 1, 1)),
    ((Coord::Z, Coord::Y, Coord::X), (1, -1, 1)),
    ((Coord::Z, Coord::X, Coord::Y), (1, -1, -1)),
    ((Coord::Z, Coord::Y, Coord::X), (-1, 1, 1)),
    ((Coord::Z, Coord::X, Coord::Y), (-1, 1, -1)),
    ((Coord::Z, Coord::Y, Coord::X), (-1, -1, -1)),
    ((Coord::Z, Coord::X, Coord::Y), (-1, -1, 1)),
];

#[derive(Debug)]
pub(super) struct Sensor {
    rotated_beacons: Vec<Vec<Point3D>>,
    mismatch: HashSet<usize>,
}

#[derive(Debug)]
pub(super) struct AlignedSensor {
    sub_sensors: Vec<Vec<Point3D>>,
    distances: Vec<Point3D>,
}

fn coord(coord: Coord, orig: Point3D) -> isize {
    match coord {
        Coord::X => orig.0,
        Coord::Y => orig.1,
        Coord::Z => orig.2,
    }
}

fn rotate_beacons(rotation: &Rotation, beacons: &[Point3D]) -> Vec<Point3D> {
    beacons
        .iter()
        .copied()
        .map(|orig| {
            (
                coord(rotation.0 .0, orig) * rotation.1 .0,
                coord(rotation.0 .1, orig) * rotation.1 .1,
                coord(rotation.0 .2, orig) * rotation.1 .2,
            )
        })
        .collect()
}

impl std::str::FromStr for Sensor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let beacons = &s.parse::<AlignedSensor>()?.sub_sensors[0];
        let rotated_beacons = ROTATIONS
            .iter()
            .map(|rotation| rotate_beacons(rotation, beacons))
            .collect();
        Ok(Self {
            rotated_beacons,
            mismatch: HashSet::new(),
        })
    }
}

impl std::str::FromStr for AlignedSensor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        lines.next(); // header
        let beacons = lines
            .map(|line| {
                let mut parts = line.split(',');
                (
                    parts.next().unwrap().parse().unwrap(),
                    parts.next().unwrap().parse().unwrap(),
                    parts.next().unwrap().parse().unwrap(),
                )
            })
            .collect();
        Ok(Self {
            sub_sensors: vec![beacons],
            distances: vec![(0, 0, 0)],
        })
    }
}

fn align(orig: &[Point3D], beacons: &[Point3D]) -> Option<Point3D> {
    for p1 in orig {
        for p2 in beacons {
            let alignment = (p1.0 - p2.0, p1.1 - p2.1, p1.2 - p2.2);
            let mut count = 0;
            for beacon in beacons {
                let aligned = (
                    beacon.0 + alignment.0,
                    beacon.1 + alignment.1,
                    beacon.2 + alignment.2,
                );
                if orig.contains(&aligned) {
                    count += 1;
                }
            }
            if count >= 12 {
                return Some(alignment);
            }
        }
    }
    None
}

impl AlignedSensor {
    fn try_align(&mut self, sensor: &mut Sensor) -> Option<(Vec<Point3D>, Point3D)> {
        for (index, aligned_sub_sensor) in self.sub_sensors.iter().enumerate() {
            if sensor.mismatch.contains(&index) {
                continue;
            }
            for beacons in sensor.rotated_beacons.iter() {
                if let Some(alignment) = align(aligned_sub_sensor, beacons) {
                    return Some((
                        beacons
                            .iter()
                            .copied()
                            .map(|beacon| {
                                (
                                    beacon.0 + alignment.0,
                                    beacon.1 + alignment.1,
                                    beacon.2 + alignment.2,
                                )
                            })
                            .collect(),
                        alignment,
                    ));
                }
            }
            sensor.mismatch.insert(index);
        }
        None
    }

    pub(super) fn align_and_merge(&mut self, mut sensors: Vec<Sensor>) {
        while !sensors.is_empty() {
            for mut sensor in std::mem::take(&mut sensors) {
                if let Some((aligned_sensor, alignment)) = self.try_align(&mut sensor) {
                    self.sub_sensors.push(aligned_sensor);
                    self.distances.push(alignment);
                } else {
                    sensors.push(sensor);
                }
            }
        }
    }

    fn count_unique(&self) -> usize {
        let unique = self
            .sub_sensors
            .iter()
            .flatten()
            .copied()
            .collect::<HashSet<_>>();
        unique.len()
    }

    pub(super) fn largest_distance(&self) -> usize {
        self.distances
            .iter()
            .tuple_combinations()
            .map(|(d1, d2)| {
                ((d1.0 - d2.0).abs() + (d1.1 - d2.1).abs() + (d1.2 - d2.2).abs()) as usize
            })
            .max()
            .unwrap()
    }
}

pub fn run(input: &str) -> usize {
    let mut input = input.split("\n\n");
    let mut aligned: AlignedSensor = input.next().unwrap().parse().unwrap();
    let sensors: Vec<Sensor> = input.map(|scanner| scanner.parse().unwrap()).collect();
    aligned.align_and_merge(sensors);
    aligned.count_unique()
}
