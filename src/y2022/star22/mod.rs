mod common;
pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 89224, include_str!("input.txt")),
        (part1_example1, 6032, include_str!("input.example1.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 136_182, include_str!("input.txt")),
        (part2_example1, 5031, include_str!("input.example1.txt")),
    );
}
