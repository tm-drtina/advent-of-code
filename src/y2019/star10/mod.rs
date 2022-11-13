pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 286, include_str!("input.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 504, include_str!("input.txt")),
        (part2_sanity1, 802, include_str!("input.sanity1.txt")),
    );
}
