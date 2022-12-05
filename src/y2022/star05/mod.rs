mod common;
pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, "CFFHVVHNC", include_str!("input.txt")),
        (part1_sanity1, "CMZ", include_str!("input.sanity1.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, "FSZWBPTBG", include_str!("input.txt")),
        (part2_sanity1, "MCD", include_str!("input.sanity1.txt")),
    );
}
