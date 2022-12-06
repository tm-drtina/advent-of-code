pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 1625, include_str!("input.txt")),
        (part1_sanity1, 7, include_str!("input.sanity1.txt")),
        (part1_sanity2, 5, include_str!("input.sanity2.txt")),
        (part1_sanity3, 6, include_str!("input.sanity3.txt")),
        (part1_sanity4, 10, include_str!("input.sanity4.txt")),
        (part1_sanity5, 11, include_str!("input.sanity5.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 2250, include_str!("input.txt")),
        (part2_sanity1, 19, include_str!("input.sanity1.txt")),
        (part2_sanity2, 23, include_str!("input.sanity2.txt")),
        (part2_sanity3, 23, include_str!("input.sanity3.txt")),
        (part2_sanity4, 29, include_str!("input.sanity4.txt")),
        (part2_sanity5, 26, include_str!("input.sanity5.txt")),
    );
}
