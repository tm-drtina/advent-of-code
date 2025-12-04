fn joltage(line: &str) -> u32 {
    let bytes = line.as_bytes();
    let first = *bytes[..bytes.len() - 1].iter().max().unwrap();
    let first_pos = bytes.iter().position(|x| *x == first).unwrap();
    let second = *bytes[first_pos + 1..].iter().max().unwrap();

    (10 * (first - b'0') + (second - b'0')) as u32
}

pub fn run(input: &str) -> u32 {
    input.lines().map(joltage).sum()
}
