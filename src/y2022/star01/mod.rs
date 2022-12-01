pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 68802, include_str!("input.txt")),
        (part1_sanity1, 24000, include_str!("input.sanity1.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 205370, include_str!("input.txt")),
        (part2_sanity1, 45000, include_str!("input.sanity1.txt")),
    );
}
