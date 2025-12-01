mod common;

pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 4246, include_str!("input.txt")),
        (part1_example1, 8, include_str!("input.example1.txt")),
        (part1_example2, 86, include_str!("input.example2.txt")),
        (part1_example3, 132, include_str!("input.example3.txt")),
        (part1_example4, 136, include_str!("input.example4.txt")),
        (part1_example5, 81, include_str!("input.example5.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 1940, include_str!("input.txt")),
        (part2_example6, 8, include_str!("input.example6.txt")),
        (part2_example7, 24, include_str!("input.example7.txt")),
        (part2_example8, 32, include_str!("input.example8.txt")),
        (part2_example9, 72, include_str!("input.example9.txt")),
    );
}
