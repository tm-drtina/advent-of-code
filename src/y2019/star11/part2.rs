use std::collections::HashSet;

use itertools::Itertools;

use super::part1::{IntcodeProgram, Robot};

pub fn run(input: &str) -> String {
    let mut prog = IntcodeProgram::new(input);
    let mut robot = Robot::new();
    let mut white: HashSet<(i32, i32)> = HashSet::new();
    prog.input.push_back(1);
    white.insert(robot.position);
    loop {
        if prog.run_until_output().unwrap() {
            break;
        }
        match prog.output.pop_front().unwrap() {
            0 => {
                white.remove(&robot.position);
            }
            1 => {
                white.insert(robot.position);
            }
            _ => panic!("Invalid color"),
        }
        if prog.run_until_output().unwrap() {
            break;
        }
        match prog.output.pop_front().unwrap() {
            0 => robot.left(),
            1 => robot.right(),
            _ => panic!("Invalid rotation"),
        }
        robot.step();
        prog.input.push_back(if white.contains(&robot.position) {
            1
        } else {
            0
        });
    }
    (0..6)
        .map(|y| {
            (1..40)
                .map(|x| if white.contains(&(x, -y)) { "#" } else { " " })
                .join("")
        })
        .join("\n")
}
