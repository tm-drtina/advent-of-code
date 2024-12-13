use anyhow::Result;

use super::part1::Equation;

impl Equation {
    fn append(a: usize, b: usize) -> usize {
        a * 10_usize.pow(b.ilog10() + 1) + b
    }

    fn is_possible_part2(&self) -> bool {
        let mut operands = self.operands.iter();
        let mut partial_results =
            vec![*operands.next().expect("At least one operand must be given")];
        for operand in operands {
            let mut new_partial_results = Vec::new();
            for partial_result in partial_results {
                for new_result in [
                    partial_result * operand,
                    partial_result + operand,
                    Self::append(partial_result, *operand),
                ] {
                    match new_result.cmp(&self.result) {
                        std::cmp::Ordering::Less => new_partial_results.push(new_result),
                        std::cmp::Ordering::Equal => return true,
                        std::cmp::Ordering::Greater => {}
                    }
                }
            }
            if new_partial_results.is_empty() {
                return false;
            }
            partial_results = new_partial_results;
        }
        false
    }
}

pub fn run(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(str::parse::<Equation>)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .filter(Equation::is_possible_part2)
        .map(|eq| eq.result)
        .sum())
}
