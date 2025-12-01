pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 193607, include_str!("input.txt")),
        (part1_example1, 55312, include_str!("input.example1.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 229557103025807, include_str!("input.txt")),
    );
}
