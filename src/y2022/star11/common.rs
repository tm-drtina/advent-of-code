use std::cmp::Reverse;
use std::collections::VecDeque;
use std::str::{FromStr, Lines};

use anyhow::{anyhow, bail, Context, Error, Ok, Result};
use num::integer::lcm;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(usize),
    Mul(usize),
    Pow,
}

impl Operation {
    fn apply(self, lhs: usize) -> usize {
        match self {
            Operation::Add(rhs) => lhs + rhs,
            Operation::Mul(rhs) => lhs * rhs,
            Operation::Pow => lhs * lhs,
        }
    }
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, rhs) = s.split_once(' ').ok_or_else(|| anyhow!("Invalid op format"))?;

        Ok(match op {
            "+" => Operation::Add(rhs.parse()?),
            "*" if rhs == "old" => Operation::Pow,
            "*" => Operation::Mul(rhs.parse()?),
            _ => bail!("Invalid operation"),
        })
    }
}

struct Monkey {
    items: VecDeque<usize>,
    op: Operation,
    divisible: usize,
    true_cond: usize,
    false_cond: usize,
    processed: usize,
}

impl<'a> TryFrom<&mut Lines<'a>> for Monkey {
    type Error = Error;

    fn try_from(lines: &mut Lines<'a>) -> Result<Self, Self::Error> {
        let name = lines.next().ok_or_else(|| anyhow!("Missing name"))?;
        debug_assert!(name.starts_with("Monkey "));

        let items = lines.next().ok_or_else(|| anyhow!("Missing items"))?;
        let items = items
            .strip_prefix("  Starting items: ")
            .ok_or_else(|| anyhow!("Invalid items line"))?;
        let items = items
            .split(", ")
            .map(str::parse::<usize>)
            .collect::<Result<_, _>>()
            .context("Invalid items str")?;

        let op = lines.next().ok_or_else(|| anyhow!("Missing operation"))?;
        let op = op
            .strip_prefix("  Operation: new = old ")
            .ok_or_else(|| anyhow!("Invalid op line"))?;
        let op = op.parse().context("Invalid operation str")?;

        let divisible = lines.next().ok_or_else(|| anyhow!("Missing divisible line"))?;
        let divisible = divisible
            .strip_prefix("  Test: divisible by ")
            .ok_or_else(|| anyhow!("Invalid divisible line"))?;
        let divisible = divisible.parse().context("Invalid divisible str")?;

        let true_cond = lines.next().ok_or_else(|| anyhow!("Missing true cond line"))?;
        let true_cond = true_cond
            .strip_prefix("    If true: throw to monkey ")
            .ok_or_else(|| anyhow!("Invalid true cond line"))?;
        let true_cond = true_cond.parse().context("Invalid true cond str")?;

        let false_cond = lines.next().ok_or_else(|| anyhow!("Missing false cond line"))?;
        let false_cond = false_cond
            .strip_prefix("    If false: throw to monkey ")
            .ok_or_else(|| anyhow!("Invalid false cond line"))?;
        let false_cond = false_cond.parse().context("Invalid false cond str")?;

        Ok(Self {
            items,
            op,
            divisible,
            true_cond,
            false_cond,
            processed: 0,
        })
    }
}

pub(super) struct Monkeys<const SANE: bool> {
    monkeys: Vec<Monkey>,
    lcm: usize,
}

impl<const T: bool> FromStr for Monkeys<T> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let mut monkeys = Vec::<Monkey>::new();
        loop {
            monkeys.push(Monkey::try_from(&mut lines)?);
            if lines.next().is_none() {
                break;
            }
        }

        let lcm = monkeys.iter().fold(1, |acc, m| lcm(acc, m.divisible));

        Ok(Self { monkeys, lcm })
    }
}

impl<const SANE: bool> Monkeys<SANE> {
    pub(super) fn round(&mut self) {
        for index in 0..self.monkeys.len() {
            self.monkeys[index].processed += self.monkeys[index].items.len();
            while let Some(item) = self.monkeys[index].items.pop_front() {
                let mut new_item = self.monkeys[index].op.apply(item);
                if SANE {
                    new_item /= 3;
                } else {
                    new_item %= self.lcm;
                }
                let target_index = if new_item % self.monkeys[index].divisible == 0 {
                    self.monkeys[index].true_cond
                } else {
                    self.monkeys[index].false_cond
                };
                self.monkeys[target_index].items.push_back(new_item);
            }
        }
    }

    pub(super) fn level(&self) -> usize {
        let mut processed: Vec<_> = self.monkeys.iter().map(|m| Reverse(m.processed)).collect();
        processed.sort_unstable();
        processed[0].0 * processed[1].0
    }
}
