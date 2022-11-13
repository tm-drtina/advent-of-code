use super::part1::{Area, Command};

pub fn run(input: &str) -> usize {
    let commands: Vec<Command> = input
        .lines()
        .map(|line| line.parse::<Command>().unwrap())
        .collect();
    let mut area = Area::new();
    for command in commands {
        area.step(command);
    }
    area.area()
}
