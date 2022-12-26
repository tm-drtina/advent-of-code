mod common;
pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 1_477_771, include_str!("input.txt")),
        (part1_sanity1, 95437, include_str!("input.sanity1.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 3_579_501, include_str!("input.txt")),
        (part2_sanity1, 24_933_642, include_str!("input.sanity1.txt")),
    );
}
