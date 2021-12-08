pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 296, include_str!("input.txt")),
        (part1_sanity1, 295, include_str!("input.sanity1.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 535296695251210, include_str!("input.txt")),
        (part2_sanity1, 1068781, include_str!("input.sanity1.txt")),
        (part2_sanity2, 3417, include_str!("input.sanity2.txt")),
        (part2_sanity3, 754018, include_str!("input.sanity3.txt")),
        (part2_sanity4, 779210, include_str!("input.sanity4.txt")),
        (part2_sanity5, 1261476, include_str!("input.sanity5.txt")),
        (part2_sanity6, 1202161486, include_str!("input.sanity6.txt")),
    );
}
