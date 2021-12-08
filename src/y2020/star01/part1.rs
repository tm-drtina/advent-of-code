use itertools::Itertools;

pub fn run(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .permutations(2)
        .filter(|nums| nums.iter().sum::<usize>() == 2020)
        .map(|nums| nums.iter().product())
        .next()
        .unwrap()
}
