pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 12_918_250_417_632, include_str!("input.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 171_259_538_712_010, include_str!("input.txt")),
    );
}
