pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 13045, include_str!("input.txt"), 1000),
        (part1_example1, 179, include_str!("input.example1.txt"), 10),
        (
            part1_example2,
            1940,
            include_str!("input.example2.txt"),
            100
        ),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 344_724_687_853_944, include_str!("input.txt")),
        (part2_example1, 2772, include_str!("input.example1.txt")),
        (
            part2_example2,
            4_686_774_924,
            include_str!("input.example2.txt")
        ),
    );
}
