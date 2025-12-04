fn joltage_part(bytes: &[u8], res: &mut u64, len: usize) {
    let num_b = *bytes[..bytes.len() - (len - 1)].iter().max().unwrap();
    let num = (num_b - b'0') as u64;
    *res = *res * 10 + num;
    if len > 1 {
        let pos = bytes.iter().position(|x| *x == num_b).unwrap();
        joltage_part(&bytes[pos + 1..], res, len - 1);
    }
}

fn joltage(line: &str) -> u64 {
    let mut res = 0;
    joltage_part(line.as_bytes(), &mut res, 12);
    res
}

pub fn run(input: &str) -> u64 {
    input.lines().map(joltage).sum()
}
