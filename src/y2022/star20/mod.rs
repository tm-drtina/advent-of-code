mod common;
pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 8372, include_str!("input.txt")),
        (part1_example1, 3, include_str!("input.example1.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 7_865_110_481_723, include_str!("input.txt")),
        (
            part2_example1,
            1_623_178_306,
            include_str!("input.example1.txt")
        ),
    );
}
