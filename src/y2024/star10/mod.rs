pub mod part1;
//pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 694, include_str!("input.txt")),
        (part1_sanity1, 36, include_str!("input.sanity1.txt")),
    );

    /*aoc_test_suite!(
        super::part2::run,
        (part2_main, 369, include_str!("input.txt")),
        (part2_sanity1, 3, include_str!("input.sanity1.txt")),
        (part2_sanity2, 6, include_str!("input.sanity2.txt")),
    );*/
}
