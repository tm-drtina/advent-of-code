pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 298586, include_str!("input.txt")),
        (part1_sanity1, 43210, include_str!("input.sanity1.txt")),
        (part1_sanity2, 54321, include_str!("input.sanity2.txt")),
        (part1_sanity3, 65210, include_str!("input.sanity3.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 9246095, include_str!("input.txt")),
        (part2_sanity4, 139629729, include_str!("input.sanity4.txt")),
        (part2_sanity5, 18216, include_str!("input.sanity5.txt")),
    );
}
