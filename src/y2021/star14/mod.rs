pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 2915, include_str!("input.txt")),
        (part1_sanity, 1588, include_str!("input.sanity.txt")),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 3_353_146_900_153, include_str!("input.txt")),
        (
            part2_sanity,
            2_188_189_693_529,
            include_str!("input.sanity.txt")
        ),
    );
}
