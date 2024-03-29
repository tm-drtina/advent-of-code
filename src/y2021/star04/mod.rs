pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 33348, include_str!("input.txt")),
        (part1_sanity, 4512, include_str!("input.sanity.txt"))
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 8112, include_str!("input.txt")),
        (part2_sanity, 1924, include_str!("input.sanity.txt"))
    );
}
