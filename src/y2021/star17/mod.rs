pub mod part1;
pub mod part2;

#[macro_use]
#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, 7503, "target area: x=124..174, y=-123..-86"),
        (part1_sanity, 45, "target area: x=20..30, y=-10..-5"),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 3229, "target area: x=124..174, y=-123..-86"),
        (part2_sanity, 112, "target area: x=20..30, y=-10..-5"),
    );
}
