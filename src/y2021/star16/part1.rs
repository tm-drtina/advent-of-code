use std::str::Chars;

pub(super) struct HexStringIter<'a> {
    chars: Chars<'a>,
    current: Vec<u8>,
    counter: usize,
}
impl<'a> HexStringIter<'a> {
    pub(super) fn new(s: &'a str) -> Self {
        Self {
            chars: s.chars(),
            current: Vec::new(),
            counter: 0,
        }
    }

    fn next_int(&mut self, bit_len: usize) -> usize {
        let mut res = 0;

        for _ in 0..bit_len {
            res = (res << 1) + self.next().unwrap() as usize;
        }

        res
    }
}
impl<'a> Iterator for HexStringIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.current.pop();
        if next.is_some() {
            self.counter += 1;
            next
        } else if let Some(next_char) = self.chars.next() {
            self.current = match next_char {
                '0' => vec![0, 0, 0, 0],
                '1' => vec![1, 0, 0, 0],
                '2' => vec![0, 1, 0, 0],
                '3' => vec![1, 1, 0, 0],
                '4' => vec![0, 0, 1, 0],
                '5' => vec![1, 0, 1, 0],
                '6' => vec![0, 1, 1, 0],
                '7' => vec![1, 1, 1, 0],
                '8' => vec![0, 0, 0, 1],
                '9' => vec![1, 0, 0, 1],
                'A' => vec![0, 1, 0, 1],
                'B' => vec![1, 1, 0, 1],
                'C' => vec![0, 0, 1, 1],
                'D' => vec![1, 0, 1, 1],
                'E' => vec![0, 1, 1, 1],
                'F' => vec![1, 1, 1, 1],
                _ => panic!("Invalid hex char!"),
            };
            self.counter += 1;
            self.current.pop()
        } else {
            None
        }
    }
}

enum PacketValue {
    Literal(usize),
    SubPackets(Vec<Packet>),
}

pub(super) struct Packet {
    p_version: usize,
    p_type: usize,
    value: PacketValue,
}

impl Packet {
    fn version_sum(&self) -> usize {
        match self.value {
            PacketValue::Literal(_) => self.p_version,
            PacketValue::SubPackets(ref packets) => {
                self.p_version + packets.iter().map(Packet::version_sum).sum::<usize>()
            }
        }
    }

    pub(super) fn compute_value(&self) -> usize {
        match self.value {
            PacketValue::Literal(value) => value,
            PacketValue::SubPackets(ref packets) => match self.p_type {
                0 => packets.iter().map(Packet::compute_value).sum(),
                1 => packets.iter().map(Packet::compute_value).product(),
                2 => packets.iter().map(Packet::compute_value).min().unwrap(),
                3 => packets.iter().map(Packet::compute_value).max().unwrap(),
                5 => {
                    if packets[0].compute_value() > packets[1].compute_value() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if packets[0].compute_value() < packets[1].compute_value() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if packets[0].compute_value() == packets[1].compute_value() {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            },
        }
    }
}

pub(super) fn parse(iter: &mut HexStringIter<'_>) -> Packet {
    let p_version = iter.next_int(3);
    let p_type = iter.next_int(3);

    let value = if p_type == 4 {
        let mut value = 0;
        loop {
            let continue_bit = iter.next().unwrap();
            value = (value << 4) + iter.next_int(4);
            if continue_bit == 0 {
                break;
            }
        }
        PacketValue::Literal(value)
    } else {
        let length_id = iter.next().unwrap();
        let packets = match length_id {
            0 => {
                let mut packets = Vec::new();
                let num = iter.next_int(15);
                let start_index = iter.counter;
                loop {
                    packets.push(parse(iter));
                    if iter.counter >= start_index + num {
                        break;
                    }
                }
                packets
            }
            1 => {
                let num = iter.next_int(11);
                (0..num).map(|_| parse(iter)).collect()
            }
            _ => unreachable!(),
        };
        PacketValue::SubPackets(packets)
    };
    Packet {
        p_version,
        p_type,
        value,
    }
}

pub fn run(input: &str) -> usize {
    let mut iter = HexStringIter::new(input);
    let packet = parse(&mut iter);
    packet.version_sum()
}
