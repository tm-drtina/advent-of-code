mod common;
pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 3117, include_str!("input.txt")),
        (part1_example1, 3068, include_str!("input.example1.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 1_553_314_121_019, include_str!("input.txt")),
        (
            part2_example1,
            1_514_285_714_288,
            include_str!("input.example1.txt")
        ),
    );
}
