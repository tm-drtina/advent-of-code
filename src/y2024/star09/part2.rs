use std::collections::VecDeque;

use anyhow::Result;

use super::part1::Field;

struct CompressionIterator {
    data: VecDeque<Field>,
    current: Field,
}

impl CompressionIterator {
    pub fn new(data: VecDeque<Field>) -> Self {
        Self {
            data,
            current: Field::Empty(0),
        }
    }
}

impl Iterator for CompressionIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.current {
            Field::Data { value, length } if *length > 0 => {
                *length -= 1;
                return Some(*value);
            }
            _ => {}
        }

        while self.data.front()?.length() == 0 {
            self.data.pop_front();
        }

        let empty_length = match self.data.front()? {
            Field::Data { .. } => {
                self.current = self.data.pop_front()?;
                return self.next();
            }
            Field::Empty(length) => *length,
        };

        if let Some(new_field) =
            self.data.iter_mut().rev().find(
                |field| matches!(field, Field::Data { length, .. } if *length <= empty_length),
            )
        {
            let field = std::mem::replace(new_field, Field::Empty(new_field.length()));
            self.current = field;

            match self.data.front_mut() {
                Some(Field::Empty(length)) => {
                    *length -= self.current.length();
                }
                _ => unreachable!(),
            }
            self.next()
        } else {
            match self.data.front_mut() {
                Some(Field::Empty(length)) => {
                    *length -= 1;
                    Some(0)
                }
                _ => unreachable!(),
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
