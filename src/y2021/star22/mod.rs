pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 545118, include_str!("input.txt")),
        (part1_sanity1, 39, include_str!("input.sanity1.txt")),
        (part1_sanity2, 590784, include_str!("input.sanity2.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 1227298136842375, include_str!("input.txt")),
        (
            part2_sanity3,
            2758514936282235,
            include_str!("input.sanity3.txt")
        ),
    );
}
