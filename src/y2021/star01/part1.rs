use itertools::Itertools;

pub fn run(input: &str) -> usize {
    input
        .lines()
        .map(|s| s.parse::<u32>().unwrap())
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
}
