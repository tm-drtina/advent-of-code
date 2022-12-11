use anyhow::Result;

use super::common::Monkeys;

pub fn run(input: &str) -> Result<usize> {
    let mut monkeys: Monkeys<true> = input.parse()?;
    for _ in 0..20 {
        monkeys.round();
    }
    Ok(monkeys.level())
}
