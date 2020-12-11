use itertools::Itertools;
use std::str::FromStr;

pub fn run(input: &str) -> i32 {
    input
        .lines()
        .map(|line| i32::from_str(line).unwrap())
        .permutations(2)
        .filter(|nums| {
            let i: i32 = nums.iter().sum();
            i == 2020
        })
        .map(|nums| nums.iter().product())
        .next()
        .unwrap()
}
