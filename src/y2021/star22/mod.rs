pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 545_118, include_str!("input.txt")),
        (part1_example1, 39, include_str!("input.example1.txt")),
        (part1_example2, 590_784, include_str!("input.example2.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 1_227_298_136_842_375, include_str!("input.txt")),
        (
            part2_example3,
            2_758_514_936_282_235,
            include_str!("input.example3.txt")
        ),
    );
}
