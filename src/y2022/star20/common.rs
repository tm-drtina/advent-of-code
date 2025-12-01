use std::str::FromStr;

use anyhow::{Error, Result, anyhow};

#[derive(Debug, Clone, Copy)]
struct Value {
    value: i64,
    position: usize,
}

#[derive(Debug)]
pub(super) struct Puzzle(Vec<Value>);

impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(
            s.lines()
                .enumerate()
                .map(|(index, s)| {
                    Ok(Value {
                        value: s.parse()?,
                        position: index,
                    })
                })
                .collect::<Result<_>>()?,
        ))
    }
}

impl Puzzle {
    pub(super) fn mix(&mut self) {
        for i in 0..self.0.len() {
            let Value { value, position } = self.0[i];

            let v = value % (self.0.len() - 1) as i64;

            #[allow(clippy::comparison_chain)]
            if v == 0 {
                // nothing to do
            } else if v > 0 {
                let v = v as usize;
                if v + position >= self.0.len() {
                    // overflow case
                    let new_pos = v + 1 + position - self.0.len();
                    self.0
                        .iter_mut()
                        .filter(|v| v.position < position && v.position >= new_pos)
                        .for_each(|v| v.position += 1);
                    self.0[i].position = new_pos;
                } else {
                    let new_pos = position + v;
                    self.0
                        .iter_mut()
                        .filter(|v| v.position > position && v.position <= new_pos)
                        .for_each(|v| v.position -= 1);
                    self.0[i].position = new_pos;
                }
            } else {
                let v = (-v) as usize;
                if position >= v {
                    let new_pos = position - v;
                    self.0
                        .iter_mut()
                        .filter(|v| v.position < position && v.position >= new_pos)
                        .for_each(|v| v.position += 1);
                    self.0[i].position = new_pos;
                } else {
                    // overflow
                    let new_pos = self.0.len() + position - v - 1;
                    self.0
                        .iter_mut()
                        .filter(|v| v.position > position && v.position <= new_pos)
                        .for_each(|v| v.position -= 1);
                    self.0[i].position = new_pos;
                }
            }
        }
    }

    pub(super) fn apply_decryption_key(&mut self, key: i64) {
        self.0.iter_mut().for_each(|v| v.value *= key);
    }

    pub(super) fn groove(self) -> Result<i64> {
        let mut values = self.0;
        values.sort_unstable_by_key(|v| v.position);
        let start = values
            .iter()
            .position(|v| v.value == 0)
            .ok_or_else(|| anyhow!("Zero value not found"))?;
        Ok(values[(start + 1000) % values.len()].value
            + values[(start + 2000) % values.len()].value
            + values[(start + 3000) % values.len()].value)
    }
}
