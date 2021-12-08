pub mod part1;
pub mod part2;

#[macro_use]
#[cfg(test)]
mod tests {
    use crate::aoc_test_suite;

    aoc_test_suite!(
        super::part1::run,
        (part1_main, "43896725", "589174263"),
        (part1_sanity, "67384529", "389125467"),
    );

    aoc_test_suite!(
        super::part2::run,
        (part2_main, 2911418906, "589174263"),
        (part2_sanity, 149245887792, "389125467"),
    );
}
