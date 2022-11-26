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
        (part2_main, 535_296_695_251_210, include_str!("input.txt")),
        (part2_sanity1, 1_068_781, include_str!("input.sanity1.txt")),
        (part2_sanity2, 3417, include_str!("input.sanity2.txt")),
        (part2_sanity3, 754_018, include_str!("input.sanity3.txt")),
        (part2_sanity4, 779_210, include_str!("input.sanity4.txt")),
        (part2_sanity5, 1_261_476, include_str!("input.sanity5.txt")),
        (
            part2_sanity6,
            1_202_161_486,
            include_str!("input.sanity6.txt")
        ),
    );
}
