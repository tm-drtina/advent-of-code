pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 213, include_str!("input.txt")),
        (part1_example1, 2, include_str!("input.example1.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 147, include_str!("input.txt")),
        (part2_example2, 0, include_str!("input.example2.txt")),
        (part2_example3, 4, include_str!("input.example3.txt")),
    );
}
