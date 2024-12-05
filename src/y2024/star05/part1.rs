use std::collections::{BTreeMap, BTreeSet};
use std::num::ParseIntError;
use std::str::FromStr;

use anyhow::{Context, Result, anyhow};

pub(super) struct Rules(pub(super) BTreeMap<usize, Vec<usize>>);

impl Rules {
    pub(super) fn from_lines<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Result<Self> {
        let mut rules: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for line in lines {
            if line.is_empty() {
                break;
            }
            let (l, r) = line
                .split_once('|')
                .ok_or(anyhow!("invalid format of rule"))?;
            let l = l.parse().context("Failed to parse left number of rule")?;
            let r = r.parse().context("Failed to parse right number of rule")?;

            rules.entry(r).or_default().push(l);
        }

        Ok(Self(rules))
    }

    pub(super) fn get_previous(&self, num: usize) -> &[usize] {
        self.0.get(&num).map_or(&[], |v| &**v)
    }
}

pub(super) struct Book(pub(super) Vec<usize>);
impl Book {
    pub(super) fn from_str(line: &str) -> Result<Self> {
        Ok(Self(
            line.split(',')
                .map(str::parse)
                .collect::<Result<Vec<usize>, ParseIntError>>()
                .context("Invalid book format")?,
        ))
    }

    pub(super) fn middle_page(self) -> usize {
        self.0[self.0.len() / 2]
    }

    pub(super) fn is_valid(&self, rules: &Rules) -> bool {
        let mut banned = BTreeSet::<usize>::new();
        for &num in &self.0 {
            if banned.contains(&num) {
                return false;
            }
            banned.extend(rules.get_previous(num));
        }
        true
    }
}

pub(super) struct Puzzle {
    pub(super) rules: Rules,
    pub(super) books: Vec<Book>,
}

impl FromStr for Puzzle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();

        let rules = Rules::from_lines(&mut lines)?;

        let books = lines.map(Book::from_str).collect::<Result<Vec<_>>>()?;

        Ok(Self { rules, books })
    }
}

pub fn run(input: &str) -> Result<usize> {
    let puzzle = input.parse::<Puzzle>()?;

    Ok(puzzle
        .books
        .into_iter()
        .filter(|b| b.is_valid(&puzzle.rules))
        .map(Book::middle_page)
        .sum())
}
