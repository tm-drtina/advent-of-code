pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 292387, include_str!("input.txt")),
        (part1_sanity1, 42, "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L"),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 433, include_str!("input.txt")),
        (part2_sanity1, 4, "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN"),
    );
}
