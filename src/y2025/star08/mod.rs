pub mod part1;
//pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 90036, include_str!("input.txt"), 1000),
        (part1_example1, 40, include_str!("input.example1.txt"), 10),
    );

    /*aoc_test_suite!(
        super::part2::run,
        (part2_main, 0, include_str!("input.txt")),
        (part2_example1, 0, include_str!("input.example1.txt")),
    );*/
}
