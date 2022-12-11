use anyhow::Result;

use super::common::Monkeys;

pub fn run(input: &str) -> Result<usize> {
    let mut monkeys: Monkeys<false> = input.parse()?;
    for _ in 0..10_000 {
        monkeys.round();
    }
    Ok(monkeys.level())
}
