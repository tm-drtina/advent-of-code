pub mod part1;
//pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 1400386, include_str!("input.txt")),
        (part1_example1, 140, include_str!("input.example1.txt")),
        (part1_example2, 772, include_str!("input.example2.txt")),
        (part1_example3, 1930, include_str!("input.example3.txt")),
    );

    /*aoc_test_suite!(
        super::part2::run,
        (part2_main, 369, include_str!("input.txt")),
        (part2_example1, 3, include_str!("input.example1.txt")),
        (part2_example2, 6, include_str!("input.example2.txt")),
    );*/
}
