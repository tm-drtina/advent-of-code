pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 351_188, include_str!("input.txt")),
        (part1_sanity, 5934, include_str!("input.sanity.txt"))
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 1_595_779_846_729, include_str!("input.txt")),
        (
            part2_sanity,
            26_984_457_539,
            include_str!("input.sanity.txt")
        )
    );
}
