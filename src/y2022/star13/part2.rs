use std::iter::once;

use super::common::Packet;

pub fn run(input: &str) -> usize {
    let dec1 = Packet::from(&mut &b"[[2]]"[..]);
    let dec2 = Packet::from(&mut &b"[[6]]"[..]);
    let mut packets: Vec<_> = input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|line| Packet::from(&mut line.as_bytes()))
        .chain(once(dec1.clone()))
        .chain(once(dec2.clone()))
        .collect();
    packets.sort();
    let pos1 = packets.iter().position(|x| x == &dec1).unwrap();
    let pos2 = packets.iter().position(|x| x == &dec2).unwrap();
    (pos1 + 1) * (pos2 + 1)
}
