pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 26869, include_str!("input.txt")),
        (part1_sanity1, 71, include_str!("input.sanity1.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 855275529001, include_str!("input.txt")),
        (part2_sanity2, 12, include_str!("input.sanity2.txt")),
        (part2_sanity3, 11, include_str!("input.sanity3.txt")),
        (part2_sanity4, 13, include_str!("input.sanity4.txt")),
    );
}
