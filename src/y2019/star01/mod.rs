pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 3_425_624, include_str!("input.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 5_135_558, include_str!("input.txt")),
    );
}
