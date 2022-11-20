mod common;

pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 4246, include_str!("input.txt")),
        (part1_sanity1, 8, include_str!("input.sanity1.txt")),
        (part1_sanity2, 86, include_str!("input.sanity2.txt")),
        (part1_sanity3, 132, include_str!("input.sanity3.txt")),
        (part1_sanity4, 136, include_str!("input.sanity4.txt")),
        (part1_sanity5, 81, include_str!("input.sanity5.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 1940, include_str!("input.txt")),
        (part2_sanity6, 8, include_str!("input.sanity6.txt")),
        (part2_sanity7, 24, include_str!("input.sanity7.txt")),
        (part2_sanity8, 32, include_str!("input.sanity8.txt")),
        (part2_sanity9, 72, include_str!("input.sanity9.txt")),
    );
}
