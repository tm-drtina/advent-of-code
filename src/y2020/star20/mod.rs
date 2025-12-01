pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 16_937_516_456_219, include_str!("input.txt")),
        (
            part1_example,
            20_899_048_083_289,
            include_str!("input.example.txt")
        ),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 1858, include_str!("input.txt")),
        (part2_example, 273, include_str!("input.example.txt")),
    );
}
