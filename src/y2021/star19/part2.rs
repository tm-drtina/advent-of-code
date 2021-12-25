use super::part1::{AlignedSensor, Sensor};

pub fn run(input: &str) -> usize {
    let mut input = input.split("\n\n");
    let mut aligned: AlignedSensor = input.next().unwrap().parse().unwrap();
    let sensors: Vec<Sensor> = input.map(|scanner| scanner.parse().unwrap()).collect();
    aligned.align_and_merge(sensors);
    aligned.largest_distance()
}