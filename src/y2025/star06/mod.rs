pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 4722948564882, include_str!("input.txt")),
        (part1_example1, 4277556, include_str!("input.example1.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 9581313737063, include_str!("input.txt")),
        (part2_example1, 3263827, include_str!("input.example1.txt")),
    );
}
