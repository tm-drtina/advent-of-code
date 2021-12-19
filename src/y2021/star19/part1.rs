type Rotation = ((usize, usize, usize), (isize, isize, isize));

// TODO: set rotations
const ROTATIONS: [Rotation; 24] = [
    ((0, 1, 2), (1, 1, 1)); 24
];

#[derive(Debug)]
pub(super) struct Sensor {
    rotated_beacons: Vec<Vec<(isize, isize, isize)>>,
}

#[derive(Debug)]
pub(super) struct AlignedSensor {
    beacons: Vec<(isize, isize, isize)>,
}

fn rotate_beacons(_rotation: &Rotation, _beacons: &[(isize, isize, isize)]) -> Vec<(isize, isize, isize)> {
    unimplemented!("Rotate beacon coordinates")
}

impl std::str::FromStr for Sensor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let beacons = s.parse::<AlignedSensor>()?.beacons;
        let rotated_beacons  = (&ROTATIONS).iter().map(|rotation| {
            rotate_beacons(rotation, &beacons)
        }).collect();
        Ok(Self{ rotated_beacons })
    }
}

impl std::str::FromStr for AlignedSensor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        lines.next(); // header
        let beacons = lines.map(|line| {
            let mut parts = line.split(",");
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        }).collect();
        Ok(Self{ beacons })
    }
}

impl AlignedSensor {
    fn try_merge(&mut self, sensor: &Sensor) -> bool {
        for _beacons in sensor.rotated_beacons.iter() {
            unimplemented!("if beacons match, merge and return true")
        }
        false
    }
}

pub fn run(input: &str) -> usize {
    let mut input = input.split("\n\n");
    let mut aligned: AlignedSensor = input.next().unwrap().parse().unwrap();
    let mut sensors: Vec<Sensor> = input
        .map(|scanner| scanner.parse().unwrap())
        .collect();

    while !sensors.is_empty() {
        for sensor in std::mem::replace(&mut sensors, Vec::new()) {
            if !aligned.try_merge(&sensor) {
                sensors.push(sensor);
            }
        }
    }
    aligned.beacons.len()
}
