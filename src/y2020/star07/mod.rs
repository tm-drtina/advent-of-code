pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 272, include_str!("input.txt")),
        (part1_example, 4, include_str!("input.example1.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 172_246, include_str!("input.txt")),
        (part2_example, 126, include_str!("input.example2.txt")),
    );
}
