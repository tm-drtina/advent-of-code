pub fn run(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_once(" | ")
                .unwrap()
                .1
                .split(' ')
                .filter(|num| matches!(num.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum()
}
