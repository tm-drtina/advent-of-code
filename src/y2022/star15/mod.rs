mod common;
pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 5832528, include_str!("input.txt"), 2_000_000),
        (part1_sanity1, 26, include_str!("input.sanity1.txt"), 10),
    );

    aoc_test_suite!(
        super::part2::run,
        (
            part2_main,
            13360899249595,
            include_str!("input.txt"),
            4_000_000
        ),
        (
            part2_sanity1,
            56000011,
            include_str!("input.sanity1.txt"),
            20
        ),
    );
}
