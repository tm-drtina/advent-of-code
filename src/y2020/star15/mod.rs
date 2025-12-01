pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 929, "16,1,0,18,12,14,19"),
        (part1_example1, 436, "0,3,6"),
        (part1_example2, 1, "1,3,2"),
        (part1_example3, 10, "2,1,3"),
        (part1_example4, 27, "1,2,3"),
        (part1_example5, 78, "2,3,1"),
        (part1_example6, 438, "3,2,1"),
        (part1_example7, 1836, "3,1,2"),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 16_671_510, "16,1,0,18,12,14,19"),
        (part2_example1, 175_594, "0,3,6"),
        (part2_example2, 2578, "1,3,2"),
        (part2_example3, 3_544_142, "2,1,3"),
        (part2_example4, 261_214, "1,2,3"),
        (part2_example5, 6_895_259, "2,3,1"),
        (part2_example6, 18, "3,2,1"),
        (part2_example7, 362, "3,1,2"),
    );
}
