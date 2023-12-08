use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::successors;

use anyhow::Result;

use super::part1::{GoalIter, Loc, Puzzle};

type Step = (Loc, usize);

impl GoalIter<'_> {
    fn find_end_pos(self) -> Step {
        for (i, pos) in self.enumerate() {
            if pos & 255 == b'Z' as Loc {
                return (pos, i + 1);
            }
        }
        unreachable!()
    }
}

struct MergedIter<T>(T, T);

impl<T: Iterator<Item = Step>> Iterator for MergedIter<T> {
    type Item = Step;

    fn next(&mut self) -> Option<Self::Item> {
        let (mut pos1, mut steps1) = self.0.next()?;
        let (_, mut steps2) = self.1.next()?;
        loop {
            match steps1.cmp(&steps2) {
                Ordering::Less => {
                    let (a, b) = self.0.next()?;
                    pos1 = a;
                    steps1 += b;
                }
                Ordering::Equal => {
                    break Some((pos1, steps1));
                }
                Ordering::Greater => {
                    steps2 += self.1.next()?.1;
                }
            }
        }
    }
}

struct State {
    start: Loc,
    steps: usize,
    increments: Vec<Step>,
    cycle: Vec<Step>,
}

impl State {
    fn compute_increment_cycle(it: impl Iterator<Item = Step>) -> (Vec<Step>, Vec<Step>) {
        let mut transitions = Vec::<Step>::new();
        for a in it {
            if let Some(pos) = transitions.iter().position(|b| a.0 == b.0) {
                let increments = transitions[..=pos].to_vec();
                let mut cycle = transitions[(pos + 1)..].to_vec();
                cycle.push(a);
                return (increments, cycle);
            }
            transitions.push(a);
        }
        unreachable!()
    }

    fn new(start: Loc, steps: usize, transitions: &HashMap<Loc, (Loc, usize)>) -> Self {
        let (increments, cycle) =
            Self::compute_increment_cycle(successors(transitions.get(&start).copied(), |prev| {
                transitions.get(&prev.0).copied()
            }));
        Self {
            start,
            steps,
            increments,
            cycle,
        }
    }

    fn merge(mut self, mut other: Self) -> Self {
        let mut it1 = self
            .increments
            .into_iter()
            .chain(self.cycle.into_iter().cycle());
        let mut it2 = other
            .increments
            .into_iter()
            .chain(other.cycle.into_iter().cycle());
        loop {
            match self.steps.cmp(&other.steps) {
                Ordering::Less => {
                    let (new_pos, steps) = it1.next().unwrap();
                    self.start = new_pos;
                    self.steps += steps;
                }
                Ordering::Equal => break,
                Ordering::Greater => {
                    let (new_pos, steps) = it2.next().unwrap();
                    other.start = new_pos;
                    other.steps += steps;
                }
            }
        }

        let (increments, cycle) = Self::compute_increment_cycle(MergedIter(it1, it2));

        Self {
            start: self.start,
            steps: self.steps,
            increments,
            cycle,
        }
    }
}

impl Puzzle {
    fn solve_multi_goal(self) -> usize {
        let simple_transitions: HashMap<Loc, (Loc, usize)> = self
            .transitions
            .keys()
            .copied()
            .filter(|p| p & 255 == b'Z' as Loc)
            .map(|p| (p, self.goal_iter(p).find_end_pos()))
            .collect();

        self.transitions
            .keys()
            .copied()
            .filter(|p| p & 255 == b'A' as Loc)
            .map(|p| {
                let (pos, steps) = self.goal_iter(p).find_end_pos();
                State::new(pos, steps, &simple_transitions)
            })
            .reduce(State::merge)
            .unwrap()
            .steps
    }
}

pub fn run(input: &str) -> Result<usize> {
    Ok(input.parse::<Puzzle>()?.solve_multi_goal())
}
