pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 213, include_str!("input.txt")),
        (part1_sanity1, 2, include_str!("input.sanity1.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 147, include_str!("input.txt")),
        (part2_sanity2, 0, include_str!("input.sanity2.txt")),
        (part2_sanity3, 4, include_str!("input.sanity3.txt")),
    );
}
