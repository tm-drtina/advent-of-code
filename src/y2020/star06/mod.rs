pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 6504, include_str!("input.txt")),
        (part1_example, 11, include_str!("input.example.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 3351, include_str!("input.txt")),
        (part2_example, 6, include_str!("input.example.txt")),
    );
}
