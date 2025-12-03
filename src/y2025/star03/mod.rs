pub mod part1;
// pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 17408, include_str!("input.txt")),
        (part1_example1, 357, include_str!("input.example1.txt")),
    );

    /*aoc_test_suite!(
        super::part2::run,
        (part2_main, 369, include_str!("input.txt")),
        (part2_example1, 3, include_str!("input.example1.txt")),
    );*/
}
