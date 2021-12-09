pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 1645, include_str!("input.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 35292, include_str!("input.txt")),
        (part2_sanity, 286, include_str!("input.sanity.txt")),
    );
}
