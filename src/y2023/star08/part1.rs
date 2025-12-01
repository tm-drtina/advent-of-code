use std::collections::HashMap;
use std::iter::Cycle;
use std::str::FromStr;

use anyhow::{Result, anyhow, bail};

pub(super) type Loc = u32;

pub(super) const fn encode(s: &str) -> Loc {
    let b = s.as_bytes();

    ((b[0] as Loc) << 16) + ((b[1] as Loc) << 8) + (b[2] as Loc)
}

const START: Loc = encode("AAA");
const END: Loc = encode("ZZZ");

#[derive(Debug, Clone, Copy)]
pub(super) enum Dir {
    Left,
    Right,
}

pub(super) struct Puzzle {
    pub(super) seq: Vec<Dir>,
    pub(super) transitions: HashMap<Loc, (Loc, Loc)>,
}

impl FromStr for Puzzle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();
        let seq = lines
            .next()
            .ok_or(anyhow!("Missing first line"))?
            .bytes()
            .map(|b| match b {
                b'L' => Ok(Dir::Left),
                b'R' => Ok(Dir::Right),
                _ => bail!("Wrong direction"),
            })
            .collect::<Result<_>>()?;
        lines.next();

        let transitions = lines
            .map(|l| {
                let (from, to) = l.split_once(" = (").ok_or(anyhow!("wrong format"))?;
                let to = to.strip_suffix(')').ok_or(anyhow!("missing end suffix"))?;
                let (left, right) = to
                    .split_once(", ")
                    .ok_or(anyhow!("Failed to split left and right"))?;
                Ok((encode(from), (encode(left), encode(right))))
            })
            .collect::<Result<_>>()?;

        Ok(Self { seq, transitions })
    }
}

impl Puzzle {
    pub(super) fn goal_iter(&self, start: Loc) -> GoalIter<'_> {
        GoalIter {
            pos: start,
            transitions: &self.transitions,
            seq: self.seq.iter().cycle(),
        }
    }
}

pub(super) struct GoalIter<'a> {
    pos: Loc,
    transitions: &'a HashMap<Loc, (Loc, Loc)>,
    seq: Cycle<std::slice::Iter<'a, Dir>>,
}

impl Iterator for GoalIter<'_> {
    type Item = Loc;

    fn next(&mut self) -> Option<Self::Item> {
        let dir = self.seq.next().unwrap();
        let transition = *self.transitions.get(&self.pos).unwrap();
        self.pos = match dir {
            Dir::Left => transition.0,
            Dir::Right => transition.1,
        };
        Some(self.pos)
    }
}

pub fn run(input: &str) -> Result<usize> {
    Ok(input
        .parse::<Puzzle>()?
        .goal_iter(START)
        .take_while(|p| *p != END)
        .count()
        + 1)
}
