pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 4411, include_str!("input.txt")),
        (part1_example1, 10, include_str!("input.example1.txt")),
        (part1_example2, 19, include_str!("input.example2.txt")),
        (part1_example3, 226, include_str!("input.example3.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 136_767, include_str!("input.txt")),
        (part2_example1, 36, include_str!("input.example1.txt")),
        (part2_example2, 103, include_str!("input.example2.txt")),
        (part2_example3, 3509, include_str!("input.example3.txt")),
    );
}
