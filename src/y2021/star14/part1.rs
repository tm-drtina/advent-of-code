use std::{collections::HashMap, iter::once};

use itertools::Itertools;

pub(super) fn step(template: &[char], rules: &HashMap<(char, char), char>) -> Vec<char> {
    template
        .iter()
        .tuple_windows()
        .flat_map(|(left, right)| once(*left).chain(once(*rules.get(&(*left, *right)).unwrap())))
        .chain(once(*template.last().unwrap()))
        .collect()
}

pub(super) fn parse(input: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let mut lines = input.lines();
    let template = lines.next().unwrap().chars().collect();
    lines.next().unwrap();

    let mut rules = HashMap::new();

    for line in lines {
        let (left, right) = line.split_once(" -> ").unwrap();
        let mut left = left.chars();
        let ch1 = left.next().unwrap();
        let ch2 = left.next().unwrap();
        let ch3 = right.chars().next().unwrap();
        rules.insert((ch1, ch2), ch3);
    }

    (template, rules)
}

pub(super) fn score(mut template: Vec<char>) -> usize {
    template.sort_unstable();

    let ((_min_char, min_count), (_max_char, max_count)) = template
        .clone()
        .into_iter()
        .group_by(|x| *x)
        .into_iter()
        .map(|(ch, l)| (ch, l.into_iter().count()))
        .minmax_by_key(|(_ch, len)| *len)
        .into_option()
        .unwrap();

    max_count - min_count
}

pub fn run(input: &str) -> usize {
    let (mut template, rules) = parse(input);

    for _ in 0..10 {
        template = step(&template, &rules);
    }

    score(template)
}
