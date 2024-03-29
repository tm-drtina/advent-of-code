pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(super::part1::run, (part1_main, 99_919_692_496_939, ""),);

    aoc_test_suite!(super::part2::run, (part2_main, 81_914_111_161_714, ""),);
}
