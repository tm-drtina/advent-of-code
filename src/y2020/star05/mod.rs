pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 822, include_str!("input.txt")),
        (part1_sanity1, 567, "BFFFBBFRRR"),
        (part1_sanity2, 119, "FFFBBBFRRR"),
        (part1_sanity3, 820, "BBFFBBFRLL"),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 705, include_str!("input.txt")),
    );
}
