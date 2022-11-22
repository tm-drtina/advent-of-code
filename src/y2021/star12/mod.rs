pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 4411, include_str!("input.txt")),
        (part1_sanity1, 10, include_str!("input.sanity1.txt")),
        (part1_sanity2, 19, include_str!("input.sanity2.txt")),
        (part1_sanity3, 226, include_str!("input.sanity3.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 136767, include_str!("input.txt")),
        (part2_sanity1, 36, include_str!("input.sanity1.txt")),
        (part2_sanity2, 103, include_str!("input.sanity2.txt")),
        (part2_sanity3, 3509, include_str!("input.sanity3.txt")),
    );
}
