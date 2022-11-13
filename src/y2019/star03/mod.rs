pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 768, include_str!("input.txt")),
        (part1_sanity1, 6, include_str!("input.sanity1.txt")),
        (part1_sanity2, 159, include_str!("input.sanity2.txt")),
        (part1_sanity3, 135, include_str!("input.sanity3.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 8684, include_str!("input.txt")),
        (part2_sanity1, 30, include_str!("input.sanity1.txt")),
        (part2_sanity2, 610, include_str!("input.sanity2.txt")),
        (part2_sanity3, 410, include_str!("input.sanity3.txt")),
    );
}
