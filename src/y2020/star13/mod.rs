pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 296, include_str!("input.txt")),
        (part1_example1, 295, include_str!("input.example1.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 535_296_695_251_210, include_str!("input.txt")),
        (
            part2_example1,
            1_068_781,
            include_str!("input.example1.txt")
        ),
        (part2_example2, 3417, include_str!("input.example2.txt")),
        (part2_example3, 754_018, include_str!("input.example3.txt")),
        (part2_example4, 779_210, include_str!("input.example4.txt")),
        (
            part2_example5,
            1_261_476,
            include_str!("input.example5.txt")
        ),
        (
            part2_example6,
            1_202_161_486,
            include_str!("input.example6.txt")
        ),
    );
}
