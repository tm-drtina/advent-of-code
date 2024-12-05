use std::collections::{BTreeMap, BTreeSet};
use std::num::ParseIntError;
use std::str::FromStr;

use anyhow::{Context, Result, anyhow};

struct Rules(BTreeMap<usize, Vec<usize>>);

impl Rules {
    fn from_lines<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Result<Self> {
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

    fn get_previous(&self, num: usize) -> &[usize] {
        self.0.get(&num).map_or(&[], |v| &**v)
    }
}

struct Book(Vec<usize>);
impl Book {
    fn from_str(line: &str) -> Result<Self> {
        Ok(Self(
            line.split(',')
                .map(str::parse)
                .collect::<Result<Vec<usize>, ParseIntError>>()
                .context("Invalid book format")?,
        ))
    }

    fn middle_page(self) -> usize {
        self.0[self.0.len() / 2]
    }

    fn is_valid(&self, rules: &Rules) -> bool {
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

struct Puzzle {
    rules: Rules,
    books: Vec<Book>,
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
