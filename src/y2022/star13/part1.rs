use super::common::Packet;

pub fn run(input: &str) -> usize {
    let mut lines = input.lines();
    let mut res = 0;
    let mut index = 1;
    loop {
        let (Some(left), Some(right)) = (lines.next(), lines.next()) else {
            break;
        };
        lines.next();
        let left = Packet::from(&mut left.as_bytes());
        let right = Packet::from(&mut right.as_bytes());

        if left < right {
            res += index;
        }

        index += 1;
    }

    res
}
