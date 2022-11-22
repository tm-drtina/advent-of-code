use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(super) struct Point(pub usize, pub usize);

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        Ok(Self(x.parse().unwrap(), y.parse().unwrap()))
    }
}

impl Point {
    pub(super) fn fold(self, fold: Fold) -> Self {
        match fold {
            Fold::X(x) if self.0 > x => Self(2 * x - self.0, self.1),
            Fold::Y(y) if self.1 > y => Self(self.0, 2 * y - self.1),
            _ => self,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) enum Fold {
    X(usize),
    Y(usize),
}

pub(super) fn parse(input: &str) -> (HashSet<Point>, Vec<Fold>) {
    let mut points = HashSet::new();

    let mut lines = input.lines();

    loop {
        let line = lines.next();
        if line.is_none() || line.unwrap().is_empty() {
            break;
        }
        points.insert(line.unwrap().parse().unwrap());
    }

    let mut folds = Vec::new();
    for line in lines {
        if let Some(num) = line.strip_prefix("fold along x=") {
            folds.push(Fold::X(num.parse().unwrap()));
        } else if let Some(num) = line.strip_prefix("fold along y=") {
            folds.push(Fold::Y(num.parse().unwrap()));
        } else {
            panic!("Unrecognized line.");
        }
    }

    (points, folds)
}

pub fn run(input: &str) -> usize {
    let (mut points, folds) = parse(input);
    let fold = *folds.first().unwrap();
    points = points.into_iter().map(|pt| pt.fold(fold)).collect();
    points.len()
}
