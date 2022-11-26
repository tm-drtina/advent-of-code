pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 13045, include_str!("input.txt"), 1000),
        (part1_sanity1, 179, include_str!("input.sanity1.txt"), 10),
        (part1_sanity2, 1940, include_str!("input.sanity2.txt"), 100),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 344724687853944, include_str!("input.txt")),
        (part2_sanity1, 2772, include_str!("input.sanity1.txt")),
        (part2_sanity2, 4686774924, include_str!("input.sanity2.txt")),
    );
}
