pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 798147, include_str!("input.txt")),
        (part1_sanity, 739785, include_str!("input.sanity.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 809953813657517, include_str!("input.txt")),
        (
            part2_sanity,
            444356092776315,
            include_str!("input.sanity.txt")
        ),
    );
}
