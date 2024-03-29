pub mod part1;
pub mod part2;

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
        (part2_main, 2_911_418_906, "589174263"),
        (part2_sanity, 149_245_887_792, "389125467"),
    );
}
