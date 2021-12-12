pub mod part1;
pub mod part2;

#[macro_use]
#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 1749, include_str!("input.txt")),
        (part1_sanity, 1656, include_str!("input.sanity.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 285, include_str!("input.txt")),
        (part2_sanity, 195, include_str!("input.sanity.txt")),
    );
}
