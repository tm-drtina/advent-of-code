use super::part1::{HexStringIter, parse};

pub fn run(input: &str) -> usize {
    let mut iter = HexStringIter::new(input);
    let packet = parse(&mut iter);
    packet.compute_value()
}
