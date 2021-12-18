use itertools::Itertools;

use super::part1::Number;

pub fn run(input: &str) -> usize {
    let numbers: Vec<_> = input
        .lines()
        .map(|line| Number::from_str(&mut line.chars()))
        .collect();

    numbers
        .iter()
        .tuple_combinations()
        .map(|(n1, n2)| {
            let mut res1 = n1.clone().add(n2.clone());
            res1.reduce();
            let m1 = res1.magnitude();

            let mut res2 = n2.clone().add(n1.clone());
            res2.reduce();
            let m2 = res2.magnitude();

            m1.max(m2)
        })
        .max()
        .unwrap()
}
