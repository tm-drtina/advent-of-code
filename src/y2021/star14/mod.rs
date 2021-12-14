pub mod part1;
pub mod part2;

#[macro_use]
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
        (part2_main, 3353146900153, include_str!("input.txt")),
        (part2_sanity, 2188189693529, include_str!("input.sanity.txt")),
    );
}
