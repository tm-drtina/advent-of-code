pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 4173, include_str!("input.txt")),
        (part1_sanity, 4140, include_str!("input.sanity.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 4706, include_str!("input.txt")),
        (part2_sanity, 3993, include_str!("input.sanity.txt")),
    );
}
