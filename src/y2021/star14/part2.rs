use std::collections::HashMap;

use itertools::Itertools;

use super::part1::parse;

const STEPS: usize = 40;
type Rules = HashMap<(char, char), char>;

type Counts = HashMap<char, usize>;

fn combine(left: &Counts, right: &Counts, mid: Option<char>) -> Counts {
    let mut res = HashMap::new();

    for (ch, count) in left {
        *res.entry(*ch).or_default() += *count;
    }
    for (ch, count) in right {
        *res.entry(*ch).or_default() += *count;
    }
    if let Some(ch) = mid {
        *res.entry(ch).or_default() += 1;
    }

    res
}

fn dfs(
    left: char,
    right: char,
    depth: usize,
    precomputed: &mut [HashMap<(char, char), Counts>],
    rules: &Rules,
) -> Counts {
    let mid = *rules.get(&(left, right)).unwrap();

    if depth == STEPS - 1 {
        let mut counts = HashMap::new();
        counts.insert(mid, 1);
        counts
    } else {
        let cached = precomputed[depth].get(&(left, right)).cloned();
        if let Some(cached) = cached {
            cached
        } else {
            let left_res = dfs(left, mid, depth + 1, precomputed, rules);
            let right_res = dfs(mid, right, depth + 1, precomputed, rules);
            let comb = combine(&left_res, &right_res, Some(mid));
            precomputed[depth].insert((left, right), comb.clone());
            comb
        }
    }
}

pub fn run(input: &str) -> usize {
    let (template, rules) = parse(input);

    let mut precomputed = vec![HashMap::new(); STEPS - 1];

    let mut res = HashMap::new();
    res.insert(*template.first().unwrap(), 1);

    for (left, right) in template.iter().tuple_windows() {
        res = combine(
            &res,
            &dfs(*left, *right, 0, &mut precomputed, &rules),
            Some(*right),
        );
    }

    let (min, max) = res.values().minmax().into_option().unwrap();
    *max - *min
}
