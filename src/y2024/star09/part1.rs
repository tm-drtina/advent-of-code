use std::collections::VecDeque;

use anyhow::Result;

#[derive(Debug, Clone, Copy)]
pub enum Field {
    Data { value: usize, length: usize },
    Empty(usize),
}
impl From<(usize, char)> for Field {
    fn from((index, length): (usize, char)) -> Self {
        let length = (length as u8 - b'0') as usize;
        if index & 1 > 0 {
            Self::Empty(length)
        } else {
            Self::Data {
                value: index / 2,
                length,
            }
        }
    }
}
impl Field {
    pub fn length(&self) -> usize {
        match self {
            Field::Data { length, .. } | Field::Empty(length) => *length,
        }
    }
}

struct CompressionIterator {
    data: VecDeque<Field>,
}

impl CompressionIterator {
    pub fn new(data: VecDeque<Field>) -> Self {
        Self { data }
    }
}

impl Iterator for CompressionIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.data.front()?.length() == 0 {
            self.data.pop_front();
        }

        let first = self.data.front_mut()?;
        match first {
            Field::Data { value, length } => {
                *length -= 1;
                Some(*value)
            }
            Field::Empty(length) => {
                *length -= 1;
                while matches!(
                    self.data.back()?,
                    Field::Empty(_) | Field::Data { length: 0, .. }
                ) {
                    self.data.pop_back();
                }

                let last = self.data.back_mut()?;
                match last {
                    Field::Data { value, length } => {
                        *length -= 1;
                        Some(*value)
                    }
                    Field::Empty(_) => unreachable!("Filtered in previous loop"),
                }
            }
        }
    }
}

pub fn run(input: &'static str) -> Result<usize> {
    let data = input
        .chars()
        .enumerate()
        .map(Field::from)
        .collect::<VecDeque<_>>();

    Ok(CompressionIterator::new(data)
        .enumerate()
        .map(|(i, value)| i * value)
        .sum())
}
