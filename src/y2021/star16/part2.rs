use super::part1::{parse, HexStringIter};

pub fn run(input: &str) -> usize {
    let mut iter = HexStringIter::new(input);
    let packet = parse(&mut iter);
    packet.compute_value()
}
