pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 1625, include_str!("input.txt")),
        (part1_example1, 7, include_str!("input.example1.txt")),
        (part1_example2, 5, include_str!("input.example2.txt")),
        (part1_example3, 6, include_str!("input.example3.txt")),
        (part1_example4, 10, include_str!("input.example4.txt")),
        (part1_example5, 11, include_str!("input.example5.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 2250, include_str!("input.txt")),
        (part2_example1, 19, include_str!("input.example1.txt")),
        (part2_example2, 23, include_str!("input.example2.txt")),
        (part2_example3, 23, include_str!("input.example3.txt")),
        (part2_example4, 29, include_str!("input.example4.txt")),
        (part2_example5, 26, include_str!("input.example5.txt")),
    );
}
