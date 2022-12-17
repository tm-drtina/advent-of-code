mod common;
pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 3117, include_str!("input.txt")),
        (part1_sanity1, 3068, include_str!("input.sanity1.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 1553310011699, include_str!("input.txt")), // >1553310011699
        //(part2_sanity1, 1_514_285_714_288, include_str!("input.sanity1.txt")),
    );
}
