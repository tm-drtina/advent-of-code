pub fn run(input: &str) -> usize {
    let mut map = super::part1::parse(input);

    (1..)
        .map(|index| (index, super::part1::step(&mut map)))
        .find(|(_index, value)| *value == 100)
        .unwrap()
        .0
}
