pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 185797128, include_str!("input.txt")),
        (part1_example1, 161, include_str!("input.example1.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 89798695, include_str!("input.txt")),
        (part2_example2, 48, include_str!("input.example2.txt")),
    );
}
