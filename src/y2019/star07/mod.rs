pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 298_586, include_str!("input.txt")),
        (part1_example1, 43210, include_str!("input.example1.txt")),
        (part1_example2, 54321, include_str!("input.example2.txt")),
        (part1_example3, 65210, include_str!("input.example3.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 9_246_095, include_str!("input.txt")),
        (
            part2_example4,
            139_629_729,
            include_str!("input.example4.txt")
        ),
        (part2_example5, 18216, include_str!("input.example5.txt")),
    );
}
