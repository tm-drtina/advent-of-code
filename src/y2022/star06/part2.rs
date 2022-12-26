use anyhow::{anyhow, Result};

pub fn run(input: &str) -> Result<usize> {
    let bytes = input.as_bytes();
    (0..bytes.len() - 14)
        .find(|&start_index| {
            let mut seen = [false; 26];
            for &b in &bytes[start_index..(start_index + 14)] {
                let b = (b - b'a') as usize;
                if seen[b] {
                    return false;
                }
                seen[b] = true;
            }
            true
        })
        .map(|x| x + 14)
        .ok_or_else(|| anyhow!("Pattern not found"))
}
