pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 264, include_str!("input.txt")),
        (part1_example1, 0, include_str!("input.example1.txt")),
        (part1_example2, 26, include_str!("input.example2.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 1_063_760, include_str!("input.txt")),
        (part2_example1, 5353, include_str!("input.example1.txt")),
        (part2_example2, 61229, include_str!("input.example2.txt")),
    );
}
