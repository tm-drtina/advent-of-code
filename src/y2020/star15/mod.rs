pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 929, "16,1,0,18,12,14,19"),
        (part1_sanity1, 436, "0,3,6"),
        (part1_sanity2, 1, "1,3,2"),
        (part1_sanity3, 10, "2,1,3"),
        (part1_sanity4, 27, "1,2,3"),
        (part1_sanity5, 78, "2,3,1"),
        (part1_sanity6, 438, "3,2,1"),
        (part1_sanity7, 1836, "3,1,2"),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 16_671_510, "16,1,0,18,12,14,19"),
        (part2_sanity1, 175_594, "0,3,6"),
        (part2_sanity2, 2578, "1,3,2"),
        (part2_sanity3, 3_544_142, "2,1,3"),
        (part2_sanity4, 261_214, "1,2,3"),
        (part2_sanity5, 6_895_259, "2,3,1"),
        (part2_sanity6, 18, "3,2,1"),
        (part2_sanity7, 362, "3,1,2"),
    );
}
