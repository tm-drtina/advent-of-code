pub mod part1;
// pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 0, include_str!("input.txt")),
        (part1_sanity1, 4, include_str!("input.sanity1.txt")),
        (
            part1_sanity1_clean,
            4,
            include_str!("input.sanity1_clean.txt")
        ),
        (part1_sanity2, 8, include_str!("input.sanity2.txt")),
        (
            part1_sanity2_clean,
            8,
            include_str!("input.sanity2_clean.txt")
        ),
    );

    // aoc_test_suite!(
    //     super::part2::run,
    //     (part2_main, 369, include_str!("input.txt")),
    //     (part2_sanity1, 3, include_str!("input.sanity1.txt")),
    //     (part2_sanity2, 6, include_str!("input.sanity2.txt")),
    // );
}
