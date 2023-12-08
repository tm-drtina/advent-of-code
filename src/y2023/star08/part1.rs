use std::collections::HashMap;
use std::iter::Cycle;
use std::str::FromStr;

use anyhow::{anyhow, bail, Result};

type Loc = u32;

const fn encode(s: &str) -> Loc {
    let b = s.as_bytes();

    ((b[0] as Loc) << 16) + ((b[1] as Loc) << 8) + (b[2] as Loc)
}

const START: Loc = encode("AAA");
const END: Loc = encode("ZZZ");

#[derive(Debug, Clone, Copy)]
enum Dir {
    Left,
    Right,
}

struct Puzzle {
    seq: Vec<Dir>,
    transitions: HashMap<Loc, (Loc, Loc)>,
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
    fn into_goal_iter(self) -> GoalIter {
        GoalIter {
            pos: START,
            transitions: self.transitions,
            seq: self.seq.into_iter().cycle(),
        }
    }
}

struct GoalIter {
    pos: Loc,
    transitions: HashMap<Loc, (Loc, Loc)>,
    seq: Cycle<<Vec<Dir> as IntoIterator>::IntoIter>,
}

impl Iterator for GoalIter {
    type Item = Loc;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == END {
            return None;
        }
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
    Ok(input.parse::<Puzzle>()?.into_goal_iter().count())
}
