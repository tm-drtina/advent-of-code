mod common;
pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 118_674, include_str!("input.txt")),
        (part1_sanity1, 10605, include_str!("input.sanity1.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 32_333_418_600, include_str!("input.txt")),
        (
            part2_sanity1,
            2_713_310_158,
            include_str!("input.sanity1.txt")
        ),
    );
}
