pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 6870, include_str!("input.txt")),
        (part1_example1, 4, include_str!("input.example1.txt")),
        (
            part1_example1_clean,
            4,
            include_str!("input.example1_clean.txt")
        ),
        (part1_example2, 8, include_str!("input.example2.txt")),
        (
            part1_example2_clean,
            8,
            include_str!("input.example2_clean.txt")
        ),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 287, include_str!("input.txt")),
        (part2_example3, 4, include_str!("input.example3.txt")),
        (part2_example4, 8, include_str!("input.example4.txt")),
        (part2_example5, 10, include_str!("input.example5.txt")),
    );
}
