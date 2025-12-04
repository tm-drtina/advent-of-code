pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 5398419778, include_str!("input.txt")),
        (
            part1_example1,
            1227775554,
            include_str!("input.example1.txt")
        ),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 15704845910, include_str!("input.txt")),
        (
            part2_example1,
            4174379265,
            include_str!("input.example1.txt")
        ),
    );
}
